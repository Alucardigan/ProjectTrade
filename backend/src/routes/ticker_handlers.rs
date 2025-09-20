use axum :: Json;
use crate::models::stock_ticker::{Ticker};
use crate::services::ticker_service::{demo_tickers};
pub async fn tickers_handler() -> Json<Vec<Ticker>> {
    Json(demo_tickers().await)
}