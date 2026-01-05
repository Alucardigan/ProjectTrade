use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use strum::Display;
use strum::EnumString;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub quantity: BigDecimal,
    pub price_per_share: BigDecimal,
    pub order_type: OrderType, // Buy or Sell
    pub status: OrderStatus,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Display, EnumString, Serialize, Deserialize, sqlx::Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "order_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Display, EnumString, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "order_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Pending,
    Reserved,
    Executed,
    Cancelled,
}
