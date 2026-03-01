use bigdecimal::BigDecimal;
use chrono::{Timelike, Utc};
use num_traits::{FromPrimitive, ToPrimitive, Zero};
use rand_distr::{Distribution, Normal};
use sqlx::{PgPool, Row};
use tokio::{sync::RwLock, task::JoinHandle};
use tracing::info;
use uuid::Uuid;

use crate::{
    models::{
        errors::trade_error::TradeError,
        order::{Order, OrderStatus, OrderType},
    },
    services::{
        order_management_service::OrderManagementService,
        order_matchbook_service::OrderMatchbookService, ticker_service::TickerService,
    },
};
use std::{collections::HashMap, sync::Arc};

pub struct MarketMakerService {
    db: PgPool,
    ticker_service: Arc<TickerService>,
    order_matchbook_service: Arc<OrderMatchbookService>,
    order_management_service: Arc<OrderManagementService>,
    acceptable_tickers: Vec<String>,
    ticker_price_paths: RwLock<HashMap<String, Vec<BigDecimal>>>,
    market_maker_user_id: Uuid,
}

impl MarketMakerService {
    const TIME_STEP: u32 = 1440;
    const STOCK_QUANTITY: u32 = 100;
    const SPREAD_PERCENTAGE: f64 = 0.005;
    const POSTING_FREQUENCY_SECS: u64 = 60;

    pub fn new(
        db: PgPool,
        ticker_service: Arc<TickerService>,
        order_matchbook_service: Arc<OrderMatchbookService>,
        order_management_service: Arc<OrderManagementService>,
        acceptable_tickers: Vec<String>,
        user_id: Uuid,
    ) -> Self {
        Self {
            db,
            ticker_service,
            order_matchbook_service,
            order_management_service,
            acceptable_tickers,
            ticker_price_paths: RwLock::new(HashMap::new()),
            market_maker_user_id: user_id,
        }
    }

    pub async fn get_current_price(
        &self,
        ticker: &String,
    ) -> Result<Option<BigDecimal>, TradeError> {
        info!("get current price");
        let current_price_opt = sqlx::query(
            "SELECT close FROM stock_prices WHERE ticker = $1 ORDER BY date DESC LIMIT 1",
        )
        .bind(ticker)
        .fetch_optional(&self.db)
        .await?;

        if let Some(row) = current_price_opt {
            let price: BigDecimal = row.try_get("close")?;
            info!("Current price {} for ticker {}", price, ticker);
            Ok(Some(price))
        } else {
            info!("No current price found for ticker {}", ticker);
            Ok(None)
        }
    }

    /**everyday we intialise the market
     * for each ticker, fetch opening price via api
     * fetch close price from db
     * generate price path using brownian motion
     * initialise the worker thread to use the price path to place orders
     **/
    pub async fn initialise_market(&self) -> Result<(), TradeError> {
        info!(
            "Initialising market for tickers : {:?}",
            self.acceptable_tickers
        );
        for ticker in &self.acceptable_tickers {
            let market_orders = self.generate_market_orders(ticker.clone()).await?;
            let mut ticker_price_paths = self.ticker_price_paths.write().await;
            ticker_price_paths.insert(ticker.clone(), market_orders);
        }
        // self.spawn_worker_thread().await;
        Ok(())
    }
    pub async fn generate_market_orders(
        &self,
        ticker: String,
    ) -> Result<Vec<BigDecimal>, TradeError> {
        let market_price = self.ticker_service.search_symbol(&ticker).await?;
        let current_price_opt = self.get_current_price(&ticker).await?;
        let start_price = match current_price_opt {
            Some(price) => price,
            None => market_price.price_per_share.clone(),
        };

        let price_path = Self::brownian_motion(
            ToPrimitive::to_f64(&market_price.price_per_share).unwrap_or(0.0),
            ToPrimitive::to_f64(&start_price).unwrap_or(0.0),
            Self::TIME_STEP,
        );
        Ok(price_path)
    }

    pub async fn spawn_worker_thread(&self) -> JoinHandle<Result<(), TradeError>> {
        info!("Starting market maker thread");
        let order_management_service = self.order_management_service.clone();
        let acceptable_tickers = self.acceptable_tickers.clone();
        let user_id = self.market_maker_user_id;

        // We clone the Arc to move it into the background thread so it can constantly read live states
        let price_paths_ref = Arc::new(self.ticker_price_paths.read().await.clone());

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
                Self::POSTING_FREQUENCY_SECS,
            ));
            loop {
                interval.tick().await;
                let current_time = Utc::now();
                let current_time_index =
                    (current_time.time().hour() * 60 + current_time.time().minute()) as usize;

                for ticker in &acceptable_tickers {
                    let default_value = BigDecimal::from(120);

                    let target_price = match price_paths_ref.get(ticker) {
                        Some(paths) => paths.get(current_time_index).unwrap_or(&default_value),
                        None => {
                            tracing::warn!("No price path found for ticker {}", ticker);
                            &default_value
                        }
                    };

                    let target_price_f64 = target_price.to_f64().unwrap_or(120.0);

                    let bid_price = target_price_f64 * (1.0 - Self::SPREAD_PERCENTAGE);
                    let ask_price = target_price_f64 * (1.0 + Self::SPREAD_PERCENTAGE);

                    let bid_decimal =
                        BigDecimal::from_f64(bid_price).unwrap_or(target_price.clone());
                    let ask_decimal =
                        BigDecimal::from_f64(ask_price).unwrap_or(target_price.clone());

                    if let Err(e) = order_management_service
                        .place_order(
                            user_id,
                            &ticker.clone(),
                            BigDecimal::from(Self::STOCK_QUANTITY),
                            OrderType::Buy,
                            BigDecimal::zero(),
                            Some(bid_decimal),
                        )
                        .await
                    {
                        tracing::error!(
                            "Market Maker failed to place Buy order for {}: {:?}",
                            ticker,
                            e
                        );
                    }

                    if let Err(e) = order_management_service
                        .place_order(
                            user_id,
                            &ticker.clone(),
                            BigDecimal::from(Self::STOCK_QUANTITY),
                            OrderType::Sell,
                            BigDecimal::zero(),
                            Some(ask_decimal),
                        )
                        .await
                    {
                        tracing::error!(
                            "Market Maker failed to place Sell order for {}: {:?}",
                            ticker,
                            e
                        );
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

            // Interpolate the error removal
            // At i=0, we remove 0% of error. At i=N, we remove 100% of error.
            let progress = i as f64 / time_step as f64;
            let bridged_log = raw_log - (total_error * progress);

            let price_f64 = bridged_log.exp();
            let price_bigdecimal: BigDecimal = FromPrimitive::from_f64(price_f64).unwrap();

            result_path.push(price_bigdecimal);
        }
        result_path
    }
}
