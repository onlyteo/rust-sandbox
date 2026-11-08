use std::sync::Arc;

use askama::Template;
use axum::Router;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum_extra::extract::cookie::CookieJar;

use crate::model::{Alert, BootstrapPrincipal, Cart, CartSummary, Product};
use crate::state::AppState;

pub fn home_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_home))
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    products: Vec<Product>,
    alert: Option<Alert>,
    cart: CartSummary,
    cart_count: i32,
    principal: Arc<BootstrapPrincipal>,
}

async fn get_home(State(state): State<Arc<AppState>>, jar: CookieJar) -> Response {
    let cart = Cart::from_jar(&jar);
    let cart_count = cart.item_count();

    let template = match state
        .product_service()
        .find_products(state.principal())
        .await
    {
        Ok(products) => IndexTemplate {
            cart: CartSummary::build(&products, &cart),
            products,
            alert: None,
            cart_count,
            principal: state.principal(),
        },
        Err(error) => {
            tracing::error!("Failed to fetch products: {error}");
            IndexTemplate {
                products: Vec::new(),
                alert: Some(Alert::danger("Failed to load products.")),
                cart: CartSummary::empty(),
                cart_count,
                principal: state.principal(),
            }
        }
    };

    Html(template.render().expect("failed to render index template")).into_response()
}
