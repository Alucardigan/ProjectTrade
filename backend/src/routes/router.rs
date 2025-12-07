use crate::routes::oms_handler::{
    cancel_order, get_order, get_order_status, get_orders, place_order,
};
use crate::routes::user_handler::register_user;
use crate::{app_state::AppState, routes::ticker_handler::get_ticker};
use axum::routing::post;
use axum::{routing::get, Router};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/tickers/:symbol", get(get_ticker))
        .route("/orders/:user_id", get(get_orders))
        .route("/place_order/:user_id", post(place_order))
        .route("/cancel_order/:order_id", post(cancel_order))
        .route("/register_user", post(register_user))
        .route("/order_status/:order_id", get(get_order_status))
        .route("/order/:order_id", get(get_order))
}
