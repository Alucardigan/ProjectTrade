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
    account_management_service: Arc<AccountManagementService>,
    portfolio_service: Arc<PortfolioManagementService>,
}

impl LoanService {
    pub fn new(
        db: PgPool,
        account_management_service: Arc<AccountManagementService>,
        portfolio_service: Arc<PortfolioManagementService>,
    ) -> LoanService {
        LoanService {
            db,
            account_management_service,
            portfolio_service,
        }
    }
    //should convert this into a function that also checks if user has a loan
    pub async fn get_loan(&self, user_id: Uuid) -> Result<Loan, UserError> {
        let loan = sqlx::query("SELECT * FROM loans WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| UserError::DatabaseError(e))?;
        Ok(Loan::new(
            loan.try_get("loan_id")?,
            loan.try_get("user_id")?,
            loan.try_get("principal")?,
            loan.try_get("interest_rate")?,
            loan.try_get("status")?,
            loan.try_get("created_at")?,
            loan.try_get("last_paid_at")?,
        ))
    }
    pub async fn request_loan(&self, user_id: Uuid, loan_type: LoanType) -> Result<(), UserError> {
        //check if user already has a loan
        let existing_loan = sqlx::query("SELECT * FROM loans WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| UserError::DatabaseError(e))?;
        if existing_loan.is_some() {
            return Err(UserError::UserAlreadyHasLoan);
        }
        //match to loan type - needs a refactor
        match loan_type {
            LoanType::Standard => {
                let loan = Loan::new(
                    Uuid::new_v4(),
                    user_id,
                    BigDecimal::from(100000),
                    BigDecimal::from(5),
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
            }
            LoanType::Premium => {
                let loan = Loan::new(
                    Uuid::new_v4(),
                    user_id,
                    BigDecimal::from(10000000),
                    BigDecimal::from(10),
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
            }
        }
        Ok(())
    }
    pub async fn get_user_credit_score(&self, user_id: Uuid) -> Result<BigDecimal, TradeError> {
        //check if user already has a loan
        let existing_loan = sqlx::query("SELECT * FROM loans WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db)
            .await;

        match existing_loan {
            Ok(existing_loan) => {
                let existing_liability = Loan::new(
                    existing_loan.try_get("loan_id")?,
                    existing_loan.try_get("user_id")?,
                    existing_loan.try_get("principal")?,
                    existing_loan.try_get("interest_rate")?,
                    existing_loan.try_get("status")?,
                    existing_loan.try_get("created_at")?,
                    existing_loan.try_get("last_paid_at")?,
                );
                let current_cash_balance = self
                    .account_management_service
                    .get_user_balance(user_id)
                    .await?;
                let portfolio_value = self
                    .portfolio_service
                    .get_total_portfolio_value(user_id)
                    .await?;
                let credit_score = (&current_cash_balance + &portfolio_value
                    - existing_liability.get_current_balance())
                    / (&current_cash_balance + &portfolio_value);
                Ok(credit_score * BigDecimal::from(100))
            }
            //if the user doesnt have a loan auto 100 credit score
            Err(sqlx::Error::RowNotFound) => Ok(BigDecimal::from(100)),
            Err(e) => Err(TradeError::DatabaseError(e)),
        }
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
    pub async fn check_for_bankruptcy(&self, user_id: Uuid) -> Result<(), TradeError> {
        let credit_score = self.get_user_credit_score(user_id).await.unwrap();
        if credit_score < BigDecimal::zero() {
            let user_portfolio = self.portfolio_service.get_portfolio(user_id).await?;
            for portfolio_item in user_portfolio {
                self.portfolio_service
                    .remove_from_portfolio(
                        portfolio_item.user_id,
                        &portfolio_item.ticker,
                        &portfolio_item.quantity,
                    )
                    .await?;
            }
            self.account_management_service
                .reset_user_balance(user_id)
                .await?;
            self.set_loan_status(user_id, LoanStatus::DEFAULTED).await?;
        }
        Ok(())
    }
}
