use backend::services::ticker_service::TickerService;
use sqlx::PgPool; // Adjust the path as needed

#[tokio::test]
async fn test_search_symbol_mock() {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to DB");
    let service = TickerService::new("mock", pool);

    let ticker = service.search_symbol("AAPL").await;
    print!("{}", ticker.symbol);
    assert_eq!(ticker.symbol, "AAPL");
    assert_eq!(ticker.trend.len(), 5);
}
