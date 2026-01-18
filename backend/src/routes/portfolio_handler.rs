use crate::{
    app_state::AppState,
    models::{errors::api_error::ApiError, portfolio_ticker::PortfolioTicker},
};
use axum::{extract::State, Extension, Json};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct PortfolioResponse {
    user_id: Uuid,
    portfolio: Vec<PortfolioTicker>,
}

#[tracing::instrument(skip(app_state))]
pub async fn get_portfolio(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<PortfolioResponse>, ApiError> {
    let portfolio = app_state.portfolio_service.get_portfolio(user_id).await?;
    Ok(Json(PortfolioResponse { user_id, portfolio }))
}
