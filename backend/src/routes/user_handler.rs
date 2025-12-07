use crate::app_state::AppState;
use crate::models::errors::api_error::ApiError;
use axum::{extract::State, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}
pub async fn register_user(
    State(app_state): State<AppState>,
    Json(request_body): Json<RegisterUserRequest>,
) -> Result<(), ApiError> {
    app_state
        .user_service
        .register_user(
            &request_body.username,
            &request_body.email,
            &request_body.password,
        )
        .await?;
    Ok(())
}
