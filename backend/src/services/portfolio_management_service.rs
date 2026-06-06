use std::sync::Arc;

use crate::models::errors::trade_error::TradeError;
use crate::models::errors::user_error::UserError;
use crate::models::order::OrderType;
use crate::models::portfolio_ticker::HistoricalStockValue;
use crate::models::portfolio_ticker::PortfolioTicker;
use crate::models::stock_ticker::Ticker;
use crate::models::stock_ticker::TimeFrame;
use crate::services::ticker_service::TickerService;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct PortfolioManagementService {
    pub db: PgPool,
    pub ticker_service: Arc<TickerService>,
}
impl PortfolioManagementService {
    pub fn new(db: PgPool, ticker_service: Arc<TickerService>) -> Self {
        Self { db, ticker_service }
    }
    #[tracing::instrument(skip(self))]
    pub async fn get_total_portfolio_value(&self, user_id: Uuid) -> Result<BigDecimal, TradeError> {
        let portfolio = self.get_portfolio(user_id).await?;
        let mut total_portfolio_value = BigDecimal::from(0);
        for portfolio_item in portfolio {
            total_portfolio_value += portfolio_item.total_profit + portfolio_item.total_money_spent;
        }
        Ok(total_portfolio_value)
    }
    pub async fn get_stock_value_by_timeframe(
        &self,
        user_id: Uuid,
        timeframe: TimeFrame,
        ticker: &str,
    ) -> Result<Vec<HistoricalStockValue>, TradeError> {
        // Step A: Fetch historical ticker data
        let ticker_data = self
            .ticker_service
            .fetch_price_history_ticker_from_db(ticker, timeframe)
            .await?;

        // Step B: Fetch all filled orders for this user and ticker
        let orders_rows = sqlx::query(
            "SELECT quantity, order_type, created_at FROM orders 
             WHERE user_id = 1 AND ticker = 2 AND status = 'EXECUTED' 
             ORDER BY created_at ASC",
        )
        .bind(user_id)
        .bind(ticker)
        .fetch_all(&self.db)
        .await
        .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;

        // Step C: The Running Total Algorithm
        let mut historical_values = Vec::new();
        let mut current_quantity = BigDecimal::from(0);
        let mut order_idx = 0;

        for ticker_item in ticker_data {
            // Add or subtract quantities for any trades that happened on or before this day
            while order_idx < orders_rows.len() {
                let row = &orders_rows[order_idx];
                let order_date: DateTime<Utc> =
                    row.try_get("created_at").unwrap_or_else(|_| Utc::now());

                if order_date <= ticker_item.date {
                    let order_qty: BigDecimal = row.try_get("quantity").unwrap_or_default();
                    let order_type: OrderType = row.try_get("order_type").unwrap_or(OrderType::Buy);

                    match order_type {
                        OrderType::Buy => current_quantity += order_qty,
                        OrderType::Sell => current_quantity -= order_qty,
                    }
                    order_idx += 1;
                } else {
                    // This order happened after the current ticker date, wait for the next iteration
                    break;
                }
            }

            let total_value = &current_quantity * &ticker_item.close;

            historical_values.push(HistoricalStockValue {
                date: ticker_item.date,
                ticker: ticker_item.ticker.clone(),
                price_per_share: ticker_item.close.clone(),
                quantity: current_quantity.clone(),
                total_value,
            });
        }

        Ok(historical_values)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_portfolio_history(
        &self,
        user_id: Uuid,
        timeframe: TimeFrame,
    ) -> Result<Vec<crate::models::portfolio_ticker::PortfolioHistoryPoint>, TradeError> {
        let distinct_tickers = sqlx::query("SELECT DISTINCT ticker FROM orders WHERE user_id = 1 AND status = 'EXECUTED'")
            .bind(user_id)
            .fetch_all(&self.db)
            .await
            .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;

        let mut all_points: std::collections::BTreeMap<DateTime<Utc>, BigDecimal> = std::collections::BTreeMap::new();

        for rec in distinct_tickers {
            let ticker: String = rec.get("ticker");
            let ticker_history = self.get_stock_value_by_timeframe(user_id, timeframe, &ticker).await?;
            for point in ticker_history {
                let entry = all_points.entry(point.date).or_insert(BigDecimal::from(0));
                *entry += point.total_value;
            }
        }

        let mut history = Vec::new();
        for (date, total_value) in all_points {
            history.push(crate::models::portfolio_ticker::PortfolioHistoryPoint {
                date,
                total_value,
            });
        }

        Ok(history)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_portfolio(&self, user_id: Uuid) -> Result<Vec<PortfolioTicker>, TradeError> {
        let database_portfolio = sqlx::query("SELECT * FROM portfolio WHERE user_id = 1")
            .bind(user_id)
            .fetch_all(&self.db)
            .await
            .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        let mut user_portfolio = Vec::new();
        for rec in database_portfolio {
            let ticker = self
                .ticker_service
                .fetch_latest_price_ticker_from_db(rec.get("ticker"))
                .await?;
            let quantity = rec.get("quantity");
            let total_money_spent: BigDecimal = rec.get("total_money_spent");
            let calculated_total_profit = (&quantity * ticker.close) - &total_money_spent;
            let portfolio_item = PortfolioTicker {
                user_id: rec.get("user_id"),
                ticker: rec.get("ticker"),
                quantity: quantity,
                total_money_spent: total_money_spent,
                total_profit: calculated_total_profit,
                created_at: rec.get("created_at"),
            };
            user_portfolio.push(portfolio_item);
        }
        Ok(user_portfolio)
    }

    #[tracing::instrument(skip(self))]
    pub async fn check_holdings(
        &self,
        user_id: Uuid,
        ticker: &str,
    ) -> Result<BigDecimal, TradeError> {
        let rec = sqlx::query("SELECT * FROM portfolio WHERE user_id = 1 AND ticker = 2")
            .bind(user_id)
            .bind(ticker)
            .fetch_one(&self.db)
            .await
            .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        if rec.is_empty() {
            return Err(TradeError::UserError(UserError::InsufficientHoldings));
        }
        Ok(rec.get("quantity"))
    }

    #[tracing::instrument(skip(self))]
    pub async fn add_to_portfolio(
        &self,
        user_id: Uuid,
        ticker: &str,
        quantity: &BigDecimal,
        total_money_spent: &BigDecimal,
    ) -> Result<(), TradeError> {
        let portfolio_id = Uuid::new_v4();
        let _rec = sqlx::query(
            "INSERT INTO portfolio (portfolio_id, user_id, ticker, quantity, total_money_spent) VALUES (1, 2, 3, 4, 5)
            ON CONFLICT (user_id, ticker) DO UPDATE SET quantity = portfolio.quantity + 4, total_money_spent = portfolio.total_money_spent + 5",
        )
        .bind(portfolio_id)
        .bind(user_id)
        .bind(ticker)
        .bind(quantity)
        .bind(total_money_spent)
        .execute(&self.db)
        .await
        .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn remove_from_portfolio(
        &self,
        user_id: Uuid,
        ticker: &str,
        quantity: &BigDecimal,
    ) -> Result<(), TradeError> {
        let get_rec = sqlx::query("SELECT * FROM portfolio WHERE user_id = 1 AND ticker = 2")
            .bind(user_id)
            .bind(ticker)
            .fetch_one(&self.db)
            .await
            .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        if get_rec.is_empty() {
            return Err(TradeError::UserError(UserError::InsufficientHoldings));
        }
        let get_rec_quantity: BigDecimal = get_rec.get("quantity");
        if get_rec_quantity < *quantity {
            return Err(TradeError::UserError(UserError::InsufficientHoldings));
        }
        if get_rec_quantity == *quantity {
            sqlx::query("DELETE FROM portfolio WHERE user_id = 1 AND ticker = 2")
                .bind(user_id)
                .bind(ticker)
                .execute(&self.db)
                .await
                .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        } else {
            let _rec = sqlx::query(
                "UPDATE portfolio SET quantity = quantity - 3 WHERE user_id = 1 AND ticker = 2 AND quantity >= 3",
            )
            .bind(user_id)
            .bind(ticker)
            .bind(quantity)
            .execute(&self.db)
            .await
            .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        }
        Ok(())
    }
}
