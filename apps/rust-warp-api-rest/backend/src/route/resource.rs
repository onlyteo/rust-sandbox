use warp::Filter;

const STATIC_RESOURCES_PATH: &str = "./resources/static";

pub fn get_resources() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    warp::get()
        .and(warp::fs::dir(STATIC_RESOURCES_PATH))
}