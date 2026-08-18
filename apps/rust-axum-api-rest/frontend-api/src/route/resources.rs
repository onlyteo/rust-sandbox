use axum::response::Html;
use axum::routing::get;
use axum::Router;

pub fn resource_routes() -> Router {
    Router::new().route("/", get(get_resources))
}

async fn get_resources() -> Html<String> {
    let index_file = include_str!("../../resources/static/index.html");
    Html(index_file.to_string())
}
