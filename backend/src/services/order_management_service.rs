use std::str::FromStr;
use std::sync::Arc;

use crate::{
    models::{
        errors::{trade_error::TradeError, user_error::UserError},
        order::{Order, OrderStatus, OrderType},
    },
    services::{
        account_management_service::AccountManagementService,
        portfolio_management_service::PortfolioManagementService, ticker_service::TickerService,
        user_service::UserService,
    },
};
use num_traits::ToPrimitive;
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Clone)]
pub struct OrderManagementService {
    pub db: PgPool,
    pub user_service: Arc<UserService>,
    pub trade_service: Arc<TickerService>,
    pub account_management_service: Arc<AccountManagementService>,
    pub portfolio_management_service: Arc<PortfolioManagementService>,
}

#[allow(dead_code)]
impl OrderManagementService {
    pub fn new(
        db: PgPool,
        user_service: Arc<UserService>,
        trade_service: Arc<TickerService>,
        account_management_service: Arc<AccountManagementService>,
        portfolio_management_service: Arc<PortfolioManagementService>,
    ) -> Self {
        Self {
            db,
            user_service,
            trade_service,
            account_management_service,
            portfolio_management_service,
        }
    }

    /// Place a new order, reserve funds, and add to queue
    pub async fn place_order(
        &self,
        user_id: Uuid,
        symbol: &str,
        quantity: BigDecimal,
        order_type: OrderType,
        _price_buffer: f64,
    ) -> Result<Order, TradeError> {
        // TODO : Add atomicity to this function
        let order_id = Uuid::new_v4();
        let status = OrderStatus::Pending;
        if quantity < BigDecimal::from(0) {
            return Err(TradeError::InvalidAmount);
        }
        //price calculation
        let price = self.trade_service.search_symbol(symbol).await.price;
        let total_purchase_price = price * quantity.to_f64().ok_or(TradeError::InvalidAmount)?;
        match order_type {
            OrderType::Buy => {
                // Reserve funds
                self.account_management_service
                    .reserve_funds(user_id, total_purchase_price)
                    .await?;
            }
            OrderType::Sell => {
                // Check holdings
                let holdings = self
                    .portfolio_management_service
                    .check_holdings(user_id, symbol)
                    .await?;
                if holdings < quantity {
                    return Err(TradeError::UserError(UserError::InsufficientHoldings));
                }
            }
        }
        //Placing order
        sqlx::query(
            "INSERT INTO orders 
        (order_id, user_id, symbol, quantity, price, order_type, status) 
        VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(order_id)
        .bind(user_id)
        .bind(symbol)
        .bind(quantity.clone())
        .bind(price)
        .bind(order_type.to_string())
        .bind(status.to_string())
        .execute(&self.db)
        .await
        .map_err(|e| TradeError::DatabaseError(e))?;
        Ok(Order {
            order_id,
            user_id,
            symbol: symbol.to_string(),
            quantity,
            price,
            order_type,
            status,
        })
    }

    pub async fn get_order_status(&self, order_id: Uuid) -> Result<OrderStatus, TradeError> {
        let rec = sqlx::query("SELECT * FROM orders WHERE order_id = $1")
            .bind(order_id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| TradeError::DatabaseError(e))?;
        let order_status_str: &str = rec
            .try_get("status")
            .map_err(|_| TradeError::InvalidOrderStatus)?;
        Ok(OrderStatus::from_str(order_status_str).map_err(|_e| TradeError::InvalidOrderStatus)?)
    }

    pub async fn cancel_order(&self, order_id: Uuid, user_id: Uuid) -> Result<(), TradeError> {
        let rec = sqlx::query("SELECT * FROM orders WHERE order_id = $1")
            .bind(order_id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| TradeError::DatabaseError(e))?;
        let order_status_str: &str = rec
            .try_get("status")
            .map_err(|_| TradeError::InvalidOrderStatus)?;
        if order_status_str == "Cancelled" {
            return Ok(());
        }
        let user_id_str: &str = rec
            .try_get("user_id")
            .map_err(|_| TradeError::UserError(UserError::InvalidCredentials))?;
        if Uuid::parse_str(user_id_str).unwrap() != user_id {
            return Err(TradeError::UserError(UserError::InvalidCredentials));
        }
        sqlx::query("UPDATE orders SET status = 'Cancelled' WHERE order_id = $1")
            .bind(order_id)
            .execute(&self.db)
            .await
            .map_err(|e| TradeError::DatabaseError(e))?;
        Ok(())
    }

    pub async fn get_orders(&self, user_id: Uuid) -> Result<Vec<Order>, TradeError> {
        let rec = sqlx::query("SELECT * FROM orders WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.db)
            .await
            .map_err(|e| TradeError::DatabaseError(e))?;
        let orders: Vec<Order> = rec
            .into_iter()
            .map(|r| {
                Ok(Order {
                    order_id: r.try_get("order_id")?,
                    user_id: r.try_get("user_id")?,
                    symbol: r.try_get("symbol")?,
                    quantity: r.try_get("quantity")?,
                    price: r.try_get("price")?,
                    order_type: OrderType::from_str(r.try_get("order_type")?)
                        .map_err(|_| TradeError::InvalidOrderType)?,
                    status: OrderStatus::from_str(r.try_get("status")?)
                        .map_err(|_| TradeError::InvalidOrderStatus)?,
                })
            })
            .collect::<Result<Vec<Order>, TradeError>>()?;
        Ok(orders)
    }

    pub async fn get_order(&self, order_id: Uuid, user_id: Uuid) -> Result<Order, TradeError> {
        let rec = sqlx::query("SELECT * FROM orders WHERE order_id = $1 AND user_id = $2")
            .bind(order_id)
            .bind(user_id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| TradeError::DatabaseError(e))?;
        let order = Order {
            order_id: rec.try_get("order_id")?,
            user_id: rec.try_get("user_id")?,
            symbol: rec.try_get("symbol")?,
            quantity: rec.try_get("quantity")?,
            price: rec.try_get("price")?,
            order_type: OrderType::from_str(rec.try_get("order_type")?)
                .map_err(|_| TradeError::InvalidOrderType)?,
            status: OrderStatus::from_str(rec.try_get("status")?)
                .map_err(|_| TradeError::InvalidOrderStatus)?,
        };
        Ok(order)
    }
}
