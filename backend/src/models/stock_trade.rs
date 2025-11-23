use crate::models::stock_ticker::Ticker;

#[allow(dead_code)]
pub struct Trade {
    ticker: Ticker,
    action: TradeAction,
    quantity: i64,
    price: f64,
}

#[allow(dead_code)]
enum TradeAction {
    Buy,
    Sell,
}
