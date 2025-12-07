use crate::app_state::AppState;
use crate::models::stock_ticker::Ticker;
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn get_ticker(
    State(app_state): State<AppState>,
    Path(symbol): Path<String>,
) -> Result<Json<Vec<Ticker>>, (axum::http::StatusCode, String)> {
    Ok(Json(vec![
        app_state.ticker_service.search_symbol(&symbol).await,
    ]))
}
