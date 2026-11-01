use std::collections::HashMap;
use std::sync::Arc;

use askama::Template;
use axum::Router;
use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::get;
use uuid::Uuid;

use crate::model::OrderStatus;
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
}

struct OrderLine {
    product_name: String,
    quantity: i32,
}

struct Alert {
    variant: &'static str,
    message: String,
}

async fn get_orders(State(state): State<Arc<AppState>>) -> Response {
    match state.orders_service().find_orders().await {
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
                OrdersTemplate { orders }
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

async fn post_order_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Form(params): Form<HashMap<String, String>>,
) -> Response {
    let Some(status) = params.get("status").and_then(|value| parse_status(value)) else {
        let alert = Alert {
            variant: "danger",
            message: "Invalid status.".to_string(),
        };
        return render_order_detail(state, id, Some(alert)).await;
    };

    let order = match state.orders_service().get_order_by_id(id).await {
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
        .update_order(id, status, order.products)
        .await
    {
        Ok(_) => Redirect::to(&format!("/orders/{id}")).into_response(),
        Err(error) => {
            tracing::error!("Failed to update order {id}: {error}");
            let alert = Alert {
                variant: "danger",
                message: "Something went wrong, please try again.".to_string(),
            };
            render_order_detail(state, id, Some(alert)).await
        }
    }
}

async fn render_order_detail(state: Arc<AppState>, id: Uuid, alert: Option<Alert>) -> Response {
    let order = match state.orders_service().get_order_by_id(id).await {
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
        .find_products()
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
