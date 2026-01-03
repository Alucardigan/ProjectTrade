use std::env;

use crate::app_state::AppState;
use crate::models::errors::api_error::ApiError;
use axum::extract::Query;
use axum::extract::State;
use axum::http::HeaderValue;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::CookieJar;
use cookie::Cookie;
use oauth2::http::header::SET_COOKIE;
use oauth2::{CsrfToken, PkceCodeVerifier};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct AuthCallbackQuery {
    pub code: String,
    pub state: String,
}

pub async fn login_user(State(app_state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let auth0_login_response = app_state.user_service.login_user().await?;
    tracing::trace!(
        "Login user response: csrf token {} pkce code verifier {}",
        auth0_login_response.csrf_token.secret(),
        auth0_login_response.pkce_code_verifier.secret()
    );
    let csrf_token_cookie = set_cookie("csrf_token", auth0_login_response.csrf_token.secret());
    let pkce_code_verifier_cookie = set_cookie(
        "pkce_code_verifier",
        auth0_login_response.pkce_code_verifier.secret(),
    );
    let auth_url = auth0_login_response.auth_url.to_string();
    let mut redirect_response = Redirect::to(&auth_url).into_response();
    tracing::trace!("Redirecting to auth_url: {}", auth_url);
    let csrf_token_header = HeaderValue::from_str(&csrf_token_cookie.to_string()).map_err(|e| {
        ApiError::InternalServerError(format!("Failed to create csrf token header: {}", e))
    })?;
    let pkce_code_verifier_header = HeaderValue::from_str(&pkce_code_verifier_cookie.to_string())
        .map_err(|e| {
        ApiError::InternalServerError(format!("Failed to create pkce code verifier header: {}", e))
    })?;
    redirect_response
        .headers_mut()
        .append(SET_COOKIE, csrf_token_header);
    redirect_response
        .headers_mut()
        .append(SET_COOKIE, pkce_code_verifier_header);
    Ok(redirect_response)
}

pub async fn auth0_callback(
    State(app_state): State<AppState>,
    cookies: CookieJar,
    Query(query): Query<AuthCallbackQuery>,
) -> Result<impl IntoResponse, ApiError> {
    //extraction
    let csrf_token = cookies
        .get("csrf_token")
        .ok_or(ApiError::MissingCookie("csrf_token".to_string()))?
        .value()
        .to_string();
    let pkce_code_verifier = cookies
        .get("pkce_code_verifier")
        .ok_or(ApiError::MissingCookie("pkce_code_verifier".to_string()))?
        .value()
        .to_string();
    //make the exchange
    let session_id = app_state
        .user_service
        .login_callback(
            query.code,
            query.state,
            PkceCodeVerifier::new(pkce_code_verifier),
            CsrfToken::new(csrf_token),
        )
        .await?;
    //remove cookies
    let session_id_cookie = set_cookie("session_id", session_id.to_string().as_str());
    let cookies = cookies
        .remove("csrf_token")
        .remove("pkce_code_verifier")
        .add(session_id_cookie);
    Ok((
        cookies,
        Redirect::to(&(env::var("FRONTEND_URL").unwrap() + "/portfolio")),
    ))
}

fn set_cookie(key: &str, value: &str) -> Cookie<'static> {
    Cookie::build((key.to_string(), value.to_string()))
        .secure(false)
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .path("/")
        .build()
}
