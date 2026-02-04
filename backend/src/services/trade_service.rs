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
use sqlx::Row;
use sqlx::{types::BigDecimal, PgPool};
use std::sync::Arc;
use tracing::{info, warn};
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
    const ORDER_PROCESSOR_INTERVAL_SECS: u64 = 100;
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

    #[tracing::instrument(skip(self))]
    pub async fn get_order(&self, order_id: Uuid) -> Result<Order, TradeError> {
        let rec = sqlx::query("SELECT * FROM orders WHERE order_id = $1")
            .bind(order_id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| TradeError::DatabaseError(e))?;
        Ok(Order {
            order_id: rec.try_get("order_id")?,
            user_id: rec.try_get("user_id")?,
            ticker: rec.try_get("ticker")?,
            quantity: rec.try_get("quantity")?,
            price_per_share: rec.try_get("price_per_share")?,
            order_type: rec.try_get("order_type")?,
            status: rec.try_get("status")?,
        })
    }
    #[tracing::instrument(skip(self))]
    pub async fn execute_order(
        &self,
        order_id: Uuid,
        fullfilment_quantity: BigDecimal,
    ) -> Result<(), TradeError> {
        let order = self.get_order(order_id).await?;
        //validation checks
        if order.status != OrderStatus::Pending {
            warn!("Order is not in pending state: {}", order.status);
            return Err(TradeError::InvalidOrderStatus);
        }
        if order.quantity < BigDecimal::from(0) && order.quantity < fullfilment_quantity {
            return Err(TradeError::InvalidAmount);
        }
        //TODO: refactor price getting here
        let total_purchase_price = &order.price_per_share * &fullfilment_quantity;
        match order.order_type {
            OrderType::Buy => {
                self.account_management_service
                    .deduct_user_balance(order.user_id, &total_purchase_price)
                    .await?;
                self.portfolio_management_service
                    .add_to_portfolio(
                        order.user_id,
                        &order.ticker,
                        &order.quantity,
                        &total_purchase_price,
                    )
                    .await?;
            }
            OrderType::Sell => {
                self.portfolio_management_service
                    .remove_from_portfolio(order.user_id, &order.ticker, &order.quantity)
                    .await?;
                self.account_management_service
                    .add_user_balance(order.user_id, &total_purchase_price)
                    .await?;
            }
        }
        self.log_transaction(&order).await?;
        if fullfilment_quantity < order.quantity {
            sqlx::query("UPDATE orders SET quantity = $2 WHERE order_id = $1")
                .bind(order_id)
                .bind(order.quantity - fullfilment_quantity)
                .execute(&self.db)
                .await
                .map_err(|e| TradeError::DatabaseError(e))?;
        } else {
            sqlx::query("UPDATE orders SET status = $2 WHERE order_id = $1")
                .bind(order_id)
                .bind(OrderStatus::Executed)
                .execute(&self.db)
                .await
                .map_err(|e| TradeError::DatabaseError(e))?;
        }
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_pending_orders(&self) -> Result<Vec<Order>, TradeError> {
        let rec = sqlx::query(
            "SELECT * FROM orders WHERE status =$1::order_status ORDER BY created_at ASC",
        )
        .bind(OrderStatus::Pending.to_string())
        .fetch_all(&self.db)
        .await
        .map_err(|e| TradeError::DatabaseError(e))?;
        let mut orders = Vec::new();
        for rec in rec {
            orders.push(Order {
                order_id: rec.try_get("order_id")?,
                user_id: rec.try_get("user_id")?,
                ticker: rec.try_get("ticker")?,
                quantity: rec.try_get("quantity")?,
                price_per_share: rec.try_get("price_per_share")?,
                order_type: rec.try_get("order_type")?,
                status: rec.try_get("status")?,
            });
        }
        Ok(orders)
    }

    //deprecated
    // #[tracing::instrument(skip(self))]
    // pub fn create_order_processor(
    //     self: Arc<Self>,
    // ) -> tokio::task::JoinHandle<Result<(), TradeError>> {
    //     tokio::spawn(async move {
    //         let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
    //             Self::ORDER_PROCESSOR_INTERVAL_SECS,
    //         ));
    //         loop {
    //             interval.tick().await;
    //             match self.get_pending_orders().await {
    //                 Ok(pending_orders) => {
    //                     info!(count = pending_orders.len(), "Processing pending orders");
    //                     for order in pending_orders {
    //                         match self.execute_order(order.order_id).await {
    //                             Ok(_) => {}
    //                             Err(e) => {
    //                                 warn!(error = ?e, "Failed to execute order");
    //                             }
    //                         }
    //                     }
    //                 }
    //                 Err(e) => {
    //                     warn!(error = ?e, "Failed to fetch pending orders");
    //                 }
    //             }
    //         }
    //     })
    // }

    #[tracing::instrument(skip(self))]
    async fn log_transaction(&self, order: &Order) -> Result<(), TradeError> {
        sqlx::query(
            "INSERT INTO transactions (transaction_id, user_id, ticker, order_type, quantity, price_per_share) 
            VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(uuid::Uuid::new_v4())
            .bind(&order.user_id)
            .bind(&order.ticker)
            .bind(&order.order_type)
            .bind(&order.quantity)
            .bind(&order.price_per_share)
            .execute(&self.db)
            .await
            .map_err(|e| TradeError::DatabaseError(e))?;
        Ok(())
    }
}
