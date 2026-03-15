use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Ticker {
    pub symbol: String,
    pub date: DateTime<Utc>,
    pub close: BigDecimal,
    pub volume: Option<i64>,
    pub open: Option<BigDecimal>,
    pub high: Option<BigDecimal>,
    pub low: Option<BigDecimal>,
}
