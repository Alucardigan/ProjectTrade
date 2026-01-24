use bigdecimal::BigDecimal;
use sqlx::{PgPool, Row};

use crate::{models::errors::trade_error::TradeError, services::ticker_service::TickerService};
use std::sync::Arc;

pub struct MarketMakerService {
    db: PgPool,
    ticker_service: Arc<TickerService>,
}

impl MarketMakerService {
    pub fn new(db: PgPool, ticker_service: Arc<TickerService>) -> Self {
        Self { db, ticker_service }
    }

    pub async fn get_current_price(&self, ticker: &String) -> Result<BigDecimal, TradeError> {
        let current_price: BigDecimal = sqlx::query(
            "SELECT close FROM stock_prices WHERE symbol = $1 ORDER BY time DESC LIMIT 1",
        )
        .bind(ticker)
        .fetch_one(&self.db)
        .await?
        .try_get("close")?;
        Ok(current_price)
    }
    pub async fn generate_market_orders(&self, ticker: String) {
        let market_price = self.ticker_service.search_symbol(&ticker);
        let current_price = self.get_current_price(&ticker).await?;
    }
}
