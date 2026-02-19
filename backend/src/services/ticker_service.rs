use alpha_vantage::ApiClient;
use bigdecimal::BigDecimal;
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

        let ticker = if self.api_client.get_api_key() == "mock" {
            let prices = sqlx::query(
                "SELECT close FROM stock_prices WHERE symbol = $1 ORDER BY time DESC LIMIT 5",
            )
            .bind(symbol)
            .fetch_all(&self.mock_db)
            .await
            .map(|rows| {
                rows.iter()
                    .map(|row| row.try_get("close").unwrap_or_default())
                    .collect::<Vec<BigDecimal>>()
            })
            .unwrap_or_else(|_| {
                vec![
                    BigDecimal::from(120),
                    BigDecimal::from(121),
                    BigDecimal::from(122),
                    BigDecimal::from(123),
                    BigDecimal::from(124),
                ]
            });
            let zero = BigDecimal::zero();
            let price = prices.first().unwrap_or(&zero).clone();
            Ticker {
                symbol: symbol.into(),
                price_per_share: price,
                trend: prices,
            }
        } else {
            let stock_time_prices = match self
                .api_client
                .stock_time(alpha_vantage::stock_time::StockFunction::Daily, symbol)
                .output_size(alpha_vantage::api::OutputSize::Compact)
                .json()
                .await
            {
                Ok(stock_time_response) => {
                    let stock_time = stock_time_response.data();
                    stock_time
                        .iter()
                        .map(|time_data| BigDecimal::from_f64(time_data.close()).unwrap())
                        .collect()
                }
                Err(_) => {
                    tracing::warn!("Api limit reached or error fetching data for {}", symbol);
                    vec![
                        BigDecimal::from(120),
                        BigDecimal::from(121),
                        BigDecimal::from(122),
                        BigDecimal::from(123),
                        BigDecimal::from(124),
                    ]
                }
            };
            Ticker {
                symbol: symbol.into(),
                price_per_share: stock_time_prices
                    .first()
                    .unwrap_or(&BigDecimal::zero())
                    .clone(),
                trend: stock_time_prices,
            }
        };

        // Update cache
        {
            let mut cache = self.cache.write().await;
            cache.insert(symbol.to_string(), (ticker.clone(), Instant::now()));
        }
        ticker
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
