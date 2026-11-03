use observability::init_observability;
use rust_axum_rest_api_backend::{
    config::{load_observability_config, load_server_config},
    route,
};
use server::{
    axum::axum_server,
    task::{self, shutdown_signal},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_config = load_server_config()?;
    let observability_config = load_observability_config()?;

    init_observability(&observability_config)?;

    let routes = route::routes();
    let server_task = axum_server(routes, server_config).await;
    let signal_task = shutdown_signal();

    tokio::select! {
        result = server_task => task::run("Webserver", result),
        result = signal_task => task::shutdown(result),
    }?;
    Ok(())
}
