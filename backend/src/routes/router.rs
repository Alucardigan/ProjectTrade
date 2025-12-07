use crate::routes::oms_handler::{
    cancel_order, get_order, get_order_status, get_orders, place_order,
};
use crate::routes::user_handler::register_user;
use crate::{app_state::AppState, routes::ticker_handler::get_ticker};
use axum::{routing::get, Router};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/tickers", get(get_ticker))
        .route("/orders", get(get_orders))
        .route("/place_order", get(place_order))
        .route("/cancel_order", get(cancel_order))
        .route("/register_user", get(register_user))
        .route("/order_status", get(get_order_status))
        .route("/order", get(get_order))
}
