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
