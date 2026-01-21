use axum::{
    extract::{Path, State},
    Extension, Json,
};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    models::{
        errors::api_error::ApiError,
        loan::{Loan, LoanType},
    },
};
#[derive(Serialize, Deserialize)]
pub struct RepayLoanRequest {
    amount: BigDecimal,
}
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
#[tracing::instrument(skip(app_state, user_id))]
pub async fn get_loan(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Loan>, ApiError> {
    let loan = app_state.loan_service.get_loan(user_id).await?;
    Ok(Json(loan))
}

pub async fn repay_loan(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(request_body): Json<RepayLoanRequest>,
) -> Result<(), ApiError> {
    app_state
        .loan_service
        .repay_loan(user_id, request_body.amount)
        .await?;
    Ok(())
}
