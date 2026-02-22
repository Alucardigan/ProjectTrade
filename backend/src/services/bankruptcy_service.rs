use std::cmp::{max, min};
use std::sync::Arc;

use bigdecimal::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::errors::trade_error::TradeError;
use crate::models::errors::user_error::UserError;
use crate::models::loan::LoanStatus;
use crate::services::account_management_service::AccountManagementService;
use crate::services::loan_service::LoanService;
use crate::services::order_management_service::OrderManagementService;
use crate::services::portfolio_management_service::PortfolioManagementService;

pub struct BankruptcyService {
    db: PgPool,
    loan_service: Arc<LoanService>,
    account_management_service: Arc<AccountManagementService>,
    portfolio_management_service: Arc<PortfolioManagementService>,
    order_management_service: Arc<OrderManagementService>,
}

impl BankruptcyService {
    const CREDIT_SCORE_THRESHOLD: i32 = 25;
    pub fn new(
        db: PgPool,
        loan_service: Arc<LoanService>,
        account_management_service: Arc<AccountManagementService>,
        portfolio_management_service: Arc<PortfolioManagementService>,
        order_management_service: Arc<OrderManagementService>,
    ) -> BankruptcyService {
        BankruptcyService {
            db,
            loan_service,
            account_management_service,
            portfolio_management_service,
            order_management_service,
        }
    }
    pub async fn check_for_bankruptcy(&self, user_id: Uuid) -> Result<bool, TradeError> {
        let loan = self.loan_service.get_loan(user_id).await;
        match loan {
            Ok(loan) => {
                let current_cash_balance = self
                    .account_management_service
                    .get_user_balance(user_id)
                    .await?;
                let portfolio_value = self
                    .portfolio_management_service
                    .get_total_portfolio_value(user_id)
                    .await?;
                let liabilities = loan.get_current_balance();
                let credit_score = Self::calculate_credit_score(
                    current_cash_balance + portfolio_value,
                    liabilities.0 + liabilities.1,
                );
                if credit_score < BigDecimal::from(Self::CREDIT_SCORE_THRESHOLD) {
                    return Ok(true);
                }
            }
            Err(UserError::UserDoesNotHaveLoan) => return Ok(false),
            Err(e) => return Err(TradeError::from(e)),
        }
        Ok(false)
    }

    pub async fn handle_bankruptcy(&self, user_id: Uuid) -> Result<(), TradeError> {
        let loan = self.loan_service.get_loan(user_id).await;
        match loan {
            Ok(_loan) => {
                if self.check_for_bankruptcy(user_id).await? {
                    //set the loan to default
                    self.loan_service
                        .set_loan_status(user_id, LoanStatus::DEFAULTED)
                        .await?;
                    //reset user
                    self.account_management_service
                        .reset_user_balance(user_id)
                        .await?;
                    //liquidate portfolio
                    let user_portfolio = self
                        .portfolio_management_service
                        .get_portfolio(user_id)
                        .await?;
                    for portfolio_item in user_portfolio {
                        self.portfolio_management_service
                            .remove_from_portfolio(
                                portfolio_item.user_id,
                                &portfolio_item.ticker,
                                &portfolio_item.quantity,
                            )
                            .await?;
                    }
                    //cancel all orders from the user
                    self.order_management_service
                        .cancel_all_orders(user_id)
                        .await?;
                }
            }
            Err(UserError::UserDoesNotHaveLoan) => return Ok(()),
            Err(e) => return Err(TradeError::from(e)),
        }
        Ok(())
    }

    //note: liability cannot be 0
    pub fn calculate_credit_score(
        asset_value: BigDecimal,
        liability_value: BigDecimal,
    ) -> BigDecimal {
        if liability_value == BigDecimal::from(0) {
            return BigDecimal::from(100);
        }
        min(
            (&asset_value - &liability_value) / &liability_value,
            BigDecimal::from(100),
        )
    }
}
