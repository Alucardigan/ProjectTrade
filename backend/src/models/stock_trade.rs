use crate::models::stock_ticker::Ticker;

pub struct Trade{
    ticker: Ticker,
    action: tradeAction,
    quantity: i64,
    price: f64,
    
}

enum tradeAction{
    Buy,
    Sell
}