use std::net::SocketAddr;

use observability::init_observability;
use rust_warp_rest_api_backend::config::{load_observability_config, load_server_config};
use rust_warp_rest_api_backend::repository::order::OrderRepository;
use rust_warp_rest_api_backend::repository::product::ProductRepository;
use rust_warp_rest_api_backend::route;
use rust_warp_rest_api_backend::service::order::OrderService;
use rust_warp_rest_api_backend::service::product::ProductService;
use rust_warp_rest_api_backend::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_config = load_server_config()?;
    let observability_config = load_observability_config()?;

    init_observability(&observability_config)?;

    let order_service = OrderService::new(OrderRepository::new());
    let product_service = ProductService::new(ProductRepository::new());

    let state = AppState::new(order_service, product_service);
    let routes = route::routes(state);

    tracing::info!(
        "Starting webserver at address {}",
        server_config.to_string()
    );
    let address: SocketAddr = server_config.into();
    warp::serve(routes).run(address).await;

    Ok(())
}
