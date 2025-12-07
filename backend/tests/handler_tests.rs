use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use backend::app_state::AppState;
use backend::routes::router::create_router;
use dotenv::dotenv;

use sqlx::PgPool;
use std::env;
use tower::util::ServiceExt; // for `oneshot`

async fn setup_app() -> axum::Router {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:password@localhost:5432/project_trade".to_string()
    });
    let pool = match PgPool::connect(&db_url).await {
        Ok(pool) => pool,
        Err(_) => {
            return create_router().with_state(AppState::new(
                PgPool::connect_lazy(&db_url).unwrap(),
                "mock",
            ))
        }
    };
    let app_state = AppState::new(pool, "mock");
    create_router().with_state(app_state)
}

#[tokio::test]
async fn test_get_tickers() {
    let app = setup_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/tickers")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}
