use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use bigdecimal::BigDecimal;
use sqlx::PgPool;
use tokio::{sync::RwLock, task::JoinHandle};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    models::{
        errors::trade_error::TradeError,
        order::{Order, OrderType},
    },
    services::{ticker_service::TickerService, trade_service::TradeService},
};

struct OrderBook {
    buys: BTreeMap<BigDecimal, Vec<Order>>,
    sells: BTreeMap<BigDecimal, Vec<Order>>,
}

impl OrderBook {
    pub fn get_best_sale(&self) -> Result<(Order, Order), TradeError> {
        info!("Getting best sale");
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
        info!("Best buy: {:?}, Best sell: {:?}", best_buy, best_sell);
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
    ticker_service: Arc<TickerService>,
}

impl OrderMatchbookService {
    const ORDER_PROCESSOR_INTERVAL_SECS: u64 = 100;
    pub fn new(
        db: PgPool,
        trade_service: Arc<TradeService>,
        ticker_service: Arc<TickerService>,
    ) -> OrderMatchbookService {
        OrderMatchbookService {
            db,
            order_books: Arc::new(RwLock::new(HashMap::new())),
            trade_service,
            ticker_service,
        }
    }

    pub async fn add_order(&self, order: Order) -> Result<(), TradeError> {
        let ticker = order.ticker.clone();
        let mut books = self.order_books.write().await;
        info!("Adding order to orderbook for ticker {}", ticker);
        let order_book = books.entry(ticker).or_insert_with_key(|_ticker| OrderBook {
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

    pub async fn initialise_orderbooks(&self) -> Result<(), TradeError> {
        //load up all pending orders
        let pending_orders = self.trade_service.get_pending_orders().await?;
        for order in pending_orders {
            self.add_order(order).await?;
        }
        Ok(())
    }

    pub async fn get_open_orders(&self) -> Vec<Order> {
        let books = self.order_books.read().await;
        let mut open_orders = Vec::new();
        for order_book in books.values() {
            for orders in order_book.buys.values() {
                open_orders.extend(orders.iter().cloned());
            }
            for orders in order_book.sells.values() {
                open_orders.extend(orders.iter().cloned());
            }
        }
        open_orders
    }

    pub async fn remove_order(&self, ticker: &str, order_id: Uuid) {
        let mut books = self.order_books.write().await;
        if let Some(order_book) = books.get_mut(ticker) {
            order_book.buys.retain(|_, orders| {
                orders.retain(|o| o.order_id != order_id);
                !orders.is_empty()
            });
            order_book.sells.retain(|_, orders| {
                orders.retain(|o| o.order_id != order_id);
                !orders.is_empty()
            });
        }
    }

    pub fn create_worker_thread(&self) -> JoinHandle<Result<(), TradeError>> {
        info!("Starting order processor thread");
        let order_books = Arc::clone(&self.order_books);
        let trade_service = Arc::clone(&self.trade_service);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
                Self::ORDER_PROCESSOR_INTERVAL_SECS,
            ));
            loop {
                interval.tick().await;
                loop {
                    let mut buy_ids: Vec<(Uuid, BigDecimal, BigDecimal)> = Vec::new();
                    let mut sell_ids: Vec<(Uuid, BigDecimal, BigDecimal)> = Vec::new();
                    {
                        let books = order_books.read().await;
                        info!("Are we even reading the same books? {}", books.len());
                        for (_ticker, order_book) in books.iter() {
                            info!("Processing orderbook for ticker {}", _ticker);
                            if let Ok((best_buy, best_sell)) = order_book.get_best_sale() {
                                if best_buy.price_per_share >= best_sell.price_per_share {
                                    let match_quantity = best_buy.quantity.min(best_sell.quantity);
                                    let execution_price = best_sell.price_per_share.clone(); // Market price defined by the limit sell
                                    buy_ids.push((best_buy.order_id, match_quantity.clone(), execution_price.clone()));
                                    sell_ids.push((best_sell.order_id, match_quantity, execution_price));
                                }
                            }
                        }
                    }

                    if buy_ids.is_empty() {
                        break;
                    }

                    info!(
                        "Found {} buy orders and {} sell orders",
                        buy_ids.len(),
                        sell_ids.len()
                    );
                    
                    let mut successful_buys = HashMap::new();
                    let mut successful_sells = HashMap::new();
                    let mut failed_buys = Vec::new();
                    let mut failed_sells = Vec::new();

                    for (buy_id, match_quantity, execution_price) in buy_ids {
                        info!("Executing buy order for user {}", buy_id);
                        match trade_service
                            .execute_order(buy_id, match_quantity.clone(), execution_price)
                            .await
                        {
                            Ok(_) => {
                                successful_buys.insert(buy_id, match_quantity);
                                info!("Executed buy order for user {}", buy_id);
                            }
                            Err(e) => {
                                warn!(error = ?e, "Failed to execute order");
                                failed_buys.push(buy_id);
                            }
                        }
                    }
                    for (sell_id, match_quantity, execution_price) in sell_ids {
                        match trade_service
                            .execute_order(sell_id, match_quantity.clone(), execution_price)
                            .await
                        {
                            Ok(_) => {
                                successful_sells.insert(sell_id, match_quantity);
                            }
                            Err(e) => {
                                warn!(error = ?e, "Failed to execute order");
                                failed_sells.push(sell_id);
                            }
                        }
                    }
                    
                    {
                        let mut books = order_books.write().await;
                        for (_ticker, order_book) in books.iter_mut() {
                            order_book.buys.retain(|_, orders| {
                                for order in orders.iter_mut() {
                                    if let Some(qty) = successful_buys.get(&order.order_id) {
                                        order.quantity -= qty;
                                    }
                                }
                                orders.retain(|o| o.quantity > BigDecimal::from(0) && !failed_buys.contains(&o.order_id));
                                !orders.is_empty()
                            });
                            order_book.sells.retain(|_, orders| {
                                for order in orders.iter_mut() {
                                    if let Some(qty) = successful_sells.get(&order.order_id) {
                                        order.quantity -= qty;
                                    }
                                }
                                orders.retain(|o| o.quantity > BigDecimal::from(0) && !failed_sells.contains(&o.order_id));
                                !orders.is_empty()
                            });
                        }
                    }
                }
            }
        })
    }
}
