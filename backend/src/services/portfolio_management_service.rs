use std::sync::Arc;

use crate::models::errors::trade_error::TradeError;
use crate::models::errors::user_error::UserError;
use crate::models::portfolio_ticker::PortfolioTicker;
use crate::services::ticker_service::TickerService;
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
    pub async fn get_portfolio(&self, user_id: Uuid) -> Result<Vec<PortfolioTicker>, TradeError> {
        let database_portfolio = sqlx::query("SELECT * FROM portfolio WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.db)
            .await
            .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        let mut user_portfolio = Vec::new();
        for rec in database_portfolio {
            let ticker = self.ticker_service.search_symbol(rec.get("ticker")).await;
            let quantity = rec.get("quantity");
            let total_money_spent: BigDecimal = rec.get("total_money_spent");
            let calculated_total_profit = (&quantity * ticker.price_per_share) - &total_money_spent;
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
    pub async fn check_holdings(
        &self,
        user_id: Uuid,
        symbol: &str,
    ) -> Result<BigDecimal, TradeError> {
        let rec = sqlx::query("SELECT * FROM portfolio WHERE user_id = $1 AND ticker = $2")
            .bind(user_id)
            .bind(symbol)
            .fetch_one(&self.db)
            .await
            .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        if rec.is_empty() {
            return Err(TradeError::UserError(UserError::InsufficientHoldings));
        }
        Ok(rec.get("quantity"))
    }

    pub async fn add_to_portfolio(
        &self,
        user_id: Uuid,
        symbol: &str,
        quantity: BigDecimal,
    ) -> Result<(), TradeError> {
        let _rec = sqlx::query(
            "INSERT INTO portfolio (user_id, ticker, quantity) VALUES ($1, $2, $3)
            ON CONFLICT (user_id, ticker) DO UPDATE SET quantity = portfolio.quantity + $3",
        )
        .bind(user_id)
        .bind(symbol)
        .bind(quantity)
        .execute(&self.db)
        .await
        .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        Ok(())
    }

    pub async fn remove_from_portfolio(
        &self,
        user_id: Uuid,
        symbol: &str,
        quantity: BigDecimal,
    ) -> Result<(), TradeError> {
        let _rec = sqlx::query(
            "UPDATE portfolio SET quantity = quantity - $3 WHERE uid = $1 AND ticker = $2 AND quantity >= $3",
        )
        .bind(user_id)
        .bind(symbol)
        .bind(quantity)
        .execute(&self.db)
        .await
        .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        Ok(())
    }
}
