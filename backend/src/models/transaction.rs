use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::order::OrderStatus;
use crate::models::order::OrderType;
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub user_id: Uuid,
    pub ticker: String,
    pub quantity: BigDecimal,
    pub price_per_share: BigDecimal,
    pub order_type: OrderType,
    pub executed_at: chrono::DateTime<chrono::Utc>,
}
