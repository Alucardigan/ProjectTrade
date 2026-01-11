use backend::authentication::basic_client::AuthorizationClient;
use backend::models::order::{OrderStatus, OrderType};
use backend::services::account_management_service::AccountManagementService;
use backend::services::order_management_service::OrderManagementService;
use backend::services::portfolio_management_service::PortfolioManagementService;
use backend::services::ticker_service::TickerService;
use backend::services::user_service::UserService;
use bigdecimal::BigDecimal;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

async fn setup_db() -> PgPool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to DB")
}

#[tokio::test]
async fn test_order_placement() {
    let pool = setup_db().await;
    let account_service = Arc::new(AccountManagementService::new(pool.clone()));
    let ticker_service = Arc::new(TickerService::new("mock", pool.clone()));
    let portfolio_service = Arc::new(PortfolioManagementService::new(
        pool.clone(),
        ticker_service.clone(),
    ));
    let auth_client = Arc::new(AuthorizationClient::new());
    let user_service = Arc::new(UserService::new(
        pool.clone(),
        account_service.clone(),
        portfolio_service.clone(),
        auth_client,
    ));

    let oms = OrderManagementService::new(
        pool.clone(),
        user_service.clone(),
        ticker_service.clone(),
        account_service.clone(),
        portfolio_service.clone(),
    );

    // Create a test user
    let username = format!("trader_{}", Uuid::new_v4());
    let email = format!("{}@example.com", username);
    let auth0_id = format!("auth0|{}", Uuid::new_v4());
    let user_id = Uuid::new_v4();
    user_service
        .upsert_user(user_id, &auth0_id, &username, &email)
        .await
        .unwrap();
    let user_id = user_service.get_user_uuid(&username).await.unwrap();

    // Place an order
    let symbol = "AAPL";
    let quantity = BigDecimal::from_str("10").unwrap();
    let price_buffer = BigDecimal::from(0);

    let order = oms
        .place_order(
            user_id,
            symbol,
            quantity.clone(),
            OrderType::Buy,
            price_buffer,
        )
        .await;
    assert!(order.is_ok());
    let order = order.unwrap();

    assert_eq!(order.user_id, user_id);
    assert_eq!(order.ticker, symbol);
    assert_eq!(order.quantity, quantity);
    assert_eq!(order.status, OrderStatus::Pending);
}
