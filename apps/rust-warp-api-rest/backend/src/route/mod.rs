mod api;
mod resources;

use std::sync::Arc;

use warp::{Filter, Reply};

use crate::route::api::api_filter;
use crate::route::resources::resources_filter;
use crate::state::AppState;
use error_handling::warp::middleware::error_handling;

pub fn routes(
    state: Arc<AppState>,
) -> impl Filter<Extract = (impl Reply,), Error = std::convert::Infallible> + Clone {
    resources_filter()
        .map(Reply::into_response)
        .or(api_filter(state).map(Reply::into_response))
        .unify()
        .recover(error_handling)
        .unify()
}
