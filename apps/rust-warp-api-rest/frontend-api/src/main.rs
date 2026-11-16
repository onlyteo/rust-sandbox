use std::net::SocketAddr;

use observability::init_observability;
use rust_warp_rest_api_frontend_api::config::{
    load_app_config, load_observability_config, load_server_config,
};
use rust_warp_rest_api_frontend_api::route;
use rust_warp_rest_api_frontend_api::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_config = load_server_config()?;
    let app_config = load_app_config()?;
    let observability_config = load_observability_config()?;

    init_observability(&observability_config)?;

    let routes = route::routes(AppState::new(app_config));

    tracing::info!(
        "Starting webserver at address {}",
        server_config.to_string()
    );
    let address: SocketAddr = server_config.into();
    warp::serve(routes).run(address).await;

    Ok(())
}
