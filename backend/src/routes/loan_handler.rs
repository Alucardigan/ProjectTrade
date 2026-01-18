use axum::{
    extract::{Path, State},
    Extension, Json,
};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    models::{
        errors::api_error::ApiError,
        loan::{Loan, LoanType},
    },
};

pub async fn request_loan(
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

pub async fn get_loan(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Loan>, ApiError> {
    let loan = app_state.loan_service.get_loan(user_id).await?;
    Ok(Json(loan))
}
