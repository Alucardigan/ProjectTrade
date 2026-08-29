use bigdecimal::BigDecimal;
use chrono::Utc;
use num_traits::{FromPrimitive, ToPrimitive, Zero};
use rand::Rng;
use rand_distr::{Distribution, Normal};
use sqlx::{PgPool, Row};
use std::{collections::HashMap, sync::Arc};
use tokio::{sync::RwLock, task::JoinHandle};
use tracing::info;
use uuid::Uuid;

use crate::{
    models::{errors::trade_error::TradeError, order::OrderType},
    services::{
        order_management_service::OrderManagementService,
        synthetic_data::{get_company_by_symbol, get_universe_symbols},
        ticker_service::TickerService,
    },
};

pub struct MarketMakerService {
    db: PgPool,
    ticker_service: Arc<TickerService>,
    order_management_service: Arc<OrderManagementService>,
    acceptable_tickers: Vec<String>,
    ticker_price_paths: RwLock<HashMap<String, Vec<BigDecimal>>>,
    market_maker_user_id: Uuid,
}

impl MarketMakerService {
    const TIME_STEP: u32 = 1440;
    const STOCK_QUANTITY: u32 = 100;
    const SPREAD_PERCENTAGE: f64 = 0.005; // 0.5% bid-ask spread
    const POSTING_FREQUENCY_SECS: u64 = 5;

    pub fn new(
        db: PgPool,
        ticker_service: Arc<TickerService>,
        order_management_service: Arc<OrderManagementService>,
        acceptable_tickers: Vec<String>,
        user_id: Uuid,
    ) -> Self {
        let tickers = if acceptable_tickers.is_empty() {
            get_universe_symbols()
        } else {
            acceptable_tickers
        };

        Self {
            db,
            ticker_service,
            order_management_service,
            acceptable_tickers: tickers,
            ticker_price_paths: RwLock::new(HashMap::new()),
            market_maker_user_id: user_id,
        }
    }

    pub async fn get_current_price(
        &self,
        ticker: &str,
    ) -> Result<Option<BigDecimal>, TradeError> {
        let current_price_opt = sqlx::query(
            "SELECT close FROM stock_prices WHERE ticker = $1 ORDER BY date DESC LIMIT 1",
        )
        .bind(ticker)
        .fetch_optional(&self.db)
        .await
        .map_err(TradeError::DatabaseError)?;

        if let Some(row) = current_price_opt {
            let price: BigDecimal = row.try_get("close").unwrap_or_else(|_| BigDecimal::from(100));
            Ok(Some(price))
        } else {
            Ok(None)
        }
    }

    pub async fn initialise_market(&self) -> Result<(), TradeError> {
        info!(
            "Initialising synthetic market for tickers: {:?}",
            self.acceptable_tickers
        );
        for ticker in &self.acceptable_tickers {
            let market_orders = self.generate_market_orders(ticker.clone()).await?;
            let mut ticker_price_paths = self.ticker_price_paths.write().await;
            ticker_price_paths.insert(ticker.clone(), market_orders);
        }
        Ok(())
    }

    pub async fn generate_market_orders(
        &self,
        ticker: String,
    ) -> Result<Vec<BigDecimal>, TradeError> {
        let latest_price_opt = self.get_current_price(&ticker).await?;
        let base_price = get_company_by_symbol(&ticker)
            .map(|c| c.base_price)
            .unwrap_or(150.0);

        let start_price = match latest_price_opt {
            Some(price) => price.to_f64().unwrap_or(base_price),
            None => base_price,
        };

        let target_price = start_price * (1.0 + (rand::random::<f64>() - 0.5) * 0.02);
        let price_path = Self::brownian_motion(target_price, start_price, Self::TIME_STEP);
        Ok(price_path)
    }

