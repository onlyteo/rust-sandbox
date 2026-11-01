use async_handler::{handler, task::shutdown_signal};
use axum::Router;
use observability::init_observability;
use rust_axum_crud_sqlx_frontend::client::HttpClient;
use rust_axum_crud_sqlx_frontend::config::{
    load_app_config, load_observability_config, load_server_config,
};
use rust_axum_crud_sqlx_frontend::model::User;
use rust_axum_crud_sqlx_frontend::route;
use rust_axum_crud_sqlx_frontend::state::AppState;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let observability_config = load_observability_config()?;
    init_observability(&observability_config)?;

    let app_config = load_app_config()?;

    let http_client = HttpClient::new(app_config.backend.url);
    let state = AppState::new(User::from(app_config.user), http_client);
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
