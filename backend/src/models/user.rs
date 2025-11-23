use crate::models::{stock_ticker::Ticker, stock_trade::Trade};

#[allow(dead_code)]
pub struct User {
    user_id: String,
    name: String,
    email: String,
    trade_history: Vec<Trade>,
    portfolio: Vec<Ticker>,
    balance: f64,
}
