use observability::init_observability;
use rust_axum_rest_api_backend::config::{load_observability_config, load_server_config};
use rust_axum_rest_api_backend::repository::order::OrderRepository;
use rust_axum_rest_api_backend::repository::product::ProductRepository;
use rust_axum_rest_api_backend::route;
use rust_axum_rest_api_backend::service::order::OrderService;
use rust_axum_rest_api_backend::service::product::ProductService;
use rust_axum_rest_api_backend::state::AppState;
use server::axum::axum_server;
use server::task::{self, shutdown_signal};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_config = load_server_config()?;
    let observability_config = load_observability_config()?;

    init_observability(&observability_config)?;

    let order_service = OrderService::new(OrderRepository::new());
    let product_service = ProductService::new(ProductRepository::new());

    let state = AppState::new(order_service, product_service);
    let routes = route::routes(state);
    let server_task = axum_server(routes, server_config).await;
    let signal_task = shutdown_signal();

    tokio::select! {
        result = server_task => task::run("Webserver", result),
        result = signal_task => task::shutdown(result),
    }?;
    Ok(())
}
