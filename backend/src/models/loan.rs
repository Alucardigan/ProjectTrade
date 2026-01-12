use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use num_traits::pow;
use num_traits::Pow;
use serde::{Deserialize, Serialize};
use strum::Display;
use strum::EnumString;
use uuid::Uuid;

#[derive(Debug, Clone, Display, EnumString, Serialize, Deserialize, sqlx::Type)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "loan_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LoanStatus {
    ONGOING,
    ONHOLD,
    PAID,
    FORGIVEN,
    DEFAULTED,
}

pub enum LoanType {
    Standard,
    Premium,
}

impl LoanType {
    pub fn get_rates(&self) -> (BigDecimal, BigDecimal) {
        match self {
            LoanType::Standard => (BigDecimal::from(100_000), BigDecimal::from(5)),
            LoanType::Premium => (BigDecimal::from(1_000_000), BigDecimal::from(10)),
        }
    }
}

pub struct Loan {
    pub loan_id: Uuid,
    pub user_id: Uuid,
    pub principal: BigDecimal,
    pub interest_rate: BigDecimal,
    pub status: LoanStatus,
    pub created_at: DateTime<Utc>,
    pub last_paid_at: DateTime<Utc>,
}

impl Loan {
    pub fn new(
        loan_id: Uuid,
        user_id: Uuid,
        principal: BigDecimal,
        interest_rate: BigDecimal,
        status: LoanStatus,
        created_at: DateTime<Utc>,
        last_paid_at: DateTime<Utc>,
    ) -> Loan {
        Loan {
            loan_id,
            user_id,
            principal,
            interest_rate,
            status,
            created_at,
            last_paid_at,
        }
    }
    pub fn get_current_balance(&self) -> BigDecimal {
        let mut balance = self.principal.clone();
        let interest_rate = BigDecimal::from(1 + &self.interest_rate / 365);
        let days = Utc::now()
            .signed_duration_since(self.last_paid_at)
            .num_days();
        let u_days: usize = if days < 0 {
            0
        } else {
            days.try_into().unwrap()
        };

        let interest_rate_over_time = pow(interest_rate, u_days);
        balance += &self.principal * interest_rate_over_time;
        balance
    }
}
