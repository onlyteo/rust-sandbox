use axum::response::Html;

pub async fn get_resources() -> Html<String> {
    let index_file = include_str!("../../resources/static/index.html");
    Html(index_file.to_string())
}
