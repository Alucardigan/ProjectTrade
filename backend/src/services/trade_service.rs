use crate::{
    models::{
        errors::trade_error::TradeError,
        order::{Order, OrderStatus, OrderType},
    },
    services::{
        account_management_service::AccountManagementService,
        portfolio_management_service::PortfolioManagementService, ticker_service::TickerService,
    },
};
use num_traits::ToPrimitive;
use sqlx::Row;
use sqlx::{types::BigDecimal, PgPool};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct TradeService {
    pub db: PgPool,
    pub ticker_service: Arc<TickerService>,
    pub account_management_service: Arc<AccountManagementService>,
    pub portfolio_management_service: Arc<PortfolioManagementService>,
}
#[allow(dead_code)]
impl TradeService {
    pub fn new(
        db: PgPool,
        ticker_service: Arc<TickerService>,
        account_management_service: Arc<AccountManagementService>,
        portfolio_management_service: Arc<PortfolioManagementService>,
    ) -> Self {
        Self {
            db,
            ticker_service,
            account_management_service,
            portfolio_management_service,
        }
    }

    pub async fn get_order(&self, order_id: Uuid) -> Result<Order, TradeError> {
        let rec = sqlx::query("SELECT * FROM orders WHERE order_id = $1")
            .bind(order_id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| TradeError::DatabaseError(e))?;
        let order_status_str: &str = rec
            .try_get("status")
            .map_err(|_| TradeError::InvalidOrderStatus)?;
        Ok(Order {
            order_id: rec.try_get("order_id")?,
            user_id: rec.try_get("user_id")?,
            symbol: rec.try_get("symbol")?,
            quantity: rec.try_get("quantity")?,
            price: rec.try_get("price")?,
            order_type: OrderType::from_str(rec.try_get("order_type")?)
                .map_err(|_| TradeError::InvalidOrderType)?,
            status: OrderStatus::from_str(order_status_str)
                .map_err(|_| TradeError::InvalidOrderStatus)?,
        })
    }

    pub async fn execute_order(&self, order_id: Uuid) -> Result<(), TradeError> {
        let order = self.get_order(order_id).await?;
        //validation checks
        if order.status != OrderStatus::Pending {
            return Err(TradeError::InvalidOrderStatus);
        }
        if order.quantity < BigDecimal::from(0) {
            return Err(TradeError::InvalidAmount);
        }
        let price = self.ticker_service.search_symbol(&order.symbol).await.price;
        let total_purchase_price =
            price * order.quantity.to_f64().ok_or(TradeError::InvalidAmount)?;
        match order.order_type {
            OrderType::Buy => {
                self.account_management_service
                    .deduct_user_balance(order.user_id, total_purchase_price)
                    .await?;
                self.portfolio_management_service
                    .add_to_portfolio(order.user_id, &order.symbol, order.quantity)
                    .await?;
            }
            OrderType::Sell => {
                self.portfolio_management_service
                    .remove_from_portfolio(order.user_id, &order.symbol, order.quantity)
                    .await?;
                self.account_management_service
                    .add_user_balance(order.user_id, total_purchase_price)
                    .await?;
            }
        }
        Ok(())
    }
}
