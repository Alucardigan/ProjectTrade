use axum::{
    extract::{Path, State},
    Extension,
};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    models::{errors::api_error::ApiError, loan::LoanType},
};

pub async fn get_loan(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(loan_type): Path<LoanType>,
) -> Result<(), ApiError> {
    app_state
        .loan_service
        .request_loan(user_id, loan_type)
        .await?;
    Ok(())
}
