use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use bigdecimal::BigDecimal;
use sqlx::PgPool;
use tokio::{sync::RwLock, task::JoinHandle};
use tracing::warn;
use uuid::Uuid;

use crate::{
    models::{
        errors::trade_error::TradeError,
        order::{Order, OrderType},
    },
    services::{order_management_service, trade_service::TradeService},
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
            .next_back() //get highest price
            .and_then(|(_, orders)| orders.first().cloned());
        let best_sell = self
            .sells
            .iter()
            .next() //get lowest price
            .and_then(|(_, orders)| orders.first().cloned());
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
                let mut buy_ids: Vec<(Uuid, BigDecimal)> = Vec::new();
                let mut sell_ids: Vec<(Uuid, BigDecimal)> = Vec::new();
                for (_ticker, order_book) in books.iter() {
                    if let Ok((best_buy, best_sell)) = order_book.get_best_sale() {
                        if best_buy.price_per_share >= best_sell.price_per_share {
                            let match_quantity = best_buy.quantity.min(best_sell.quantity);
                            buy_ids.push((best_buy.order_id, match_quantity.clone()));
                            sell_ids.push((best_sell.order_id, match_quantity));
                        }
                    }
                }
                // This is currently N+1 and also non atomic. Refactor trade to get better perf
                //execute orders
                let mut successful_buys = HashMap::new();
                let mut successful_sells = HashMap::new();

                for (buy_id, match_quantity) in buy_ids {
                    match trade_service
                        .execute_order(buy_id, match_quantity.clone())
                        .await
                    {
                        Ok(_) => {
                            successful_buys.insert(buy_id, match_quantity);
                        }
                        Err(e) => {
                            warn!(error = ?e, "Failed to execute order");
                        }
                    }
                }
                for (sell_id, match_quantity) in sell_ids {
                    match trade_service
                        .execute_order(sell_id, match_quantity.clone())
                        .await
                    {
                        Ok(_) => {
                            successful_sells.insert(sell_id, match_quantity);
                        }
                        Err(e) => {
                            warn!(error = ?e, "Failed to execute order");
                        }
                    }
                }
                /*
                Why cant this be done in the Ok() section of the match?:
                would have to acquire write lock everytime for every single match. Needlessly pointless
                */
                {
                    let mut books = order_books.write().await;
                    for (_ticker, order_book) in books.iter_mut() {
                        order_book.buys.retain(|_, orders| {
                            for order in orders.iter_mut() {
                                if let Some(qty) = successful_buys.get(&order.order_id) {
                                    order.quantity -= qty;
                                }
                            }
                            orders.retain(|o| o.quantity > BigDecimal::from(0));
                            !orders.is_empty()
                        });
                        order_book.sells.retain(|_, orders| {
                            for order in orders.iter_mut() {
                                if let Some(qty) = successful_sells.get(&order.order_id) {
                                    order.quantity -= qty;
                                }
                            }
                            orders.retain(|o| o.quantity > BigDecimal::from(0));
                            !orders.is_empty()
                        });
                    }
                }
            }
        })
    }
}
