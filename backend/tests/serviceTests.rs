use sqlx::{PgPool, Executor};
use uuid::Uuid;
use services::user_service::UserService;

#[tokio::test]
async fn test_create_user() {
    // Setup: create a test database pool (use a test DB or transaction)
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let pool = PgPool::connect(&db_url).await.expect("Failed to connect to DB");
    let service = UserService::new(pool.clone());

    // Generate test data
    let user_id = Uuid::new_v4();
    let username = format!("testuser_{}", user_id);
    let email = format!("{}@example.com", username);
    let password_hash = "testhash";

    // Run the function
    let result = service.create_user(user_id, &username, &email, password_hash).await;
    assert!(result.is_ok());

    // Cleanup: remove the test user
    sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_register_user() {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    let pool = PgPool::connect(&db_url).await.expect("Failed to connect to DB");
    let service = UserService::new(pool.clone());

    let username = format!("reguser_{}", Uuid::new_v4());
    let email = format!("{}@example.com", username);
    let password = "testpassword";

    let result = service.register_user(&username, &email, password).await;
    assert!(result.is_ok());

    // Cleanup: remove the test user
    sqlx::query!("DELETE FROM users WHERE username = $1", username)
        .execute(&pool)
        .await
        .unwrap();
}