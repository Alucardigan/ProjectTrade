use thiserror::Error;

use crate::models::errors::ticker_error::TickerError;
use crate::models::errors::user_error::UserError;
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum TradeError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid trade amount")]
    InvalidAmount,
    #[error("Invalid order status")]
    InvalidOrderStatus,
    #[error("Invalid order type")]
    InvalidOrderType,
    #[error("Market closed")]
    MarketClosed,
    #[error("User error: {0}")]
    UserError(#[from] UserError), // Can wrap UserError
    #[error("Ticker error: {0}")]
    TickerError(#[from] TickerError),
}
