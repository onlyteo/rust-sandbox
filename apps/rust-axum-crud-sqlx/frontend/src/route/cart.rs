use std::sync::Arc;

use askama::Template;
use axum::Router;
use axum::extract::{Form, State};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::{get, post};
use serde::Deserialize;
use uuid::Uuid;

use crate::model::{Alert, BootstrapPrincipal, Cart, CartSummary, Order, OrderStatus};
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
    principal: Arc<BootstrapPrincipal>,
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

async fn get_cart(State(state): State<Arc<AppState>>) -> Response {
    render_cart(&state, None).await
}

async fn post_add(State(state): State<Arc<AppState>>, Form(form): Form<QuantityForm>) -> Response {
    update_quantity(&state, form, true, "/").await
}

async fn post_update(
    State(state): State<Arc<AppState>>,
    Form(form): Form<QuantityForm>,
) -> Response {
    update_quantity(&state, form, false, "/cart").await
}

async fn post_remove(
    State(state): State<Arc<AppState>>,
    Form(form): Form<ProductForm>,
) -> Response {
    let (order, mut cart) = match load_cart(&state).await {
        Ok(loaded) => loaded,
        Err(response) => return *response,
    };
    cart.remove(&form.product_id);
    save_and_redirect(&state, order.as_ref(), &cart, "/cart").await
}

async fn post_clear(State(state): State<Arc<AppState>>) -> Response {
    let (order, _) = match load_cart(&state).await {
        Ok(loaded) => loaded,
        Err(response) => return *response,
    };
    save_and_redirect(&state, order.as_ref(), &Cart::default(), "/cart").await
}

async fn post_checkout(State(state): State<Arc<AppState>>) -> Response {
    let (order, cart) = match load_cart(&state).await {
        Ok(loaded) => loaded,
        Err(response) => return *response,
    };
    let Some(order) = order.filter(|_| !cart.is_empty()) else {
        return render_cart(&state, Some(Alert::danger("Your cart is empty."))).await;
    };

    match state
        .orders_service()
        .update_order(
            state.principal(),
            order.id,
            OrderStatus::Completed,
            cart.to_order_products(),
        )
        .await
    {
        Ok(_) => Redirect::to(&format!("/orders/{}", order.id)).into_response(),
        Err(error) => {
            tracing::error!("Failed to complete order {}: {error}", order.id);
            render_cart(
                &state,
                Some(Alert::danger("Something went wrong, please try again.")),
            )
            .await
        }
    }
}

/// Loads the open order and the cart derived from it. On failure returns a ready response.
async fn load_cart(state: &AppState) -> Result<(Option<Order>, Cart), Box<Response>> {
    match state
        .orders_service()
        .find_open_order(state.principal())
        .await
    {
        Ok(order) => {
            let cart = Cart::from_order(order.as_ref());
            Ok((order, cart))
        }
        Err(error) => {
            tracing::error!("Failed to fetch open order: {error}");
            Err(Box::new(
                render_cart(state, Some(Alert::danger("Failed to load cart."))).await,
            ))
        }
    }
}

async fn save_and_redirect(
    state: &AppState,
    order: Option<&Order>,
    cart: &Cart,
    redirect_to: &str,
) -> Response {
    match state
        .orders_service()
        .save_cart(state.principal(), order, cart)
        .await
    {
        Ok(()) => Redirect::to(redirect_to).into_response(),
        Err(error) => {
            tracing::error!("Failed to save cart: {error}");
            render_cart(state, Some(Alert::danger("Failed to update cart."))).await
        }
    }
}

/// Adds to (`add`) or sets the cart quantity for a product, capped at stock, then redirects.
async fn update_quantity(
    state: &AppState,
    form: QuantityForm,
    add: bool,
    redirect_to: &str,
) -> Response {
    let products = match state
        .product_service()
        .find_products(state.principal())
        .await
    {
        Ok(products) => products,
        Err(error) => {
            tracing::error!("Failed to fetch products: {error}");
            return render_cart(state, Some(Alert::danger("Failed to update cart."))).await;
        }
    };
    let Some(product) = products
        .iter()
        .find(|product| product.id == form.product_id)
    else {
        return Redirect::to(redirect_to).into_response();
    };

    let (order, mut cart) = match load_cart(state).await {
        Ok(loaded) => loaded,
        Err(response) => return *response,
    };
    if add {
        cart.add(form.product_id, form.quantity, product.stock);
    } else {
        cart.set(form.product_id, form.quantity, product.stock);
    }
    save_and_redirect(state, order.as_ref(), &cart, redirect_to).await
}

async fn render_cart(state: &AppState, alert: Option<Alert>) -> Response {
    let (cart, load_error) = match state
        .orders_service()
        .find_open_order(state.principal())
        .await
    {
        Ok(order) => (Cart::from_order(order.as_ref()), false),
        Err(error) => {
            tracing::error!("Failed to fetch open order: {error}");
            (Cart::default(), true)
        }
    };
    let cart_count = cart.item_count();

    let template = match state
        .product_service()
        .find_products(state.principal())
        .await
    {
        Ok(products) => CartTemplate {
            cart: CartSummary::build(&products, &cart),
            cart_count,
            principal: state.principal(),
            alert: if load_error {
                Some(Alert::danger("Failed to load cart."))
            } else {
                alert
            },
        },
        Err(error) => {
            tracing::error!("Failed to fetch products: {error}");
            CartTemplate {
                cart: CartSummary::empty(),
                cart_count,
                principal: state.principal(),
                alert: Some(Alert::danger("Failed to load cart.")),
            }
        }
    };

    Html(template.render().expect("failed to render cart template")).into_response()
}
