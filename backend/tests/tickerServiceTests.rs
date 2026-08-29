use backend::services::synthetic_data::{get_company_by_symbol, get_synthetic_universe};
use backend::services::ticker_service::TickerService;
use sqlx::PgPool;

#[tokio::test]
async fn test_synthetic_universe() {
    let universe = get_synthetic_universe();
    assert_eq!(universe.len(), 8);

    let cybr = get_company_by_symbol("CYBR").expect("CYBR company should exist");
    assert_eq!(cybr.name, "Cyberdyne Systems");
    assert_eq!(cybr.sector, "Robotics & AI");
    assert!(cybr.base_price > 0.0);
}

#[tokio::test]
async fn test_synthetic_ticker_service() {
    dotenv::dotenv().ok();
    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        if let Ok(pool) = PgPool::connect(&db_url).await {
            let service = TickerService::new(pool);
            let tickers = service.get_all_tickers().await;
            assert!(tickers.is_ok());
        }
    }
}
