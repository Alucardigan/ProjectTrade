use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use bigdecimal::BigDecimal;
use sqlx::PgPool;
use tokio::{sync::RwLock, task::JoinHandle};
use tracing::warn;

use crate::{
    models::{
        errors::trade_error::TradeError,
        order::{Order, OrderType},
    },
    services::trade_service::{self, TradeService},
};

struct OrderBook {
    ticker: String,
    buys: BTreeMap<BigDecimal, Vec<Order>>,
    sells: BTreeMap<BigDecimal, Vec<Order>>,
}

impl OrderBook {
    pub fn get_best_sale(&self) -> Result<(Order, Order), TradeError> {
        let best_buy = self
            .buys
            .iter()
            .next_back()
            .map(|(_, orders)| orders[0].clone());
        let best_sell = self
            .sells
            .iter()
            .next()
            .map(|(_, orders)| orders[0].clone());
        match (best_buy, best_sell) {
            (Some(best_buy), Some(best_sell)) => Ok((best_buy, best_sell)),
            _ => Err(TradeError::NoMatchForOrder),
        }
    }
}
pub struct OrderMatchbookService {
    db: PgPool,
    order_books: Arc<RwLock<HashMap<String, OrderBook>>>,
    trade_service: Arc<TradeService>,
}

impl OrderMatchbookService {
    const ORDER_PROCESSOR_INTERVAL_SECS: u64 = 100;
    pub fn new(db: PgPool, trade_service: Arc<TradeService>) -> OrderMatchbookService {
        OrderMatchbookService {
            db,
            order_books: Arc::new(RwLock::new(HashMap::new())),
            trade_service,
        }
    }
    pub async fn add_order(&self, order: Order) -> Result<(), TradeError> {
        let ticker = order.ticker.clone();
        let mut books = self.order_books.write().await;
        let order_book = books.entry(ticker).or_insert_with_key(|ticker| OrderBook {
            ticker: ticker.clone(),
            buys: BTreeMap::new(),
            sells: BTreeMap::new(),
        });
        match order.order_type {
            OrderType::Buy => {
                order_book
                    .buys
                    .entry(order.price_per_share.clone())
                    .or_insert_with(Vec::new)
                    .push(order);
            }
            OrderType::Sell => {
                order_book
                    .sells
                    .entry(order.price_per_share.clone())
                    .or_insert_with(Vec::new)
                    .push(order);
            }
        }
        Ok(())
    }

    pub async fn create_worker_thread(&self) -> JoinHandle<Result<(), TradeError>> {
        let order_books = Arc::clone(&self.order_books);
        let trade_service = Arc::clone(&self.trade_service);
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
            Self::ORDER_PROCESSOR_INTERVAL_SECS,
        ));
        tokio::spawn(async move {
            loop {
                interval.tick().await;
                let books = order_books.read().await;
                for (ticker, order_book) in books.iter() {
                    if let Ok((best_buy, best_sell)) = order_book.get_best_sale() {
                        if best_buy.price_per_share > best_sell.price_per_share {
                            //call trade engine
                            match trade_service.execute_order(best_buy.order_id).await {
                                Ok(_) => {}
                                Err(e) => {
                                    warn!(error = ?e, "Failed to execute order");
                                }
                            }
                            match trade_service.execute_order(best_sell.order_id).await {
                                Ok(_) => {}
                                Err(e) => {
                                    warn!(error = ?e, "Failed to execute order");
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}
