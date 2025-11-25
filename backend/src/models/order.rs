use sqlx::types::BigDecimal;
use strum::Display;
use strum::EnumString;
use uuid::Uuid;

#[allow(dead_code)]
pub struct Order {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub quantity: BigDecimal,
    pub price: f64,
    pub order_type: OrderType, // Buy or Sell
    pub status: OrderStatus,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Display, EnumString)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Display, EnumString, PartialEq)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Pending,
    Reserved,
    Executed,
    Cancelled,
}
