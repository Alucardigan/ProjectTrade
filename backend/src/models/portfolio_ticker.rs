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

#[derive(Serialize)]
pub struct HistoricalStockValue {
    pub date: DateTime<Utc>,
    pub ticker: String,
    pub price_per_share: BigDecimal,
    pub quantity: BigDecimal,
    pub total_value: BigDecimal,
}

#[derive(Serialize)]
pub struct PortfolioHistoryPoint {
    pub date: DateTime<Utc>,
    pub total_value: BigDecimal,
}
