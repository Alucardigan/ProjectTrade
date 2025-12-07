use backend::models;
use backend::routes;
use backend::services;

use anyhow::Result;
use axum::{routing::get, Router};
use backend::app_state::AppState;
use dotenv::dotenv;
use routes::ticker_handler::get_ticker;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    //database setup
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL NOT FOUND");
    let _db = PgPool::connect(&db_url).await?;

    let app_state = AppState::new(_db, "mock");
    let app = Router::new()
        .route("/tickers", get(get_ticker))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!(
        "🦀 Server running on http://{}",
        listener.local_addr().unwrap()
    );

    // Axum 0.7 helper: no hyper boilerplate needed
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
