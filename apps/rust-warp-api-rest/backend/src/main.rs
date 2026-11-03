use std::net::SocketAddr;

use observability::init_observability;
use rust_warp_rest_api_backend::{
    config::{load_observability_config, load_server_config},
    route,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_config = load_server_config()?;
    let observability_config = load_observability_config()?;

    init_observability(&observability_config)?;

    let routes = route::routes();

    tracing::info!("Starter webserver på adresse {}", server_config.to_string());
    let address: SocketAddr = server_config.into();
    warp::serve(routes).run(address).await;

    Ok(())
}
