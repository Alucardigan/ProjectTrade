use crate::models::errors::user_error::UserError;
use sqlx::postgres::types::PgMoney;
use sqlx::PgPool;
use uuid::Uuid;
//current implementation assumes no Errors
#[allow(dead_code)]
pub struct UserService {
    pub user_db: PgPool,
}
#[allow(dead_code)]
impl UserService {
    //need to implement getters
    pub fn new(db: PgPool) -> Self {
        Self { user_db: (db) }
    }
    pub async fn create_user(
        &self,
        user_id: Uuid,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
            user_id,
            username,
            email,
            password_hash
        )
        .execute(&self.user_db)
        .await?;

        print!("Created user");
        Ok(())
    }

    pub async fn create_user_balance_account(&self, user_id: Uuid) -> Result<(), UserError> {
        let starting_amount: i64 = 100000;
        sqlx::query(
            "INSERT INTO user_balance(uid, balance, available_balance) VALUES ($1, $2, $3)",
        )
        .bind(user_id)
        .bind(PgMoney(starting_amount))
        .bind(PgMoney(starting_amount))
        .execute(&self.user_db)
        .await?;
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
        self.create_user_balance_account(user_id).await?;
        Ok(())
    }
    pub async fn get_user_uuid(&self, username: &str) -> Result<Uuid, UserError> {
        let rec = sqlx::query!("SELECT id FROM users WHERE username = $1", username)
            .fetch_one(&self.user_db)
            .await?;
        Ok(rec.id)
    }
}
