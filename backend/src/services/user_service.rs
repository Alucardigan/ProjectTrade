use crate::models::errors::trade_error::TradeError;
use crate::models::errors::user_error::UserError;
use sqlx::postgres::types::PgMoney;
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use sqlx::Row;
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
    pub async fn get_user_balance(&self, user_id: Uuid) -> Result<(PgMoney, PgMoney), UserError> {
        let rec = sqlx::query("SELECT balance,available_balance FROM user_balance WHERE uid = $1")
            .bind(user_id)
            .fetch_one(&self.user_db)
            .await?;
        Ok((rec.get("balance"), rec.get("available_balance")))
    }

    pub async fn reserve_funds(
        &self,
        user_id: Uuid,
        reserve_amount: f64,
    ) -> Result<(), TradeError> {
        let reserve_cents = (reserve_amount * 100.0) as i64;
        if reserve_cents <= 0 {
            return Err(TradeError::InvalidAmount);
        }
        let rows_affected = sqlx::query(
        "UPDATE user_balance SET available_balance = available_balance - $1 WHERE uid = $2 AND available_balance >= $1",
        )
        .bind(PgMoney(reserve_cents))
        .bind(user_id)
        .execute(&self.user_db)
        .await
        .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?
        .rows_affected();
        if rows_affected > 0 {
            Ok(())
        } else {
            Err(TradeError::UserError(UserError::InsufficientFunds))
        }
    }
    pub async fn check_holdings(
        &self,
        user_id: Uuid,
        symbol: &str,
    ) -> Result<BigDecimal, TradeError> {
        let rec = sqlx::query("SELECT * FROM portfolio WHERE uid = $1 AND ticker = $2")
            .bind(user_id)
            .bind(symbol)
            .fetch_one(&self.user_db)
            .await
            .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        if rec.is_empty() {
            return Err(TradeError::UserError(UserError::InsufficientHoldings));
        }
        Ok(rec.get("quantity"))
    }
}
