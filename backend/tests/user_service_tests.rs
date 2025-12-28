use backend::authentication::basic_client::AuthorizationClient;
use backend::services::account_management_service::AccountManagementService;
use backend::services::portfolio_management_service::PortfolioManagementService;
use backend::services::user_service::UserService;
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
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
async fn test_user_service_registration() {
    let pool = setup_db().await;
    let account_service = Arc::new(AccountManagementService::new(pool.clone()));
    let portfolio_service = Arc::new(PortfolioManagementService::new(pool.clone()));
    let auth_client = Arc::new(AuthorizationClient::new());
    let service = UserService::new(
        pool.clone(),
        account_service,
        portfolio_service,
        auth_client,
    );

    let username = format!("testuser_{}", Uuid::new_v4());
    let email = format!("{}@example.com", username);
    let password = "password123";

    let result = service.register_user(&username, &email, password).await;
    assert!(result.is_ok());

    // Verify user exists and has balance account
    let user_id = service.get_user_uuid(&username).await.unwrap();
    let balance = sqlx::query("SELECT balance FROM user_balance WHERE uid = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await;

    assert!(balance.is_ok());
}
