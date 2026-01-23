use backend::models::order::{Order, OrderStatus, OrderType};
use backend::services::order_matchbook_service::OrderMatchbookService;
use bigdecimal::BigDecimal;
use num_traits::FromPrimitive;
use sqlx::PgPool;
use uuid::Uuid;

fn create_dummy_order(ticker: &str, price: f64, order_type: OrderType) -> Order {
    Order {
        order_id: Uuid::new_v4(),
        user_id: Uuid::new_v4(),
        ticker: ticker.to_string(),
        quantity: BigDecimal::from(10),
        price_per_share: BigDecimal::from_f64(price).unwrap(),
        order_type,
        status: OrderStatus::Pending,
    }
}

#[tokio::test]
async fn test_add_order_and_get_best_sale() {
    // We use a lazy connection or just a pool that won't be used since the service doesn't use DB yet
    // But since the service struct has a db field, we need to provide one.
    // Ideally we would mock it, but for now we can just try to connect to a dummy or use a real test DB if available.
    // Since the user environment has sqlx, let's try to just create a pool.
    // Note: In unit tests without a running DB, connecting might fail.
    // However, OrderMatchbookService::new takes a PgPool.
    // We can try to use `sqlx::PgPool::connect_lazy` which doesn't check connection immediately.

    let db = PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
    let service = OrderMatchbookService::new(db);

    let ticker = "AAPL";

    // Add Buy Orders: $100, $105, $102
    let buy1 = create_dummy_order(ticker, 100.0, OrderType::Buy);
    let buy2 = create_dummy_order(ticker, 105.0, OrderType::Buy); // Best Buy
    let buy3 = create_dummy_order(ticker, 102.0, OrderType::Buy);

    service.add_order(buy1).await.unwrap();
    service.add_order(buy2.clone()).await.unwrap();
    service.add_order(buy3).await.unwrap();

    // Add Sell Orders: $110, $108, $115
    let sell1 = create_dummy_order(ticker, 110.0, OrderType::Sell);
    let sell2 = create_dummy_order(ticker, 108.0, OrderType::Sell); // Best Sell
    let sell3 = create_dummy_order(ticker, 115.0, OrderType::Sell);

    service.add_order(sell1).await.unwrap();
    service.add_order(sell2.clone()).await.unwrap();
    service.add_order(sell3).await.unwrap();

    let (best_buy, best_sell) = service.get_best_sale(ticker).await.unwrap();

    assert!(best_buy.is_some());
    assert!(best_sell.is_some());

    assert_eq!(best_buy.unwrap().price_per_share, buy2.price_per_share);
    assert_eq!(best_sell.unwrap().price_per_share, sell2.price_per_share);
}

#[tokio::test]
async fn test_empty_book() {
    let db = PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
    let service = OrderMatchbookService::new(db);
    // Expect error because book doesn't exist
    let result = service.get_best_sale("UNKNOWN").await;
    assert!(result.is_err());
}
