use crate::app_state;
use crate::routes::health::health;
use crate::routes::middleware::auth0_middleware;
use crate::routes::oms_handler::{
    cancel_order, get_order, get_order_status, get_orders, place_order,
};
use crate::routes::user_handler::{auth0_callback, login_user};
use crate::{app_state::AppState, routes::ticker_handler::get_ticker};
use axum::middleware::from_fn_with_state;
use axum::routing::post;
use axum::{routing::get, Router};

pub fn create_router(app_state: AppState) -> Router<AppState> {
    let public_routes = Router::new()
        .route("/tickers/:symbol", get(get_ticker))
        .route("/login_user", post(login_user))
        .route("/health", get(health))
        .route("/auth_callback", get(auth0_callback));
    let private_routes = Router::new()
        .route("/orders", get(get_orders))
        .route("/place_order", post(place_order))
        .route("/cancel_order/:order_id", post(cancel_order))
        .route("/order_status/:order_id", get(get_order_status))
        .route("/order/:order_id", get(get_order));
    Router::new()
        .merge(public_routes)
        .merge(private_routes)
        .layer(from_fn_with_state(app_state, auth0_middleware))
}
