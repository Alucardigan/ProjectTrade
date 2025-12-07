use thiserror::Error;
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum UserError {
    #[error("User not found")]
    NotFound,
    #[error("Email already exists")]
    EmailAlreadyExists,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Insufficient holdings")]
    InsufficientHoldings,
}
use crate::models::errors::api_error::ApiError;

impl From<UserError> for ApiError {
    fn from(error: UserError) -> Self {
        match error {
            UserError::NotFound => ApiError::NotFound("User not found".to_string()),
            UserError::EmailAlreadyExists => {
                ApiError::BadRequest("Email already exists".to_string())
            }
            UserError::InvalidCredentials => {
                ApiError::Unauthorized("Invalid credentials".to_string())
            }
            UserError::DatabaseError(e) => ApiError::InternalServerError(e.to_string()),
            UserError::InsufficientFunds => ApiError::BadRequest("Insufficient funds".to_string()),
            UserError::InsufficientHoldings => {
                ApiError::BadRequest("Insufficient holdings".to_string())
            }
        }
    }
}

// trade_service.rs
