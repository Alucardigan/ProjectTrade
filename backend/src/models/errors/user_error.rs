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
}

// trade_service.rs
