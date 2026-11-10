use std::sync::Arc;

use auth_handling::basic::middleware::basic_auth_layer;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use uuid::Uuid;

use crate::error::AppError;
use crate::model::{CreateOrderRequest, Order, Product, UpdateOrderRequest};
use crate::state::AppState;

pub fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/products", get(get_products))
        .route("/api/orders", get(get_orders).post(post_order))
        .route("/api/orders/{id}", get(get_order).put(put_order))
        .route_layer(basic_auth_layer())
}

async fn get_products(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Product>>, AppError> {
    Ok(Json(state.product_service().find_products().await?))
}

async fn get_orders(State(state): State<Arc<AppState>>) -> Result<Json<Vec<Order>>, AppError> {
    Ok(Json(state.orders_service().find_orders().await?))
}

async fn get_order(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Order>, AppError> {
    match state.orders_service().get_order_by_id(id).await? {
        Some(order) => Ok(Json(order)),
        None => Err(AppError::order_not_found(id)),
    }
}

async fn post_order(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<Order>), AppError> {
    let optional_order = state.orders_service().create_order(request).await?;
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
    let optional_order = state.orders_service().update_order(id, request).await?;
    match optional_order {
        Some(order) => Ok(Json(order)),
        None => Err(AppError::order_not_found(id)),
    }
}
