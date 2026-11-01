mod api;
mod resources;

use std::sync::Arc;

use axum::Router;

use crate::route::api::api_routes;
use crate::route::resources::resource_routes;
use crate::state::AppState;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(resource_routes())
        .merge(api_routes())
        .with_state(state)
}
