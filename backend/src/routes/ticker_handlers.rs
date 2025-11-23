use std::env;

use crate::models::stock_ticker::Ticker;
use crate::services::ticker_service::TickerService;
use axum::Json;
use sqlx::PgPool;

pub async fn tickers_handler() -> Result<Json<Vec<Ticker>>, (axum::http::StatusCode, String)> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL NOT FOUND");
    let db = PgPool::connect(&db_url).await.map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database connection error: {}", e),
        )
    })?;
    let ticker = TickerService::new("mock", db);
    Ok(Json(vec![ticker.search_symbol("GOOG").await]))
}
