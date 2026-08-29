use crate::models::errors::ticker_error::TickerError;
use crate::models::errors::trade_error::TradeError;
use crate::models::stock_ticker::{Ticker, TickerSummary, TimeFrame};
use crate::services::synthetic_data::{get_company_by_symbol, get_synthetic_universe};
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::Utc;
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::debug;

#[derive(Clone)]
pub struct TickerService {
    pub db: PgPool,
    pub cache: Arc<RwLock<HashMap<String, (Ticker, Instant)>>>,
}

#[allow(dead_code)]
impl TickerService {
    pub fn new(db: PgPool) -> Self {
        Self {
            db,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn fetch_latest_price_ticker_from_db(
        &self,
        ticker: &str,
    ) -> Result<Ticker, TradeError> {
        debug!("Fetching ticker {} from DB", ticker);
        let stock = sqlx::query("SELECT * FROM stock_prices WHERE ticker = $1 ORDER BY date DESC LIMIT 1")
            .bind(ticker)
            .fetch_one(&self.db)
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
            .map_err(TradeError::DatabaseError)?;
        Ok(stock)
    }

    pub async fn fetch_price_history_ticker_from_db(
        &self,
        ticker: &str,
        timeframe: TimeFrame,
    ) -> Result<Vec<Ticker>, TradeError> {
        let max_date_query = "SELECT MAX(date) FROM stock_prices WHERE ticker = $1";
        let mut limit_date: chrono::DateTime<Utc> = sqlx::query(max_date_query)
            .bind(ticker)
            .fetch_one(&self.db)
            .await
            .and_then(|row| row.try_get(0))
            .unwrap_or_else(|_| chrono::Utc::now());

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
                limit_date = chrono::DateTime::from_timestamp(0, 0).unwrap();
            }
        }
        let query = "SELECT * FROM stock_prices WHERE ticker = $1 AND date >= $2 ORDER BY date ASC";

        let stocks = sqlx::query(query)
            .bind(ticker)
            .bind(limit_date)
            .fetch_all(&self.db)
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
            .map_err(TradeError::DatabaseError)?;

        Ok(stocks)
    }

    pub async fn get_all_tickers(&self) -> Result<Vec<TickerSummary>, TradeError> {
        let companies = get_synthetic_universe();
        let mut summaries = Vec::new();

        for company in companies {
            if let Ok(summary) = self.get_ticker_summary(company.symbol).await {
                summaries.push(summary);
            }
        }

        Ok(summaries)
    }

    pub async fn get_ticker_summary(&self, symbol: &str) -> Result<TickerSummary, TradeError> {
        let company = get_company_by_symbol(symbol).ok_or(TradeError::TickerError(TickerError::InvalidSymbol(symbol.to_string())))?;

        let latest = self.fetch_latest_price_ticker_from_db(company.symbol).await?;

        // Try to get previous price for 24h calculation
        let prev_record = sqlx::query(
            "SELECT close, open, high, low, volume FROM stock_prices WHERE ticker = $1 AND date < $2 ORDER BY date DESC LIMIT 1"
        )
        .bind(company.symbol)
        .bind(latest.date)
        .fetch_optional(&self.db)
        .await
        .map_err(TradeError::DatabaseError)?;

        let (open_price, day_high, day_low, day_volume, prev_close) = if let Some(row) = prev_record {
            let prev_close: BigDecimal = row.try_get("close").unwrap_or_else(|_| latest.close.clone());
            let open: BigDecimal = latest.open.clone().unwrap_or_else(|| latest.close.clone());
            let high: BigDecimal = latest.high.clone().unwrap_or_else(|| latest.close.clone());
            let low: BigDecimal = latest.low.clone().unwrap_or_else(|| latest.close.clone());
            let vol: i64 = latest.volume.unwrap_or(100_000);
            (open, high, low, vol, prev_close)
        } else {
            (
                latest.open.clone().unwrap_or_else(|| latest.close.clone()),
                latest.high.clone().unwrap_or_else(|| latest.close.clone()),
                latest.low.clone().unwrap_or_else(|| latest.close.clone()),
                latest.volume.unwrap_or(100_000),
                latest.close.clone(),
            )
        };

        let current_f64 = latest.close.to_f64().unwrap_or(company.base_price);
        let prev_f64 = prev_close.to_f64().unwrap_or(current_f64);
        let change_percent = if prev_f64 > 0.0 {
            ((current_f64 - prev_f64) / prev_f64) * 100.0
        } else {
            0.0
        };

        let market_cap = &latest.close * BigDecimal::from(company.total_shares);

        Ok(TickerSummary {
            symbol: company.symbol.to_string(),
            name: company.name.to_string(),
            sector: company.sector.to_string(),
            description: company.description.to_string(),
            current_price: latest.close,
            open_price,
            day_high,
            day_low,
            day_volume,
            change_percent: (change_percent * 100.0).round() / 100.0,
            is_positive: change_percent >= 0.0,
            market_cap,
        })
    }

    pub async fn get_active_stocks(&self) -> Vec<String> {
        sqlx::query("SELECT distinct ticker FROM stock_prices")
            .fetch_all(&self.db)
            .await
            .map_err(TradeError::DatabaseError)
            .map(|rows| {
                rows.iter()
                    .map(|row| row.try_get("ticker").unwrap())
                    .collect()
            })
            .unwrap_or_default()
    }
}
