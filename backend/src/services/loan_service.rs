use std::sync::Arc;

use crate::models::errors::trade_error::TradeError;
use crate::models::loan::{Loan, LoanStatus};
use crate::models::{errors::user_error::UserError, loan::LoanType};
use crate::services::account_management_service::AccountManagementService;
use crate::services::portfolio_management_service::PortfolioManagementService;
use alpha_vantage::exchange;
use bigdecimal::BigDecimal;
use chrono::Utc;
use num_traits::Zero;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;
pub struct LoanService {
    db: PgPool,
}

impl LoanService {
    pub fn new(db: PgPool) -> LoanService {
        LoanService { db }
    }
    //should convert this into a function that also checks if user has a loan
    pub async fn get_loan(&self, user_id: Uuid) -> Result<Loan, UserError> {
        let loan = sqlx::query("SELECT * FROM loans WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db)
            .await;
        match loan {
            Ok(loan) => Ok(Loan::new(
                loan.try_get("loan_id")?,
                loan.try_get("user_id")?,
                loan.try_get("principal")?,
                loan.try_get("interest_rate")?,
                loan.try_get("status")?,
                loan.try_get("created_at")?,
                loan.try_get("last_paid_at")?,
            )),
            Err(sqlx::Error::RowNotFound) => Err(UserError::UserDoesNotHaveLoan),
            Err(e) => Err(UserError::DatabaseError(e)),
        }
    }
    pub async fn request_loan(&self, user_id: Uuid, loan_type: LoanType) -> Result<(), UserError> {
        //check if user already has a loan
        let existing_loan = self.get_loan(user_id).await;
        if existing_loan.is_ok() {
            return Err(UserError::UserAlreadyHasLoan);
        }
        let (principal, interest_rate) = loan_type.get_rates();
        let loan = Loan::new(
            Uuid::new_v4(),
            user_id,
            principal,
            interest_rate,
            LoanStatus::ONGOING,
            Utc::now(),
            Utc::now(),
        );
        sqlx::query("INSERT INTO loans (loan_id, user_id, principal, interest_rate, status, created_at, last_paid_at) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(loan.loan_id)
            .bind(loan.user_id)
            .bind(loan.principal)
            .bind(loan.interest_rate)
            .bind(loan.status)
            .bind(loan.created_at)
            .bind(loan.last_paid_at)
            .execute(&self.db)
            .await
            .map_err(|e| UserError::DatabaseError(e))?;

        Ok(())
    }
    //users are assumed to only have one loan
    pub async fn set_loan_status(
        &self,
        user_id: Uuid,
        loan_status: LoanStatus,
    ) -> Result<(), TradeError> {
        sqlx::query("UPDATE loans SET status = $2 WHERE user_id = $1")
            .bind(user_id)
            .bind(loan_status)
            .execute(&self.db)
            .await
            .map_err(|e| TradeError::DatabaseError(e))?;
        Ok(())
    }
}
