mod model;
mod route;
mod client;

#[tokio::main]
async fn main() {
    let routes = route::routes();
    println!("Server started at http://localhost:8080");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
