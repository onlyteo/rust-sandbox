use std::sync::Arc;

use async_handler::{handler, task::shutdown_signal};
use axum::Router;
use observability::init_observability;
use reqwest::Client;
use rust_axum_crud_sqlx_frontend::config::{
    load_app_config, load_observability_config, load_server_config,
};
use rust_axum_crud_sqlx_frontend::model::User;
use rust_axum_crud_sqlx_frontend::route;
use rust_axum_crud_sqlx_frontend::service::order::OrderService;
use rust_axum_crud_sqlx_frontend::service::product::ProductService;
use rust_axum_crud_sqlx_frontend::state::AppState;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let observability_config = load_observability_config()?;
    init_observability(&observability_config)?;

    let app_config = load_app_config()?;

    let client = Arc::new(Client::new());
    let order_service = OrderService::new(client.clone(), app_config.backend.url.clone());
    let product_service = ProductService::new(client.clone(), app_config.backend.url.clone());
    let state = AppState::new(User::from(app_config.user), order_service, product_service);
    let routes = route::routes(state);
    let server_task = web_server(routes).await;
    let signal_task = shutdown_signal();

    tokio::select! {
        result = server_task => handler::run("Webserver", result),
        result = signal_task => handler::shutdown(result),
    }?;
    Ok(())
}

async fn web_server(routes: Router) -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(async move {
        let server_config = load_server_config()?;
        tracing::info!("Starter webserver på adresse {}", server_config.address);
        let listener = tokio::net::TcpListener::bind(&server_config.address).await?;
        axum::serve(listener, routes).await?;
        Ok(())
    })
}
