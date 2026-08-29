use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ticker {
    pub ticker: String,
    pub date: DateTime<Utc>,
    pub close: BigDecimal,
    pub volume: Option<i64>,
    pub open: Option<BigDecimal>,
    pub high: Option<BigDecimal>,
    pub low: Option<BigDecimal>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerSummary {
    pub symbol: String,
    pub name: String,
    pub sector: String,
    pub description: String,
    pub current_price: BigDecimal,
    pub open_price: BigDecimal,
    pub day_high: BigDecimal,
    pub day_low: BigDecimal,
    pub day_volume: i64,
    pub change_percent: f64,
    pub is_positive: bool,
    pub market_cap: BigDecimal,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TimeFrame {
    Day,
    Month,
    HalfYear,
    Year,
    FiveYear,
    AllYears,
}
