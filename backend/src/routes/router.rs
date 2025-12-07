use crate::{app_state::AppState, routes::ticker_handler::get_ticker};
use axum::{routing::get, Router};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/tickers", get(get_ticker))
    // .route("/users", get(users_handler))
}
