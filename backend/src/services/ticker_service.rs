use alpha_vantage::ApiClient;
use bigdecimal::BigDecimal;
use chrono::Utc;
use num_traits::FromPrimitive;
use num_traits::Zero;
use sqlx::PgPool;
use sqlx::Row;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

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

    #[tracing::instrument(skip(self))]
    pub async fn search_symbol(&self, symbol: &str) -> Ticker {
        // Check cache
        {
            let cache = self.cache.read().await;
            if let Some((ticker, timestamp)) = cache.get(symbol) {
                // Cache valid for 60 seconds
                if timestamp.elapsed() < Duration::from_secs(60) {
                    return ticker.clone();
                }
            }
        }

        let ticker = self.get_or_fetch_price_for_today(symbol).await;

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(symbol.to_string(), (ticker.clone(), Instant::now()));
        }
        ticker
    }

    pub async fn fetch_from_api(&self, symbol: &str) -> Ticker {
        let mut ticker = Ticker {
            symbol: symbol.into(),
            date: Utc::now(),
            close: BigDecimal::from(120), // default
            volume: None,
            open: None,
            high: None,
            low: None,
        };

        if self.api_client.get_api_key() == "mock" {
            return ticker;
        }

        match self
            .api_client
            .stock_time(alpha_vantage::stock_time::StockFunction::Daily, symbol)
            .output_size(alpha_vantage::api::OutputSize::Compact)
            .json()
            .await
        {
            Ok(stock_time_response) => {
                let stock_time = stock_time_response.data();
                if let Some(latest) = stock_time.first() {
                    ticker.close =
                        BigDecimal::from_f64(latest.close()).unwrap_or(BigDecimal::from(120));
                }
            }
            Err(_) => {
                tracing::warn!("Api limit reached or error fetching data for {}", symbol);
            }
        };

        ticker
    }

    pub async fn fetch_from_db(&self, symbol: &str) -> Ticker {
        let row_opt =
            sqlx::query("SELECT * FROM stock_prices WHERE ticker = $1 ORDER BY date DESC LIMIT 1")
                .bind(symbol)
                .fetch_optional(&self.mock_db)
                .await
                .unwrap_or(None);

        if let Some(row) = row_opt {
            Ticker {
                symbol: symbol.into(),
                date: row.try_get("date").unwrap_or_else(|_| Utc::now()),
                close: row
                    .try_get("close")
                    .unwrap_or_else(|_| BigDecimal::from(120)),
                volume: row.try_get("volume").ok(),
                open: row.try_get("open").ok(),
                high: row.try_get("high").ok(),
                low: row.try_get("low").ok(),
            }
        } else {
            Ticker {
                symbol: symbol.into(),
                date: Utc::now(),
                close: BigDecimal::from(120),
                volume: None,
                open: None,
                high: None,
                low: None,
            }
        }
    }

    pub async fn get_or_fetch_price_for_today(&self, symbol: &str) -> Ticker {
        let has_today_price =
            sqlx::query("SELECT 1 FROM stock_prices WHERE ticker = $1 AND date >= CURRENT_DATE")
                .bind(symbol)
                .fetch_optional(&self.mock_db)
                .await
                .unwrap_or(None)
                .is_some();

        if has_today_price {
            self.fetch_from_db(symbol).await
        } else {
            let ticker = self.fetch_from_api(symbol).await;

            let _ = sqlx::query(
                "INSERT INTO stock_prices (ticker, date, close, volume, open, high, low) VALUES ($1, NOW(), $2, $3, $4, $5, $6)",
            )
            .bind(symbol)
            .bind(&ticker.close)
            .bind(&ticker.volume)
            .bind(&ticker.open)
            .bind(&ticker.high)
            .bind(&ticker.low)
            .execute(&self.mock_db)
            .await;

            ticker
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn search_multiple_symbols(&self, symbols: Vec<&str>) {
        let mut tickers: Vec<Ticker> = vec![];
        for symbol in symbols {
            let ticker = self.search_symbol(symbol).await;
            tickers.push(ticker);
        }
    }
    //search function to find nearest matching symbol
    pub async fn query_similar_symbol(&self, _symbol: &str) {}
}
