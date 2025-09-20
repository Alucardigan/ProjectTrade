use crate::models::stock_ticker::Ticker;

pub async fn demo_tickers() -> Vec<Ticker> {
    vec![
        searchSymbol("AAPL").await,
        searchSymbol("AMZN").await,
        searchSymbol("TSLA").await
    ]
}
pub async fn searchSymbol(symbol: &str)->Ticker{
    let api = alpha_vantage::set_api("", reqwest::Client::new());
    let stockTimePrices =match api
        .stock_time(alpha_vantage::stock_time::StockFunction::Daily, symbol)
        .output_size(alpha_vantage::api::OutputSize::Compact)
        .json()
        .await
        {
            Ok((stock_time_response)) =>{
                let stock_time = stock_time_response.data();
                let prices: Vec<f64> = stock_time.iter().map(|time_data| time_data.close()).collect();
                prices
            }
            Err(_)=>{
                println!("Api limit reached");
                let prices: Vec<f64> = vec![120.0, 121.0, 122.0, 123.0, 124.0];
                prices
            }
        };
    return Ticker { symbol: symbol.into(),price: stockTimePrices[0], trend: stockTimePrices};
}