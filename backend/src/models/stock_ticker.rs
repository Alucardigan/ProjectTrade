use serde::Serialize;

#[derive(Serialize)]
pub struct Ticker {
    pub symbol: String,
    pub price: f64,
    pub trend: Vec<f64>,
}
