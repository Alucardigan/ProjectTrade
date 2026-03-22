use bigdecimal::BigDecimal;
use num_traits::Zero;

use crate::models::errors::trade_error::TradeError;
use crate::models::order::Order;

trait ValidationStrategy {
    fn validate_order(&self, order: &Order) -> Result<(), TradeError> {
        if order.quantity <= 0 {
            return Err(TradeError::InvalidAmount);
        }
        Ok(())
    }
}

struct UserOrderValidator {}
impl ValidationStrategy for UserOrderValidator {
    fn validate_order(&self, order: &Order) -> Result<(), TradeError> {
        if order.quantity <= BigDecimal::zero() {
            return Err(TradeError::InvalidAmount);
        }
        if order.price_per_share <= BigDecimal::zero() {
            return Err(TradeError::InvalidAmount);
        }

        Ok(())
    }
}

struct MarketOrderValidator {}
impl ValidationStrategy for MarketOrderValidator {
    fn validate_order(&self, order: &Order) -> Result<(), TradeError> {
        if order.quantity <= BigDecimal::zero() {
            return Err(TradeError::InvalidAmount);
        }
        Ok(())
    }
}
