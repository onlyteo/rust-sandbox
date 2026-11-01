use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use uuid::Uuid;

use crate::error::AppError;
use crate::model::{CreateOrderRequest, Order, Product, UpdateOrderRequest};
use crate::service;
use crate::state::AppState;

pub fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/products", get(get_products))
        .route("/api/orders", get(get_orders).post(post_order))
        .route("/api/orders/{id}", get(get_order_by_id).put(put_order))
}

async fn get_products(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Product>>, AppError> {
    match service::product::find_products(state.pool()).await {
        Ok(products) => Ok(Json(products)),
        Err(e) => Err(AppError::from(e)),
    }
}

async fn get_orders(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Order>>, AppError> {
    match service::order::find_orders(state.pool()).await {
        Ok(orders) => Ok(Json(orders)),
        Err(e) => Err(AppError::from(e)),
    }
}

async fn get_order_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Order>, AppError> {
    match service::order::get_order_by_id(state.pool(), id).await {
        Ok(optional_order) => match optional_order {
            Some(order) => Ok(Json(order)),
            None => Err(AppError::order_not_found(id)),
        },
        Err(e) => Err(AppError::from(e)),
    }
}

async fn post_order(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<Order>), AppError> {
    let optional_order = service::order::create_order(state.pool(), request).await?;
    match optional_order {
        Some(order) => Ok((StatusCode::CREATED, Json(order))),
        None => Err(AppError::create_order_failed()),
    }
}

async fn put_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateOrderRequest>,
) -> Result<Json<Order>, AppError> {
    let optional_order = service::order::update_order(state.pool(), id, request).await?;
    match optional_order {
        Some(order) => Ok(Json(order)),
        None => Err(AppError::order_not_found(id)),
    }
}
