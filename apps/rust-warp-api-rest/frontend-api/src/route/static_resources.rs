use warp::Filter;

const STATIC_RESOURCES_DIR: &str = "./resources/static";

pub fn static_resources_filter(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let index_file = format!("{}/index.html", STATIC_RESOURCES_DIR);
    let resources_path = warp::fs::dir(STATIC_RESOURCES_DIR);
    let root_path = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(index_file));
    resources_path.or(root_path)
}
