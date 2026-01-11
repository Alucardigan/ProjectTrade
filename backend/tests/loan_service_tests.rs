use backend::models::loan::LoanStatus;
use backend::services::account_management_service::AccountManagementService;
use backend::services::loan_service::LoanService;
use backend::services::portfolio_management_service::PortfolioManagementService;
use backend::services::ticker_service::TickerService;
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
async fn test_loan_request() {
    let pool = setup_db().await;
    let account_service = Arc::new(AccountManagementService::new(pool.clone()));
    let ticker_service = Arc::new(TickerService::new("mock", pool.clone()));
    let portfolio_service = Arc::new(PortfolioManagementService::new(
        pool.clone(),
        ticker_service,
    ));

    let loan_service = LoanService::new(
        pool.clone(),
        account_service.clone(),
        portfolio_service.clone(),
    );

    let user_id = Uuid::new_v4();
    // Create user first (omitted for brevity, assume user exists or mock it)
    // For integration test, we'd need to insert a user.
    // Let's just insert a dummy user directly into DB for this test
    sqlx::query(
        "INSERT INTO users (user_id, auth_user_id, username, email) VALUES ($1, $2, $3, $4)",
    )
    .bind(user_id)
    .bind(format!("auth0|{}", Uuid::new_v4()))
    .bind(format!("user_{}", Uuid::new_v4()))
    .bind(format!("user_{}@example.com", Uuid::new_v4()))
    .execute(&pool)
    .await
    .unwrap();

    // Initialize balance
    sqlx::query("INSERT INTO user_balance (user_id, balance, available_balance) VALUES ($1, 0, 0)")
        .bind(user_id)
        .execute(&pool)
        .await
        .unwrap();

    let principal = BigDecimal::from_str("1000.00").unwrap();
    let interest_rate = BigDecimal::from_str("0.05").unwrap();

    let loan = loan_service
        .request_loan(user_id, principal.clone(), interest_rate)
        .await;

    assert!(loan.is_ok());
    let loan = loan.unwrap();
    assert_eq!(loan.principal, principal);
    assert!(matches!(loan.status, LoanStatus::ONGOING));

    // Verify balance increased
    let balance = account_service.get_user_balance(user_id).await.unwrap();
    assert_eq!(balance.available_balance, principal);
}
