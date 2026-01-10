use axum::{extract::State, Extension, Json};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    models::{errors::api_error::ApiError, transaction::Transaction},
};

#[derive(Serialize)]
pub struct GetAccountBalanceResponse {
    user_id: Uuid,
    available_balance: BigDecimal,
}
#[derive(Serialize, Deserialize)]
pub struct ChangeToUserBalanceRequest {
    amount: BigDecimal,
}
pub async fn get_account_balance(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<GetAccountBalanceResponse>, ApiError> {
    let available_balance = app_state
        .account_management_service
        .get_user_balance(user_id)
        .await?;
    let response = GetAccountBalanceResponse {
        user_id,
        available_balance,
    };
    Ok(Json(response))
}

pub async fn get_transaction_history(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Vec<Transaction>>, ApiError> {
    let transactions = app_state
        .account_management_service
        .get_transaction_history(user_id)
        .await?;
    Ok(Json(transactions))
}

pub async fn add_to_user_balance(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(request_body): Json<ChangeToUserBalanceRequest>,
) -> Result<(), ApiError> {
    app_state
        .account_management_service
        .add_user_balance(user_id, &request_body.amount)
        .await?;
    Ok(())
}

pub async fn withdraw_funds(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(request_body): Json<ChangeToUserBalanceRequest>,
) -> Result<(), ApiError> {
    app_state
        .account_management_service
        .deduct_user_balance(user_id, &request_body.amount)
        .await?;
    Ok(())
}
