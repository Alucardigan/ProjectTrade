use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct PortfolioTicker {
    pub user_id: Uuid,
    pub ticker: String,
    pub quantity: BigDecimal,
    pub total_money_spent: BigDecimal,
    pub total_profit: BigDecimal,
    pub created_at: DateTime<Utc>,
}
