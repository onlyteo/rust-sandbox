use std::collections::HashMap;
use std::sync::Arc;

use askama::Template;
use axum::Router;
use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::get;
use axum_extra::extract::cookie::CookieJar;
use uuid::Uuid;

use crate::model::{Alert, Cart, OrderStatus, User};
use crate::state::AppState;

pub fn order_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/orders", get(get_orders))
        .route("/orders/{id}", get(get_order).post(post_order_status))
}

#[derive(Template)]
#[template(path = "orders.html")]
struct OrdersTemplate {
    orders: Vec<OrderRow>,
    cart_count: i32,
    user: Arc<User>,
}

struct OrderRow {
    id: Uuid,
    status_label: &'static str,
    item_count: usize,
}

#[derive(Template)]
#[template(path = "order_detail.html")]
struct OrderDetailTemplate {
    id: Uuid,
    status_label: &'static str,
    lines: Vec<OrderLine>,
    alert: Option<Alert>,
    cart_count: i32,
    user: Arc<User>,
}

struct OrderLine {
    product_name: String,
    quantity: i32,
}

async fn get_orders(State(state): State<Arc<AppState>>, jar: CookieJar) -> Response {
    let user = state.user();
    let cart_count = Cart::from_jar(&jar).item_count();

    match state.orders_service().find_orders(user.username()).await {
        Ok(orders) => {
            let orders = orders
                .into_iter()
                .map(|order| OrderRow {
                    id: order.id,
                    status_label: status_label(order.status),
                    item_count: order.products.len(),
                })
                .collect();
            Html(
                OrdersTemplate {
                    orders,
                    cart_count,
                    user: user.clone(),
                }
                .render()
                .expect("failed to render orders template"),
            )
            .into_response()
        }
        Err(error) => {
            tracing::error!("Failed to fetch orders: {error}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load orders").into_response()
        }
    }
}

async fn get_order(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
) -> Response {
    render_order_detail(state, &jar, id, None).await
}

async fn post_order_status(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Path(id): Path<Uuid>,
    Form(params): Form<HashMap<String, String>>,
) -> Response {
    let user = state.user();

    let Some(status) = params.get("status").and_then(|value| parse_status(value)) else {
        let alert = Alert::danger("Invalid status.");
        return render_order_detail(state, &jar, id, Some(alert)).await;
    };

    let order = match state
        .orders_service()
        .get_order_by_id(user.username(), id)
        .await
    {
        Ok(optional_order) => match optional_order {
            Some(order) => order,
            None => {
                tracing::error!("Failed to fetch order {id}");
                return (StatusCode::NOT_FOUND, "Order not found").into_response();
            }
        },
        Err(error) => {
            tracing::error!("Failed to fetch order {id}: {error}");
            return (StatusCode::NOT_FOUND, "Order not found").into_response();
        }
    };

    match state
        .orders_service()
        .update_order(user.username(), id, status, order.products)
        .await
    {
        Ok(_) => Redirect::to(&format!("/orders/{id}")).into_response(),
        Err(error) => {
            tracing::error!("Failed to update order {id}: {error}");
            let alert = Alert::danger("Something went wrong, please try again.");
            render_order_detail(state, &jar, id, Some(alert)).await
        }
    }
}

async fn render_order_detail(
    state: Arc<AppState>,
    jar: &CookieJar,
    id: Uuid,
    alert: Option<Alert>,
) -> Response {
    let user = state.user();

    let order = match state
        .orders_service()
        .get_order_by_id(user.username(), id)
        .await
    {
        Ok(optional_order) => match optional_order {
            Some(order) => order,
            None => {
                tracing::error!("Failed to fetch order {id}");
                return (StatusCode::NOT_FOUND, "Order not found").into_response();
            }
        },
        Err(error) => {
            tracing::error!("Failed to fetch order {id}: {error}");
            return (StatusCode::NOT_FOUND, "Order not found").into_response();
        }
    };
    let products = state
        .product_service()
        .find_products(user.username())
        .await
        .unwrap_or_default();

    let lines = order
        .products
        .iter()
        .map(|line| {
            let product_name = products
                .iter()
                .find(|product| product.id == line.id)
                .map(|product| product.name.clone())
                .unwrap_or_else(|| line.id.to_string());
            OrderLine {
                product_name,
                quantity: line.quantity,
            }
        })
        .collect();

    Html(
        OrderDetailTemplate {
            id: order.id,
            status_label: status_label(order.status),
            lines,
            alert,
            cart_count: Cart::from_jar(jar).item_count(),
            user: user.clone(),
        }
        .render()
        .expect("failed to render order detail template"),
    )
    .into_response()
}

fn status_label(status: OrderStatus) -> &'static str {
    match status {
        OrderStatus::Open => "OPEN",
        OrderStatus::Completed => "COMPLETED",
        OrderStatus::Cancelled => "CANCELLED",
    }
}

fn parse_status(value: &str) -> Option<OrderStatus> {
    match value {
        "OPEN" => Some(OrderStatus::Open),
        "COMPLETED" => Some(OrderStatus::Completed),
        "CANCELLED" => Some(OrderStatus::Cancelled),
        _ => None,
    }
}
