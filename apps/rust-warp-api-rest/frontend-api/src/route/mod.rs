mod api;
mod resource;

use crate::route::api::post_greetings;
use crate::route::resource::get_resources;
use warp::Filter;

pub fn routes() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    post_greetings()
        .or(get_resources())
}