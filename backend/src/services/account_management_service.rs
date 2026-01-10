use crate::models::errors::trade_error::TradeError;
use crate::models::errors::user_error::UserError;
use crate::models::transaction::Transaction;
use bigdecimal::BigDecimal;
use num_traits::Zero;
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
    #[tracing::instrument(skip(self))]
    pub async fn get_user_balance(&self, user_id: Uuid) -> Result<BigDecimal, UserError> {
        let rec = sqlx::query("SELECT available_balance FROM users WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db)
            .await?;
        Ok(rec.get("available_balance"))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_transaction_history(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Transaction>, UserError> {
        let records = sqlx::query("SELECT * FROM transactions WHERE user_id = $1")
            .bind(user_id)
            .fetch_all(&self.db)
            .await
            .map_err(|e| UserError::DatabaseError(e))?;
        let mut transactions = vec![];
        for rec in records {
            transactions.push(Transaction {
                transaction_id: rec.get("transaction_id"),
                user_id: rec.get("user_id"),
                ticker: rec.get("ticker"),
                quantity: rec.get("quantity"),
                price_per_share: rec.get("price_per_share"),
                order_type: rec.get("order_type"),
                executed_at: rec.get("executed_at"),
            });
        }
        Ok(transactions)
    }

    #[tracing::instrument(skip(self))]
    pub async fn reserve_funds(
        &self,
        user_id: Uuid,
        reserve_amount: BigDecimal,
    ) -> Result<(), TradeError> {
        if reserve_amount <= BigDecimal::zero() {
            return Err(TradeError::InvalidAmount);
        }
        tracing::debug!("Reserving funds {} for user {}", reserve_amount, user_id);
        //only available funds are deducted, balance is deducted from when the order is executed
        let rows_affected = sqlx::query(
        "UPDATE users SET available_balance = available_balance - $1 WHERE user_id = $2 AND available_balance >= $1",
        )
        .bind(reserve_amount)
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

    #[tracing::instrument(skip(self))]
    pub async fn add_user_balance(
        &self,
        user_id: Uuid,
        amount: &BigDecimal,
    ) -> Result<(), TradeError> {
        if amount <= &BigDecimal::zero() {
            return Err(TradeError::InvalidAmount);
        }
        let rows_affected = sqlx::query(
            "UPDATE users SET balance = balance + $1, available_balance = available_balance + $1 WHERE user_id = $2",
        )
        .bind(amount)
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

    #[tracing::instrument(skip(self))]
    pub async fn deduct_user_balance(
        &self,
        user_id: Uuid,
        amount: &BigDecimal,
    ) -> Result<(), TradeError> {
        if amount <= &BigDecimal::zero() {
            return Err(TradeError::InvalidAmount);
        }
        //only need to deduct balance as reserve funds already deducts from available balance
        let rows_affected = sqlx::query(
            "UPDATE users SET balance = balance - $1 WHERE user_id = $2 AND balance >= $1",
        )
        .bind(amount)
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
