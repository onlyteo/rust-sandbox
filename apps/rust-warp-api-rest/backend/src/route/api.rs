use std::sync::Arc;

use crate::error::AppError;
use crate::model::{CreateOrderRequest, UpdateOrderRequest};
use crate::state::AppState;
use auth_handling::model::BasicAuthPrincipal;
use auth_handling::warp::basic::middleware::basic_auth;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

fn with_state(
    state: Arc<AppState>,
) -> impl Filter<Extract = (Arc<AppState>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

/// All endpoints are protected with Basic auth and scoped to the authenticated user.
pub fn api_filter(
    state: Arc<AppState>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let get_products = warp::path!("api" / "products")
        .and(warp::get())
        .and(basic_auth())
        .and(with_state(state.clone()))
        .and_then(get_products);

    let get_orders = warp::path!("api" / "orders")
        .and(warp::get())
        .and(basic_auth())
        .and(with_state(state.clone()))
        .and_then(get_orders);

    let get_order = warp::path!("api" / "orders" / Uuid)
        .and(warp::get())
        .and(basic_auth())
        .and(with_state(state.clone()))
        .and_then(get_order);

    let post_order = warp::path!("api" / "orders")
        .and(warp::post())
        .and(basic_auth())
        .and(with_state(state.clone()))
        .and(warp::body::json())
        .and_then(post_order);

    let put_order = warp::path!("api" / "orders" / Uuid)
        .and(warp::put())
        .and(basic_auth())
        .and(with_state(state))
        .and(warp::body::json())
        .and_then(put_order);

    get_products
        .or(get_orders)
        .or(get_order)
        .or(post_order)
        .or(put_order)
}

async fn get_products(
    _principal: BasicAuthPrincipal,
    instance: String,
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    let products = state
        .product_service()
        .find_products()
        .await
        .map_err(|e| AppError::from(e).reject(&instance))?;
    Ok(warp::reply::json(&products))
}

async fn get_orders(
    principal: BasicAuthPrincipal,
    instance: String,
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    let orders = state
        .orders_service()
        .find_orders(&principal.username)
        .await
        .map_err(|e| AppError::from(e).reject(&instance))?;
    Ok(warp::reply::json(&orders))
}

async fn get_order(
    id: Uuid,
    principal: BasicAuthPrincipal,
    instance: String,
    state: Arc<AppState>,
) -> Result<impl Reply, Rejection> {
    let optional_order = state
        .orders_service()
        .get_order_by_id(&id, &principal.username)
        .await
        .map_err(|e| AppError::from(e).reject(&instance))?;
    match optional_order {
        Some(order) => Ok(warp::reply::json(&order)),
        None => Err(AppError::order_not_found(id).reject(&instance)),
    }
}

async fn post_order(
    principal: BasicAuthPrincipal,
    instance: String,
    state: Arc<AppState>,
    request: CreateOrderRequest,
) -> Result<impl Reply, Rejection> {
    let order = state
        .orders_service()
        .create_order(&principal.username, &request)
        .await
        .map_err(|e| AppError::from(e).reject(&instance))?;
    Ok(warp::reply::with_status(
        warp::reply::json(&order),
        StatusCode::CREATED,
    ))
}

async fn put_order(
    id: Uuid,
    principal: BasicAuthPrincipal,
    instance: String,
    state: Arc<AppState>,
    request: UpdateOrderRequest,
) -> Result<impl Reply, Rejection> {
    let optional_order = state
        .orders_service()
        .update_order(&id, &principal.username, &request)
        .await
        .map_err(|e| AppError::from(e).reject(&instance))?;
    match optional_order {
        Some(order) => Ok(warp::reply::json(&order)),
        None => Err(AppError::order_not_found(id).reject(&instance)),
    }
}
