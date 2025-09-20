mod models;
mod routes;
mod services;

use axum::{
    routing::get,Router
};
use routes::ticker_handlers::{tickers_handler};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = Router::new().route("/tickers", get(tickers_handler));
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL NOT FOUND");
    let db = PgPool::connect(&db_url).await;
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🦀 Server running on http://{}", listener.local_addr().unwrap());

    // Axum 0.7 helper: no hyper boilerplate needed
    axum::serve(listener, app).await.unwrap();
    
}