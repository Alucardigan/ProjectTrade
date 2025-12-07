use crate::models::errors::api_error::ApiError;
use thiserror::Error;
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum TickerError {
    #[error("Invalid symbol: {0}")]
    InvalidSymbol(String),
    #[error("API rate limit exceeded")]
    RateLimitExceeded,
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<TickerError> for ApiError {
    fn from(error: TickerError) -> Self {
        match error {
            TickerError::InvalidSymbol(s) => ApiError::BadRequest(format!("Invalid symbol: {}", s)),
            TickerError::RateLimitExceeded => {
                ApiError::InternalServerError("Rate limit exceeded".to_string())
            }
            TickerError::ApiError(s) => ApiError::InternalServerError(format!("API error: {}", s)),
        }
    }
}
