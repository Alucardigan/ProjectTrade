use bigdecimal::BigDecimal;
use serde::Serialize;

#[derive(Serialize)]
pub struct Ticker {
    pub symbol: String,
    pub price_per_share: BigDecimal,
    pub trend: Vec<BigDecimal>,
}
