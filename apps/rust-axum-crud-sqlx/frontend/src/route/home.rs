use std::sync::Arc;

use askama::Template;
use axum::Router;
use axum::extract::State;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum_extra::extract::cookie::CookieJar;

use crate::model::{Alert, Cart, CartSummary, Product, User};
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
    user: Arc<User>,
}

async fn get_home(State(state): State<Arc<AppState>>, jar: CookieJar) -> Response {
    let user = state.user();
    let cart = Cart::from_jar(&jar);
    let cart_count = cart.item_count();

    let template = match state
        .product_service()
        .find_products(user.username.clone())
        .await
    {
        Ok(products) => IndexTemplate {
            cart: CartSummary::build(&products, &cart),
            products,
            alert: None,
            cart_count,
            user: user.clone(),
        },
        Err(error) => {
            tracing::error!("Failed to fetch products: {error}");
            IndexTemplate {
                products: Vec::new(),
                alert: Some(Alert::danger("Failed to load products.")),
                cart: CartSummary::empty(),
                cart_count,
                user: user.clone(),
            }
        }
    };

    Html(template.render().expect("failed to render index template")).into_response()
}
