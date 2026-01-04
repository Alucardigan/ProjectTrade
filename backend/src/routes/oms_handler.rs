use std::fmt::Debug;

use crate::app_state::AppState;
use crate::models::errors::api_error::ApiError;
use crate::models::order::OrderStatus;
use crate::models::order::{Order, OrderType};
use axum::Extension;
use axum::{
    extract::{Path, State},
    Json,
};
use bigdecimal::BigDecimal;
use serde::Deserialize;
use uuid::Uuid;

//request and response bodies
#[derive(Deserialize)]
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub quantity: BigDecimal,
    pub order_type: OrderType, // Assuming OrderType also implements Deserialize
    pub price_buffer: BigDecimal,
}
impl Debug for PlaceOrderRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlaceOrderRequest")
            .field("symbol", &self.symbol)
            .field("quantity", &self.quantity)
            .field("order_type", &self.order_type)
            .field("price_buffer", &self.price_buffer)
            .finish()
    }
}

//getters
pub async fn get_order_status(
    State(app_state): State<AppState>,
    Path(order_id): Path<Uuid>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<OrderStatus>, ApiError> {
    let order_status = app_state
        .order_management_service
        .get_order_status(order_id, user_id)
        .await?;
    Ok(Json(order_status))
}
#[tracing::instrument(skip(app_state))]
pub async fn get_orders(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Vec<Order>>, ApiError> {
    let orders = app_state
        .order_management_service
        .get_orders(user_id)
        .await?;
    Ok(Json(orders))
}
#[tracing::instrument(skip(app_state))]
pub async fn get_order(
    State(app_state): State<AppState>,
    Path(order_id): Path<Uuid>,
    Extension(user_id): Extension<Uuid>,
) -> Result<Json<Order>, ApiError> {
    let order = app_state
        .order_management_service
        .get_order(order_id, user_id)
        .await?;
    Ok(Json(order))
}
// posters
#[tracing::instrument(skip(app_state))]
pub async fn place_order(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Json(request_body): Json<PlaceOrderRequest>,
) -> Result<Json<Order>, ApiError> {
    let order = app_state
        .order_management_service
        .place_order(
            user_id,
            &request_body.symbol,
            request_body.quantity,
            request_body.order_type,
            request_body.price_buffer,
        )
        .await?;
    Ok(Json(order))
}

#[tracing::instrument(skip(app_state))]
pub async fn cancel_order(
    State(app_state): State<AppState>,
    Extension(user_id): Extension<Uuid>,
    Path(order_id): Path<Uuid>,
) -> Result<(), ApiError> {
    app_state
        .order_management_service
        .cancel_order(order_id, user_id)
        .await?;
    Ok(())
}
