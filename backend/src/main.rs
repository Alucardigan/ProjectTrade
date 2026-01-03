use anyhow::Result;
use backend::app_state::AppState;
use backend::routes::router::create_router;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    //tracing setup
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    //database setup
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL NOT FOUND");
    let _db = PgPool::connect(&db_url).await?;

    let app_state = AppState::new(_db, "mock");
    let _task_handles = app_state.start_background_processes();
    let app = create_router(app_state.clone()).with_state(app_state);
    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;
    info!(
        "🦀 Server running on http://{}",
        listener.local_addr().unwrap()
    );

    // Axum 0.7 helper: no hyper boilerplate needed
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
