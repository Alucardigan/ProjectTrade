use bigdecimal::BigDecimal;
use chrono::{Timelike, Utc};
use num_traits::{FromPrimitive, ToPrimitive};
use rand_distr::{Distribution, Normal};
use sqlx::{PgPool, Row};
use tokio::{sync::RwLock, task::JoinHandle};
use uuid::Uuid;

use crate::{
    models::{
        errors::trade_error::TradeError,
        order::{Order, OrderStatus, OrderType},
    },
    services::{order_matchbook_service::OrderMatchbookService, ticker_service::TickerService},
};
use std::{collections::HashMap, sync::Arc};

pub struct MarketMakerService {
    db: PgPool,
    ticker_service: Arc<TickerService>,
    order_matchbook_service: Arc<OrderMatchbookService>,
    acceptable_tickers: Vec<String>,
    ticker_price_paths: RwLock<HashMap<String, Vec<BigDecimal>>>,
}

impl MarketMakerService {
    const TIME_STEP: u32 = 1440;
    const STOCK_QUANTITY: u32 = 100;
    pub fn new(
        db: PgPool,
        ticker_service: Arc<TickerService>,
        order_matchbook_service: Arc<OrderMatchbookService>,
        acceptable_tickers: Vec<String>,
    ) -> Self {
        Self {
            db,
            ticker_service,
            order_matchbook_service,
            acceptable_tickers,
            ticker_price_paths: RwLock::new(HashMap::new()),
        }
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

    /**everyday we intialise the market
     * for each ticker, fetch opening price via api
     * fetch close price from db
     * generate price path using brownian motion
     * initialise the worker thread to use the price path to place orders
     **/
    pub async fn initialise_market(&self) -> Result<(), TradeError> {
        for ticker in &self.acceptable_tickers {
            let market_orders = self.generate_market_orders(ticker.clone()).await?;
            let mut ticker_price_paths = self.ticker_price_paths.write().await;
            ticker_price_paths.insert(ticker.clone(), market_orders);
        }
        self.spawn_worker_thread().await;
        Ok(())
    }
    pub async fn generate_market_orders(
        &self,
        ticker: String,
    ) -> Result<Vec<BigDecimal>, TradeError> {
        let market_price = self.ticker_service.search_symbol(&ticker).await;
        let current_price = self.get_current_price(&ticker).await?;
        let price_path = Self::brownian_motion(
            ToPrimitive::to_f64(&market_price.price_per_share).unwrap_or(0.0),
            ToPrimitive::to_f64(&current_price).unwrap_or(0.0),
            Self::TIME_STEP,
        );
        Ok(price_path)
    }

    async fn spawn_worker_thread(&self) -> JoinHandle<Result<(), TradeError>> {
        let orderbook = self.order_matchbook_service.clone();
        let current_price_paths = self.ticker_price_paths.read().await.clone();
        let current_time = Utc::now();
        let current_time_index =
            current_time.time().hour() as u32 * 60 + current_time.time().minute() as u32;
        let acceptable_tickers = self.acceptable_tickers.clone();
        tokio::spawn(async move {
            for ticker in acceptable_tickers {
                let target_price = current_price_paths
                    .get(&ticker)
                    .unwrap()
                    .get(current_time_index as usize)
                    .unwrap(); //TODO: something better than unwrap here pls
                let order = Order {
                    order_id: Uuid::new_v4(),
                    user_id: Uuid::new_v4(),
                    ticker: ticker.clone(),
                    quantity: BigDecimal::from(Self::STOCK_QUANTITY),
                    price_per_share: target_price.clone(),
                    order_type: OrderType::Buy,
                    status: OrderStatus::Pending,
                };
                orderbook.add_order(order).await?;
            }
            Ok(())
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
