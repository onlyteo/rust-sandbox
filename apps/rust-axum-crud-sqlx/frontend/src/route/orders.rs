use std::sync::Arc;

use askama::Template;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use uuid::Uuid;

use crate::model::{Alert, BootstrapPrincipal, Cart, Order, OrderStatus, format_euro};
use crate::state::AppState;

pub fn order_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/orders", get(get_orders))
        .route("/orders/{id}", get(get_order))
}

#[derive(Template)]
#[template(path = "orders.html")]
struct OrdersTemplate {
    orders: Vec<OrderRow>,
    cart_count: i32,
    principal: Arc<BootstrapPrincipal>,
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
    principal: Arc<BootstrapPrincipal>,
}

struct OrderLine {
    product_name: String,
    packing: String,
    description: String,
    price: String,
    line_total: String,
    quantity: i32,
}

async fn get_orders(State(state): State<Arc<AppState>>) -> Response {
    match state.orders_service().find_orders(state.principal()).await {
        Ok(orders) => {
            let cart_count = cart_count(&orders);
            let orders = orders
                .into_iter()
                .filter(|order| order.status != OrderStatus::Open)
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
                    principal: state.principal(),
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

async fn get_order(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Response {
    render_order_detail(state, id, None).await
}

async fn render_order_detail(state: Arc<AppState>, id: Uuid, alert: Option<Alert>) -> Response {
    let order = match state
        .orders_service()
        .get_order_by_id(state.principal(), id)
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
    if order.status == OrderStatus::Open {
        return (StatusCode::NOT_FOUND, "Order not found").into_response();
    }
    let cart_count = state
        .orders_service()
        .find_open_order(state.principal())
        .await
        .map(|open| Cart::from_order(open.as_ref()).item_count())
        .unwrap_or_default();
    let products = state
        .product_service()
        .find_products(state.principal())
        .await
        .unwrap_or_default();

    let lines = order
        .products
        .iter()
        .map(
            |line| match products.iter().find(|product| product.id == line.id) {
                Some(product) => OrderLine {
                    product_name: product.name.clone(),
                    packing: product.packing.clone(),
                    description: product.description.clone(),
                    price: format_euro(i64::from(product.price)),
                    line_total: format_euro(i64::from(product.price) * i64::from(line.quantity)),
                    quantity: line.quantity,
                },
                None => OrderLine {
                    product_name: line.id.to_string(),
                    packing: String::new(),
                    description: String::new(),
                    price: String::new(),
                    line_total: String::new(),
                    quantity: line.quantity,
                },
            },
        )
        .collect();

    Html(
        OrderDetailTemplate {
            id: order.id,
            status_label: status_label(order.status),
            lines,
            alert,
            cart_count,
            principal: state.principal(),
        }
        .render()
        .expect("failed to render order detail template"),
    )
    .into_response()
}

fn cart_count(orders: &[Order]) -> i32 {
    let open = orders
        .iter()
        .find(|order| order.status == OrderStatus::Open);
    Cart::from_order(open).item_count()
}

fn status_label(status: OrderStatus) -> &'static str {
    match status {
        OrderStatus::Open => "OPEN",
        OrderStatus::Completed => "COMPLETED",
        OrderStatus::Cancelled => "CANCELLED",
    }
}