    pub async fn spawn_price_engine(&self) -> JoinHandle<Result<(), TradeError>> {
        info!("Starting synthetic price engine background worker");
        let db = self.db.clone();
        let acceptable_tickers = self.acceptable_tickers.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
                Self::POSTING_FREQUENCY_SECS,
            ));

            loop {
                interval.tick().await;
                let now = Utc::now();

                for ticker in &acceptable_tickers {
                    let company_opt = get_company_by_symbol(ticker);
                    let (volatility, drift) = match company_opt {
                        Some(c) => (c.volatility, c.drift),
                        None => (0.25, 0.08),
                    };

                    // Fetch latest recorded candle
                    let latest_row = sqlx::query(
                        "SELECT close, open, high, low, volume FROM stock_prices WHERE ticker = $1 ORDER BY date DESC LIMIT 1"
                    )
                    .bind(ticker)
                    .fetch_optional(&db)
                    .await;

                    if let Ok(Some(row)) = latest_row {
                        let prev_close_bd: BigDecimal = row.try_get("close").unwrap_or_else(|_| BigDecimal::from(100));
                        let prev_close = prev_close_bd.to_f64().unwrap_or(100.0);

                        // Stochastic step for 5 seconds (calculate without holding ThreadRng across await)
                        let (close, open, high, low, volume) = {
                            let mut rng = rand::thread_rng();
                            let dt = 5.0 / (252.0 * 24.0 * 3600.0);
                            let normal = Normal::new(
                                (drift - 0.5 * volatility.powi(2)) * dt,
                                volatility * dt.sqrt(),
                            )
                            .unwrap();

                            let shock = normal.sample(&mut rng);
                            let new_price_f64 = ((prev_close * shock.exp()) * 100.0).round() / 100.0;
                            let new_price_f64 = new_price_f64.max(1.0);

                            let open = prev_close;
                            let close = new_price_f64;
                            let high = open.max(close) * (1.0 + rng.gen::<f64>() * 0.001);
                            let low = (open.min(close) * (1.0 - rng.gen::<f64>() * 0.001)).max(0.5);
                            let volume = 1000 + rng.gen_range(100..5000);
                            (close, open, high, low, volume)
                        };

                        let close_bd = BigDecimal::from_f64(close).unwrap_or(prev_close_bd);
                        let open_bd = BigDecimal::from_f64(open).unwrap();
                        let high_bd = BigDecimal::from_f64(high).unwrap();
                        let low_bd = BigDecimal::from_f64(low).unwrap();

                        // Update or insert latest live tick into DB
                        let _ = sqlx::query(
                            "INSERT INTO stock_prices (ticker, date, close, volume, open, high, low) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (ticker, date) DO UPDATE SET close = EXCLUDED.close, high = GREATEST(stock_prices.high, EXCLUDED.high), low = LEAST(stock_prices.low, EXCLUDED.low), volume = stock_prices.volume + EXCLUDED.volume"
                        )
                        .bind(ticker)
                        .bind(now)
                        .bind(close_bd)
                        .bind(volume)
                        .bind(open_bd)
                        .bind(high_bd)
                        .bind(low_bd)
                        .execute(&db)
                        .await;
                    }
                }
            }
        })
    }

    pub async fn spawn_market_maker_thread(&self) -> JoinHandle<Result<(), TradeError>> {
        info!("Starting synthetic market maker background liquidity worker");
        let order_management_service = self.order_management_service.clone();
        let acceptable_tickers = self.acceptable_tickers.clone();
        let user_id = self.market_maker_user_id;
        let db = self.db.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3));
            loop {
                interval.tick().await;

                // 1. Maintain bid/ask quotes around fair value for all synthetic stocks
                for ticker in &acceptable_tickers {
                    let latest_price_opt = sqlx::query(
                        "SELECT close FROM stock_prices WHERE ticker = $1 ORDER BY date DESC LIMIT 1",
                    )
                    .bind(ticker)
                    .fetch_optional(&db)
                    .await;

                    if let Ok(Some(row)) = latest_price_opt {
                        let fair_price: BigDecimal = row.try_get("close").unwrap_or_else(|_| BigDecimal::from(100));
                        let fair_f64 = fair_price.to_f64().unwrap_or(100.0);

                        let bid_f64 = ((fair_f64 * (1.0 - Self::SPREAD_PERCENTAGE)) * 100.0).round() / 100.0;
                        let ask_f64 = ((fair_f64 * (1.0 + Self::SPREAD_PERCENTAGE)) * 100.0).round() / 100.0;

                        let bid_price = BigDecimal::from_f64(bid_f64).unwrap_or_else(|| fair_price.clone());
                        let ask_price = BigDecimal::from_f64(ask_f64).unwrap_or_else(|| fair_price.clone());

                        // Place Market Maker Bid (Buy)
                        let _ = order_management_service
                            .place_order(
                                user_id,
                                ticker,
                                BigDecimal::from(Self::STOCK_QUANTITY),
                                OrderType::Buy,
                                BigDecimal::zero(),
                                Some(bid_price),
                            )
                            .await;

                        // Place Market Maker Ask (Sell)
                        let _ = order_management_service
                            .place_order(
                                user_id,
                                ticker,
                                BigDecimal::from(Self::STOCK_QUANTITY),
                                OrderType::Sell,
                                BigDecimal::zero(),
                                Some(ask_price),
                            )
                            .await;
                    }
                }

                // 2. Fill matching user open orders
                let open_orders = order_management_service
                    .order_matchbook_service
                    .get_open_orders()
                    .await;

                for order in open_orders {
                    if order.user_id == user_id {
                        continue;
                    }

                    let current_price_row = sqlx::query(
                        "SELECT close FROM stock_prices WHERE ticker = $1 ORDER BY date DESC LIMIT 1",
                    )
                    .bind(&order.ticker)
                    .fetch_optional(&db)
                    .await;

                    if let Ok(Some(row)) = current_price_row {
                        let target_price: BigDecimal = row.try_get("close").unwrap_or_else(|_| BigDecimal::from(100));
                        let max_deviation = &target_price * BigDecimal::from_f64(0.02).unwrap_or_default();
                        let deviation = (&order.price_per_share - &target_price).abs();

                        if deviation <= max_deviation {
                            let counter_order_type = match order.order_type {
                                OrderType::Buy => OrderType::Sell,
                                OrderType::Sell => OrderType::Buy,
                            };

                            let _ = order_management_service
                                .place_order(
                                    user_id,
                                    &order.ticker,
                                    order.quantity.clone(),
                                    counter_order_type,
                                    BigDecimal::zero(),
                                    Some(order.price_per_share.clone()),
                                )
                                .await;
                        }
                    }
                }
            }
        })
    }

    fn brownian_motion(target_price: f64, current_price: f64, time_step: u32) -> Vec<BigDecimal> {
        let mut rng = rand::thread_rng();
        let sigma = 0.2;
        let dt = 1.0 / time_step as f64;
        let normal_dist = Normal::new(0.0, sigma * dt.sqrt()).unwrap();

        let log_start = current_price.ln();
        let log_target = target_price.ln();

        let mut current_log = log_start;
        let mut log_path = Vec::with_capacity((time_step + 1) as usize);
        log_path.push(log_start);
        for _ in 0..time_step {
            let noise = normal_dist.sample(&mut rng);
            current_log += noise;
            log_path.push(current_log);
        }

        let final_walk_val = log_path.last().copied().unwrap();
        let total_error = final_walk_val - log_target;

        let mut result_path = Vec::with_capacity(time_step as usize);
        for i in 1..=time_step {
            let raw_log = log_path[i as usize];
            let progress = i as f64 / time_step as f64;
            let bridged_log = raw_log - (total_error * progress);

            let price_f64 = bridged_log.exp();
            let price_bigdecimal: BigDecimal = FromPrimitive::from_f64(price_f64).unwrap();
            result_path.push(price_bigdecimal);
        }
        result_path
    }
}
