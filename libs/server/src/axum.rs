use axum::Router;
use tokio::task::JoinHandle;

use crate::config::ServerConfig;

pub async fn axum_server(routes: Router, config: ServerConfig) -> JoinHandle<anyhow::Result<()>> {
    tokio::spawn(async move {
        let address = config.to_string();
        tracing::info!("Starter webserver på adresse {}", address);
        let listener = tokio::net::TcpListener::bind(&address).await?;
        axum::serve(listener, routes).await?;
        Ok(())
    })
}
