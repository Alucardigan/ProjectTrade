use std::cmp::min;
use std::sync::Arc;

use crate::models::errors::trade_error::TradeError;
use crate::models::loan::{Loan, LoanStatus};
use crate::models::{errors::user_error::UserError, loan::LoanType};
use crate::services::account_management_service::AccountManagementService;
use bigdecimal::BigDecimal;
use chrono::Utc;
use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;
pub struct LoanService {
    db: PgPool,
    account_management_service: Arc<AccountManagementService>,
}

impl LoanService {
    pub fn new(
        db: PgPool,
        account_management_service: Arc<AccountManagementService>,
    ) -> LoanService {
        LoanService {
            db,
            account_management_service,
        }
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
    pub async fn request_loan(&self, user_id: Uuid, loan_type: LoanType) -> Result<(), TradeError> {
        //check if user already has a loan
        let existing_loan = self.get_loan(user_id).await;
        if existing_loan.is_ok() {
            return Err(TradeError::UserError(UserError::UserAlreadyHasLoan));
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
            .bind(&loan.loan_id)
            .bind(&loan.user_id)
            .bind(&loan.principal)
            .bind(&loan.interest_rate)
            .bind(&loan.status)
            .bind(&loan.created_at)
            .bind(&loan.last_paid_at)
            .execute(&self.db)
            .await
            .map_err(|e| UserError::DatabaseError(e))?;
        self.account_management_service
            .add_user_balance(user_id, &loan.principal)
            .await?;
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

    pub async fn repay_loan(
        &self,
        user_id: Uuid,
        payment_amount: BigDecimal,
    ) -> Result<(), TradeError> {
        let loan = self.get_loan(user_id).await?;
        if loan.status != LoanStatus::ONGOING {
            return Err(TradeError::UserError(UserError::UserDoesNotHaveLoan));
        }
        let user_balance = self
            .account_management_service
            .get_user_balance(user_id)
            .await?;
        if user_balance < payment_amount {
            return Err(TradeError::UserError(UserError::InsufficientFunds));
        }

        let (accrued_interest, principal) = loan.get_current_balance();
        self.account_management_service
            .deduct_user_balance(
                user_id,
                &min(payment_amount.clone(), &accrued_interest + &principal),
            )
            .await?;

        //pay the accrued interest first
        let remaining_interest = &accrued_interest - min(&payment_amount, &accrued_interest);
        let remaning_payment_amount = &payment_amount - min(&payment_amount, &accrued_interest);
        let mut remaining_principal = &principal - &remaning_payment_amount;
        remaining_principal += &remaining_interest;
        if remaining_principal <= BigDecimal::from(0) {
            self.set_loan_status(user_id, LoanStatus::PAID).await?;
        } else {
            sqlx::query("UPDATE loans SET principal = $2, last_paid_at = $3 WHERE loan_id = $1")
                .bind(loan.loan_id)
                .bind(remaining_principal)
                .bind(Utc::now())
                .execute(&self.db)
                .await
                .map_err(|e| TradeError::DatabaseError(e))?;
        }
        Ok(())
    }
}
