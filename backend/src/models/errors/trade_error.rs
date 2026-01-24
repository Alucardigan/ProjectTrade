use thiserror::Error;

use crate::models::errors::api_error::ApiError;
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
    #[error("Order book not found")]
    OrderBookNotFound,
}

impl From<TradeError> for ApiError {
    fn from(error: TradeError) -> Self {
        match error {
            TradeError::DatabaseError(e) => ApiError::InternalServerError(e.to_string()),
            TradeError::InvalidAmount => ApiError::BadRequest("Invalid trade amount".to_string()),
            TradeError::InvalidOrderStatus => {
                ApiError::BadRequest("Invalid order status".to_string())
            }
            TradeError::InvalidOrderType => ApiError::BadRequest("Invalid order type".to_string()),
            TradeError::MarketClosed => ApiError::BadRequest("Market is closed".to_string()),
            TradeError::UserError(e) => e.into(),
            TradeError::TickerError(e) => e.into(),
            TradeError::OrderBookNotFound => ApiError::NotFound("Order book not found".to_string()),
        }
    }
}
