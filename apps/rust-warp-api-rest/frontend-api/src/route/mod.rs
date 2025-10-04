mod api;
mod resources;

use crate::route::api::api_filter;
use crate::route::resources::resources_filter;
use warp::Filter;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    api_filter().or(resources_filter())
}
