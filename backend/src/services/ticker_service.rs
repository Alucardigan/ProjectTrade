use alpha_vantage::ApiClient;
use bigdecimal::BigDecimal;
use chrono::Utc;
use num_traits::FromPrimitive;
use num_traits::Zero;
use sqlx::PgPool;
use sqlx::Row;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::debug;
use tracing::info;

use crate::models::errors::ticker_error::TickerError;
use crate::models::errors::trade_error::TradeError;
use crate::models::stock_ticker::Ticker;

#[derive(Clone)]
pub struct TickerService {
    pub api_client: Arc<ApiClient>,
    pub mock_db: PgPool,
    pub cache: Arc<RwLock<HashMap<String, (Ticker, Instant)>>>,
}

#[allow(dead_code)]
impl TickerService {
    pub fn new(api_key: &str, mock_db: PgPool) -> Self {
        Self {
            api_client: Arc::new(alpha_vantage::set_api(api_key, reqwest::Client::new())),
            mock_db,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub async fn fetch_ticker_from_db(&self, ticker: &str) -> Result<Ticker, TradeError> {
        debug!("Fetching ticker from DB");
        let stock = sqlx::query("SELECT * FROM stock_prices WHERE ticker = $1 ORDER BY date DESC")
            .bind(ticker)
            .fetch_optional(&self.mock_db)
            .await
            .and_then(|rec| {
                Ok(Ticker {
                    ticker: rec.try_get("ticker")?,
                    date: rec.try_get("date")?,
                    close: rec.try_get("close")?,
                    volume: rec.try_get("volume")?,
                    open: rec.try_get("open")?,
                    high: rec.try_get("high")?,
                    low: rec.try_get("low")?,
                })
            })
            .map_err(|e| TradeError::DatabaseError(e));
        return stock;
    }

    pub async fn fetch_ticker_from_api(&self, ticker: &str) -> Result<Ticker, TradeError> {
        debug!("Fetching ticker from API");
        let api_response = self
            .api_client
            .stock_time(alpha_vantage::stock_time::StockFunction::Daily, ticker)
            .output_size(alpha_vantage::api::OutputSize::Compact)
            .json()
            .await
            .map_err(|e| TradeError::TickerError(TickerError::AlphaVantageError(e)))?;
        let api_data = api_response.data();
        if let Some(latest) = api_data.first() {
            let ticker = Ticker {
                ticker: ticker.to_string(),
                date: chrono::DateTime::from_str(latest.time()).unwrap_or(Utc::now()),
                close: BigDecimal::from_f64(latest.close()).unwrap_or(BigDecimal::from(120)),
                volume: latest.volume().try_into().ok(),
                open: Some(BigDecimal::from_f64(latest.open()).unwrap_or(BigDecimal::from(120))),
                high: Some(BigDecimal::from_f64(latest.high()).unwrap_or(BigDecimal::from(120))),
                low: Some(BigDecimal::from_f64(latest.low()).unwrap_or(BigDecimal::from(120))),
            };
            return Ok(ticker);
        }
        Err(TradeError::TickerError(TickerError::InvalidSymbol(
            ticker.to_string(),
        )))
    }

    pub async fn get_active_stocks(&self) -> Vec<String> {
        sqlx::query("SELECT distinct ticker FROM stock_prices")
            .fetch_all(&self.mock_db)
            .await
            .map_err(|e| TradeError::DatabaseError(e))
            .map(|rows| {
                rows.iter()
                    .map(|row| row.try_get("ticker").unwrap())
                    .collect()
            })
            .unwrap_or_default()
    }
}
