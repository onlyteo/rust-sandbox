use std::sync::Arc;

use auth_handling::axum::basic::middleware::basic_auth;
use auth_handling::model::BasicAuthPrincipal;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Json, Router};
use uuid::Uuid;

use crate::error::AppError;
use crate::model::{CreateOrderRequest, Order, Product, UpdateOrderRequest};
use crate::state::AppState;

pub fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/products", get(get_products))
        .route("/api/orders", get(get_orders).post(post_order))
        .route("/api/orders/{id}", get(get_order).put(put_order))
        .route_layer(basic_auth())
}

async fn get_products(
    State(state): State<Arc<AppState>>,
    Extension(_principal): Extension<BasicAuthPrincipal>,
) -> Result<Json<Vec<Product>>, AppError> {
    Ok(Json(state.product_service().find_products().await?))
}

async fn get_orders(
    State(state): State<Arc<AppState>>,
    Extension(principal): Extension<BasicAuthPrincipal>,
) -> Result<Json<Vec<Order>>, AppError> {
    let username = principal.username;
    Ok(Json(state.orders_service().find_orders(&username).await?))
}

async fn get_order(
    State(state): State<Arc<AppState>>,
    Extension(principal): Extension<BasicAuthPrincipal>,
    Path(id): Path<Uuid>,
) -> Result<Json<Order>, AppError> {
    let username = principal.username;
    match state
        .orders_service()
        .get_order_by_id(&id, &username)
        .await?
    {
        Some(order) => Ok(Json(order)),
        None => Err(AppError::order_not_found(id)),
    }
}

async fn post_order(
    State(state): State<Arc<AppState>>,
    Extension(principal): Extension<BasicAuthPrincipal>,
    Json(request): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<Order>), AppError> {
    let username = principal.username;
    let order = state
        .orders_service()
        .create_order(&username, &request)
        .await?;
    Ok((StatusCode::CREATED, Json(order)))
}

async fn put_order(
    State(state): State<Arc<AppState>>,
    Extension(principal): Extension<BasicAuthPrincipal>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateOrderRequest>,
) -> Result<Json<Order>, AppError> {
    let username = principal.username;
    let optional_order = state
        .orders_service()
        .update_order(&id, &username, &request)
        .await?;
    match optional_order {
        Some(order) => Ok(Json(order)),
        None => Err(AppError::order_not_found(id)),
    }
}
