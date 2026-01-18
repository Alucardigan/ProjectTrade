use crate::app_state;
use crate::routes::account_handler::{
    add_to_user_balance, get_account_balance, get_transaction_history, withdraw_funds,
};
use crate::routes::health::health;
use crate::routes::loan_handler::get_loan;
use crate::routes::middleware::auth0_middleware;
use crate::routes::oms_handler::{
    cancel_order, get_order, get_order_status, get_pending_orders, place_order,
};
use crate::routes::portfolio_handler::get_portfolio;
use crate::routes::user_handler::{auth0_callback, login_user};
use crate::{app_state::AppState, routes::ticker_handler::get_ticker};
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, post};
use axum::{routing::get, Router};

pub fn create_router(app_state: AppState) -> Router<AppState> {
    let public_routes = Router::new()
        .route("/tickers/:symbol", get(get_ticker))
        .route("/auth/login", post(login_user))
        .route("/health", get(health))
        .route("/auth/callback", get(auth0_callback));
    let private_routes = Router::new()
        .route("/portfolio", get(get_portfolio))
        .route("/account", get(get_account_balance))
        .route("/account/withdrawals", post(withdraw_funds))
        .route("/account/deposits", post(add_to_user_balance))
        .route("/account/transactions", get(get_transaction_history))
        .route("/orders", get(get_pending_orders))
        .route("/orders/:order_id", get(get_order))
        .route("/orders", post(place_order))
        .route("/orders/:order_id", delete(cancel_order))
        .route("/loans/:loan_type", post(get_loan))
        .layer(from_fn_with_state(app_state, auth0_middleware));
    Router::new().merge(public_routes).merge(private_routes)
}
