use crate::{
    models::{
        errors::trade_error::TradeError,
        order::{Order, OrderStatus, OrderType},
    },
    services::{ticker_service::TickerService, user_service::UserService},
};
use sqlx::PgPool;
use uuid::Uuid;
#[allow(dead_code)]
pub struct OrderManagementService {
    pub db: PgPool,
    pub user_service: UserService,
    pub trade_service: TickerService, // You could add a queue here, or use a DB table as the queue
}

#[allow(dead_code)]
impl OrderManagementService {
    pub fn new(db: PgPool, user_service: UserService, trade_service: TickerService) -> Self {
        Self {
            db,
            user_service,
            trade_service,
        }
    }

    /// Place a new order, reserve funds, and add to queue
    pub async fn place_order(
        &self,
        user_id: Uuid,
        symbol: &str,
        quantity: f64,
        order_type: OrderType,
        _price_buffer: f64,
    ) -> Result<Order, TradeError> {
        // need to add: symbol validation, price buffer validation, market validation
        let order_id = Uuid::new_v4();
        let status = OrderStatus::Reserved;
        if quantity < 0.0 {
            return Err(TradeError::InvalidAmount);
        }
        //price calculation
        let price = self.trade_service.search_symbol(symbol).await.price;
        let total_purchase_price = price * quantity;
        match order_type {
            OrderType::Buy => {
                // Reserve funds
                self.user_service
                    .reserve_funds(user_id, total_purchase_price)
                    .await?;
            }
            OrderType::Sell => {
                // Check holdings - TODO: Implement holdings check
            }
        }

        // TODO: Implement fund reservation and actual DB logic
        // sqlx::query!(...)

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

    // Add more methods as needed: cancel_order, get_order_status, etc.
}
