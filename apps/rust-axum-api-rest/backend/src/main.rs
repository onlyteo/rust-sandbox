mod model;
mod route;

#[tokio::main]
async fn main() {
    let routes = route::routes();
    println!("Server started at http://localhost:8081");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, routes).await.unwrap();
}
