mod cart;
mod home;
mod orders;
mod resources;

use std::sync::Arc;

use axum::Router;

use crate::route::cart::cart_routes;
use crate::route::home::home_routes;
use crate::route::orders::order_routes;
use crate::route::resources::resource_routes;
use crate::state::AppState;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(resource_routes())
        .merge(home_routes())
        .merge(cart_routes())
        .merge(order_routes())
        .with_state(state)
}
