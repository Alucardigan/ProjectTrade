use crate::models::errors::ticker_error::TickerError;
use crate::models::errors::trade_error::TradeError;
use crate::models::stock_ticker::Ticker;
use crate::models::stock_ticker::TimeFrame;
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
    pub async fn fetch_latest_price_ticker_from_db(
        &self,
        ticker: &str,
    ) -> Result<Ticker, TradeError> {
        debug!("Fetching ticker from DB");
        let stock = sqlx::query("SELECT * FROM stock_prices WHERE ticker = $1 ORDER BY date DESC")
            .bind(ticker)
            .fetch_one(&self.mock_db)
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

    pub async fn fetch_price_history_ticker_from_db(
        &self,
        ticker: &str,
        timeframe: TimeFrame,
    ) -> Result<Vec<Ticker>, TradeError> {
        let mut limit_date = chrono::Utc::now();
        match timeframe {
            TimeFrame::Day => {
                limit_date -= chrono::Duration::days(1);
            }
            TimeFrame::Month => {
                limit_date -= chrono::Duration::days(30);
            }
            TimeFrame::HalfYear => {
                limit_date -= chrono::Duration::days(180);
            }
            TimeFrame::Year => {
                limit_date -= chrono::Duration::days(365);
            }
            TimeFrame::FiveYear => {
                limit_date -= chrono::Duration::days(5 * 365);
            }
            TimeFrame::AllYears => {
                limit_date = chrono::DateTime::from_str("1970-01-01").unwrap();
            }
        }
        let query = "SELECT * FROM stock_prices WHERE ticker = $1 AND date >= $2 ORDER BY date ASC";

        let stocks = sqlx::query(query)
            .bind(ticker)
            .bind(limit_date)
            .fetch_all(&self.mock_db)
            .await
            .and_then(|rows| {
                rows.iter()
                    .map(|rec| {
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
                    .collect()
            })
            .map_err(|e| TradeError::DatabaseError(e));

        return stocks;
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
