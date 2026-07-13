use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct Ticker {
    pub ticker: String,
    pub date: DateTime<Utc>,
    pub close: BigDecimal,
    pub volume: Option<i64>,
    pub open: Option<BigDecimal>,
    pub high: Option<BigDecimal>,
    pub low: Option<BigDecimal>,
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
