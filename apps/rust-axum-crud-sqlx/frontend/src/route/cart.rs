use std::sync::Arc;

use askama::Template;
use axum::Router;
use axum::extract::{Form, State};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use uuid::Uuid;

use crate::model::{Alert, Cart, CartSummary, User};
use crate::state::AppState;

pub fn cart_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/cart", get(get_cart))
        .route("/cart/add", post(post_add))
        .route("/cart/update", post(post_update))
        .route("/cart/remove", post(post_remove))
        .route("/cart/clear", post(post_clear))
        .route("/cart/checkout", post(post_checkout))
}

#[derive(Template)]
#[template(path = "cart.html")]
struct CartTemplate {
    cart: CartSummary,
    cart_count: i32,
    user: Arc<User>,
    alert: Option<Alert>,
}

#[derive(Deserialize)]
struct QuantityForm {
    product_id: Uuid,
    quantity: i32,
}

#[derive(Deserialize)]
struct ProductForm {
    product_id: Uuid,
}

async fn get_cart(State(state): State<Arc<AppState>>, jar: CookieJar) -> Response {
    render_cart(&state, &jar, None).await
}

async fn post_add(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<QuantityForm>,
) -> Response {
    update_quantity(&state, jar, form, true, "/").await
}

async fn post_update(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Form(form): Form<QuantityForm>,
) -> Response {
    update_quantity(&state, jar, form, false, "/cart").await
}

async fn post_remove(jar: CookieJar, Form(form): Form<ProductForm>) -> Response {
    let mut cart = Cart::from_jar(&jar);
    cart.remove(&form.product_id);
    (jar.add(cart.to_cookie()), Redirect::to("/cart")).into_response()
}

async fn post_clear(jar: CookieJar) -> Response {
    (jar.add(Cart::default().to_cookie()), Redirect::to("/cart")).into_response()
}

async fn post_checkout(State(state): State<Arc<AppState>>, jar: CookieJar) -> Response {
    let user = state.user();

    let cart = Cart::from_jar(&jar);
    if cart.is_empty() {
        return render_cart(&state, &jar, Some(Alert::danger("Your cart is empty."))).await;
    }

    match state
        .orders_service()
        .create_order(user.username(), cart.to_order_products())
        .await
    {
        Ok(Some(order)) => (
            jar.add(Cart::default().to_cookie()),
            Redirect::to(&format!("/orders/{}", order.id)),
        )
            .into_response(),
        Ok(None) => {
            tracing::error!("Failed to create order");
            render_cart(
                &state,
                &jar,
                Some(Alert::danger("Something went wrong, please try again.")),
            )
            .await
        }
        Err(error) => {
            tracing::error!("Failed to create order: {error}");
            render_cart(
                &state,
                &jar,
                Some(Alert::danger("Something went wrong, please try again.")),
            )
            .await
        }
    }
}

/// Adds to (`add`) or sets the cart quantity for a product, capped at stock, then redirects.
async fn update_quantity(
    state: &AppState,
    jar: CookieJar,
    form: QuantityForm,
    add: bool,
    redirect_to: &str,
) -> Response {
    let user = state.user();
    let products = match state.product_service().find_products(user.username()).await {
        Ok(products) => products,
        Err(error) => {
            tracing::error!("Failed to fetch products: {error}");
            return render_cart(state, &jar, Some(Alert::danger("Failed to update cart."))).await;
        }
    };
    let Some(product) = products
        .iter()
        .find(|product| product.id == form.product_id)
    else {
        return Redirect::to(redirect_to).into_response();
    };

    let mut cart = Cart::from_jar(&jar);
    if add {
        cart.add(form.product_id, form.quantity, product.stock);
    } else {
        cart.set(form.product_id, form.quantity, product.stock);
    }
    (jar.add(cart.to_cookie()), Redirect::to(redirect_to)).into_response()
}

async fn render_cart(state: &AppState, jar: &CookieJar, alert: Option<Alert>) -> Response {
    let user = state.user();
    let cart = Cart::from_jar(jar);
    let cart_count = cart.item_count();

    let template = match state.product_service().find_products(user.username()).await {
        Ok(products) => CartTemplate {
            cart: CartSummary::build(&products, &cart),
            cart_count,
            user: state.user().clone(),
            alert,
        },
        Err(error) => {
            tracing::error!("Failed to fetch products: {error}");
            CartTemplate {
                cart: CartSummary::empty(),
                cart_count,
                user,
                alert: Some(Alert::danger("Failed to load cart.")),
            }
        }
    };

    Html(template.render().expect("failed to render cart template")).into_response()
}
