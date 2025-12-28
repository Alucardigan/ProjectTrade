use crate::{app_state::AppState, models::errors::api_error::ApiError};
use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use axum_extra::extract::CookieJar;
use chrono::Utc;
use uuid::Uuid;

pub async fn auth0_middleware(
    State(app_state): State<AppState>,
    cookies: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, ApiError> {
    //get the session
    let session_id = cookies
        .get("session_id")
        .ok_or(ApiError::Unauthorized("Session ID not found".to_string()))?
        .value();
    let session_uuid = Uuid::parse_str(session_id)
        .map_err(|_| ApiError::Unauthorized("Invalid session ID".to_string()))?;
    //get the user_id
    let record = sqlx::query!(
        "SELECT user_id FROM sessions WHERE session_id = $1",
        session_uuid,
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(|_| ApiError::Unauthorized("Database error: can't fetch user id".to_string()))?;
    //update session db
    sqlx::query!(
        "UPDATE sessions SET updated_at = $1 WHERE session_id = $2",
        Utc::now().naive_utc(),
        session_uuid,
    )
    .execute(&app_state.db)
    .await
    .map_err(|_| {
        ApiError::InternalServerError("Database error: can't update session".to_string())
    })?;
    //attach the user_id to the request
    req.extensions_mut().insert(record.user_id);
    let response = next.run(req).await;
    Ok(response)
}
