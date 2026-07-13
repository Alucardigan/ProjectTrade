use crate::models::stock_ticker::{Ticker, TimeFrame};
use crate::{app_state::AppState, models::errors::api_error::ApiError};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

#[tracing::instrument(skip(app_state))]
pub async fn get_ticker(
    State(app_state): State<AppState>,
    Path(ticker): Path<String>,
) -> Result<Json<Vec<Ticker>>, ApiError> {
    Ok(Json(vec![
        app_state
            .ticker_service
            .fetch_latest_price_ticker_from_db(&ticker)
            .await?,
    ]))
}

#[derive(Deserialize, Debug)]
pub struct TickerHistoryQuery {
    timeframe: TimeFrame,
}

#[tracing::instrument(skip(app_state))]
pub async fn get_ticker_history(
    State(app_state): State<AppState>,
    Path(ticker): Path<String>,
    Query(query): Query<TickerHistoryQuery>,
) -> Result<Json<Vec<Ticker>>, ApiError> {
    Ok(Json(
        app_state
            .ticker_service
            .fetch_price_history_ticker_from_db(&ticker, query.timeframe)
            .await?,
    ))
}
