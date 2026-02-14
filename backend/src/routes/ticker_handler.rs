use crate::models::stock_ticker::Ticker;
use crate::{app_state::AppState, models::errors::api_error::ApiError};
use axum::{
    extract::{Path, State},
    Json,
};

#[tracing::instrument(skip(app_state))]
pub async fn get_ticker(
    State(app_state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<Vec<Ticker>>, ApiError> {
    Ok(Json(vec![
        app_state.ticker_service.search_symbol(&symbol).await?,
    ]))
}
