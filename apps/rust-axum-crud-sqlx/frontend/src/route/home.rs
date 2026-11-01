use std::collections::HashMap;
use std::sync::Arc;

use askama::Template;
use axum::Router;
use axum::extract::{Form, State};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::get;
use uuid::Uuid;

use crate::model::{OrderProduct, Product};
use crate::state::AppState;

pub fn home_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_home).post(post_home))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    products: Vec<Product>,
    alert: Option<Alert>,
}

struct Alert {
    variant: &'static str,
    message: String,
}

async fn get_home(State(state): State<Arc<AppState>>) -> Response {
    render(state, None).await
}

async fn post_home(
    State(state): State<Arc<AppState>>,
    Form(params): Form<HashMap<String, String>>,
) -> Response {
    let selected_products = selected_products(&params);

    if selected_products.is_empty() {
        tracing::warn!("Rejected order request with no selected products");
        let alert = Alert {
            variant: "danger",
            message: "Select at least one product.".to_string(),
        };
        return render(state, Some(alert)).await;
    }

    match state
        .http_client()
        .create_order(state.user().username.clone(), selected_products)
        .await
    {
        Ok(order) => Redirect::to(&format!("/orders/{}", order.id)).into_response(),
        Err(error) => {
            tracing::error!("Failed to create order: {error}");
            let alert = Alert {
                variant: "danger",
                message: "Something went wrong, please try again.".to_string(),
            };
            render(state, Some(alert)).await
        }
    }
}

fn selected_products(params: &HashMap<String, String>) -> Vec<OrderProduct> {
    params
        .iter()
        .filter_map(|(key, value)| {
            let id = key.strip_prefix("quantity_")?;
            let id = Uuid::parse_str(id).ok()?;
            let quantity: i32 = value.parse().ok()?;
            (quantity > 0).then_some(OrderProduct { id, quantity })
        })
        .collect()
}

async fn render(state: Arc<AppState>, alert: Option<Alert>) -> Response {
    match state.http_client().list_products().await {
        Ok(products) => Html(
            IndexTemplate { products, alert }
                .render()
                .expect("failed to render index template"),
        )
        .into_response(),
        Err(error) => {
            tracing::error!("Failed to fetch products: {error}");
            Html(
                IndexTemplate {
                    products: Vec::new(),
                    alert: Some(Alert {
                        variant: "danger",
                        message: "Failed to load products.".to_string(),
                    }),
                }
                .render()
                .expect("failed to render index template"),
            )
            .into_response()
        }
    }
}
