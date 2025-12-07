use crate::models::errors::trade_error::TradeError;
use crate::models::errors::user_error::UserError;
use sqlx::postgres::types::PgMoney;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct AccountManagementService {
    pub db: PgPool,
}
impl AccountManagementService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
    pub async fn get_user_balance(&self, user_id: Uuid) -> Result<(PgMoney, PgMoney), UserError> {
        let rec = sqlx::query("SELECT balance,available_balance FROM user_balance WHERE uid = $1")
            .bind(user_id)
            .fetch_one(&self.db)
            .await?;
        Ok((rec.get("balance"), rec.get("available_balance")))
    }

    pub async fn create_user_balance_account(&self, user_id: Uuid) -> Result<(), UserError> {
        let starting_amount: i64 = 100000;
        sqlx::query(
            "INSERT INTO user_balance(uid, balance, available_balance) VALUES ($1, $2, $3)",
        )
        .bind(user_id)
        .bind(PgMoney(starting_amount))
        .bind(PgMoney(starting_amount))
        .execute(&self.db)
        .await?;
        Ok(())
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
        .execute(&self.db)
        .await
        .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?
        .rows_affected();
        if rows_affected > 0 {
            Ok(())
        } else {
            Err(TradeError::UserError(UserError::InsufficientFunds))
        }
    }

    pub async fn add_user_balance(&self, user_id: Uuid, amount: f64) -> Result<(), TradeError> {
        let add_cents = (amount * 100.0) as i64;
        if add_cents <= 0 {
            return Err(TradeError::InvalidAmount);
        }
        let rows_affected = sqlx::query(
            "UPDATE user_balance SET balance = balance + $1, available_balance = available_balance + $1 WHERE uid = $2",
        )
        .bind(PgMoney(add_cents))
        .bind(user_id)
        .execute(&self.db)
        .await
        .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?
        .rows_affected();
        if rows_affected > 0 {
            Ok(())
        } else {
            Err(TradeError::UserError(UserError::InsufficientFunds))
        }
    }

    pub async fn deduct_user_balance(&self, user_id: Uuid, amount: f64) -> Result<(), TradeError> {
        let deduct_cents = (amount * 100.0) as i64;
        if deduct_cents <= 0 {
            return Err(TradeError::InvalidAmount);
        }
        let rows_affected = sqlx::query(
            "UPDATE user_balance SET balance = balance - $1 WHERE uid = $2 AND balance >= $1",
        )
        .bind(PgMoney(deduct_cents))
        .bind(user_id)
        .execute(&self.db)
        .await
        .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        if rows_affected.rows_affected() > 0 {
            Ok(())
        } else {
            Err(TradeError::UserError(UserError::InsufficientFunds))
        }
    }
}
