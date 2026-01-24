use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use bigdecimal::BigDecimal;
use sqlx::PgPool;
use tokio::sync::RwLock;

use crate::models::{
    errors::trade_error::TradeError,
    order::{Order, OrderType},
};

struct OrderBook {
    ticker: String,
    buys: BTreeMap<BigDecimal, Vec<Order>>,
    sells: BTreeMap<BigDecimal, Vec<Order>>,
}
pub struct OrderMatchbookService {
    db: PgPool,
    order_books: Arc<RwLock<HashMap<String, OrderBook>>>,
}

impl OrderMatchbookService {
    pub fn new(db: PgPool) -> OrderMatchbookService {
        OrderMatchbookService {
            db,
            order_books: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub async fn add_order(&self, order: Order) -> Result<(), TradeError> {
        let ticker = order.ticker.clone();
        let mut books = self.order_books.write().await;
        let mut order_book = books.entry(ticker).or_insert_with_key(|ticker| OrderBook {
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
    pub async fn get_best_sale(
        &self,
        ticker: &str,
    ) -> Result<(Option<Order>, Option<Order>), TradeError> {
        let books = self.order_books.read().await;
        let order_book = books.get(ticker).ok_or(TradeError::OrderBookNotFound)?;
        let best_buy = order_book
            .buys
            .iter()
            .next_back()
            .map(|(_, orders)| orders[0].clone());
        let best_sell = order_book
            .sells
            .iter()
            .next()
            .map(|(_, orders)| orders[0].clone());
        Ok((best_buy, best_sell))
    }
}
