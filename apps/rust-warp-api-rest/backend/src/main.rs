use rust_warp_rest_api_backend::route;

#[tokio::main]
async fn main() {
    let routes = route::routes();
    println!("Server started at http://localhost:8081");
    warp::serve(routes).run(([127, 0, 0, 1], 8081)).await;
}
