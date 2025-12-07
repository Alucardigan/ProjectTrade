use std::sync::Arc;

use crate::models::errors::user_error::UserError;
use crate::services::account_management_service::AccountManagementService;
use crate::services::portfolio_management_service::PortfolioManagementService;

use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;
//current implementation assumes no Errors
#[allow(dead_code)]
#[derive(Clone)]
pub struct UserService {
    user_db: PgPool,
    account_management_service: Arc<AccountManagementService>,
    portfolio_management_service: Arc<PortfolioManagementService>,
}
#[allow(dead_code)]
impl UserService {
    //need to implement getters
    pub fn new(
        db: PgPool,
        account_management_service: Arc<AccountManagementService>,
        portfolio_management_service: Arc<PortfolioManagementService>,
    ) -> Self {
        Self {
            user_db: db,
            account_management_service,
            portfolio_management_service,
        }
    }
    pub async fn create_user(
        &self,
        user_id: Uuid,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
        )
        .bind(user_id)
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .execute(&self.user_db)
        .await?;

        print!("Created user");
        Ok(())
    }

    pub async fn register_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<(), UserError> {
        print!("registers user");
        let user_id: Uuid = Uuid::new_v4();
        let password_hash = password;
        self.create_user(user_id, username, email, password_hash)
            .await?;
        self.account_management_service
            .create_user_balance_account(user_id)
            .await?;
        Ok(())
    }

    pub async fn get_user_uuid(&self, username: &str) -> Result<Uuid, UserError> {
        let rec = sqlx::query("SELECT id FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(&self.user_db)
            .await
            .map_err(|e| UserError::DatabaseError(e))?;
        Ok(rec.try_get("id").unwrap())
    }
}
