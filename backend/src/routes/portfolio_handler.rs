use crate::{
    app_state::AppState,
    models::{
        errors::api_error::ApiError,
        portfolio_ticker::{PortfolioHistoryPoint, PortfolioTicker},
        stock_ticker::TimeFrame,
    },
};
use axum::{
    extract::{Query, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Debug)]
pub struct PortfolioHistoryQuery {
    timeframe: TimeFrame,
}

#[tracing::instrument(skip(app_state))]
pub async fn get_portfolio_history(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Query(query): Query<PortfolioHistoryQuery>,
) -> Result<Json<Vec<PortfolioHistoryPoint>>, ApiError> {
    let history = app_state
        .portfolio_service
        .get_portfolio_history(user_id, query.timeframe)
        .await?;
    Ok(Json(history))
}
