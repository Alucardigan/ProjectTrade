use alpha_vantage::ApiClient;
use sqlx::PgPool;

use crate::models::stock_ticker::Ticker;
pub struct TickerService {
    pub api_client: ApiClient,
    pub mock_db: PgPool,
}
#[allow(dead_code)]
impl TickerService {
    pub fn new(api_key: &str, mock_db: PgPool) -> Self {
        Self {
            api_client: (alpha_vantage::set_api(api_key, reqwest::Client::new())),
            mock_db: mock_db,
        }
    }
    pub async fn search_symbol(&self, symbol: &str) -> Ticker {
        if self.api_client.get_api_key() == "mock" {
            let prices = sqlx::query!(
                "SELECT close FROM stock_prices WHERE ticker = $1 ORDER BY date DESC LIMIT 5",
                symbol
            )
            .fetch_all(&self.mock_db)
            .await
            .map(|rows| {
                rows.into_iter()
                    .map(|r| {
                        r.close
                            .and_then(|bd| num_traits::ToPrimitive::to_f64(&bd))
                            .unwrap_or(0.0)
                    })
                    .collect::<Vec<f64>>()
            })
            .unwrap_or_else(|_| vec![120.0, 121.0, 122.0, 123.0, 124.0]);

            let price = *prices.first().unwrap_or(&0.0);
            return Ticker {
                symbol: symbol.into(),
                price,
                trend: prices,
            };
        }
        let stock_time_prices = match self
            .api_client
            .stock_time(alpha_vantage::stock_time::StockFunction::Daily, symbol)
            .output_size(alpha_vantage::api::OutputSize::Compact)
            .json()
            .await
        {
            Ok(stock_time_response) => {
                let stock_time = stock_time_response.data();
                let prices: Vec<f64> = stock_time
                    .iter()
                    .map(|time_data| time_data.close())
                    .collect();
                prices
            }
            Err(_) => {
                println!("Api limit reached");
                let prices: Vec<f64> = vec![120.0, 121.0, 122.0, 123.0, 124.0];
                prices
            }
        };
        return Ticker {
            symbol: symbol.into(),
            price: stock_time_prices[0],
            trend: stock_time_prices,
        };
    }
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
