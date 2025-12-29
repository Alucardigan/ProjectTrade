use crate::models::errors::trade_error::TradeError;
use crate::models::errors::user_error::UserError;
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;

#[derive(Clone)]
pub struct PortfolioManagementService {
    pub db: PgPool,
}
impl PortfolioManagementService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
    pub async fn check_holdings(
        &self,
        user_id: Uuid,
        symbol: &str,
    ) -> Result<BigDecimal, TradeError> {
        let rec = sqlx::query("SELECT * FROM portfolio WHERE user_id = $1 AND ticker = $2")
            .bind(user_id)
            .bind(symbol)
            .fetch_one(&self.db)
            .await
            .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        if rec.is_empty() {
            return Err(TradeError::UserError(UserError::InsufficientHoldings));
        }
        Ok(rec.get("quantity"))
    }

    pub async fn add_to_portfolio(
        &self,
        user_id: Uuid,
        symbol: &str,
        quantity: BigDecimal,
    ) -> Result<(), TradeError> {
        let _rec = sqlx::query(
            "INSERT INTO portfolio (user_id, ticker, quantity) VALUES ($1, $2, $3)
            ON CONFLICT (user_id, ticker) DO UPDATE SET quantity = portfolio.quantity + $3",
        )
        .bind(user_id)
        .bind(symbol)
        .bind(quantity)
        .execute(&self.db)
        .await
        .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        Ok(())
    }

    pub async fn remove_from_portfolio(
        &self,
        user_id: Uuid,
        symbol: &str,
        quantity: BigDecimal,
    ) -> Result<(), TradeError> {
        let _rec = sqlx::query(
            "UPDATE portfolio SET quantity = quantity - $3 WHERE uid = $1 AND ticker = $2 AND quantity >= $3",
        )
        .bind(user_id)
        .bind(symbol)
        .bind(quantity)
        .execute(&self.db)
        .await
        .map_err(|e| TradeError::UserError(UserError::DatabaseError(e)))?;
        Ok(())
    }
}
