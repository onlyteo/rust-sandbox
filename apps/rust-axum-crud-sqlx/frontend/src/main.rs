use std::sync::Arc;

use observability::init_observability;
use reqwest::Client;
use rust_axum_crud_sqlx_frontend::config::{
    load_app_config, load_observability_config, load_server_config,
};
use rust_axum_crud_sqlx_frontend::model::BootstrapPrincipal;
use rust_axum_crud_sqlx_frontend::route;
use rust_axum_crud_sqlx_frontend::service::order::OrderService;
use rust_axum_crud_sqlx_frontend::service::product::ProductService;
use rust_axum_crud_sqlx_frontend::state::AppState;
use server::axum::axum_server;
use server::task::{self, shutdown_signal};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_config = load_server_config()?;
    let app_config = load_app_config()?;
    let observability_config = load_observability_config()?;

    init_observability(&observability_config)?;

    let client = Arc::new(Client::new());
    let order_service = OrderService::new(client.clone(), app_config.backend.url.clone());
    let product_service = ProductService::new(client.clone(), app_config.backend.url.clone());
    let app_state = AppState::new(
        Arc::new(BootstrapPrincipal::from(app_config)),
        order_service,
        product_service,
    );
    let routes = route::routes(app_state);
    let server_task = axum_server(routes, server_config).await;
    let signal_task = shutdown_signal();

    tokio::select! {
        result = server_task => task::run("Webserver", result),
        result = signal_task => task::shutdown(result),
    }?;
    Ok(())
}
