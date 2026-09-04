use async_handler::{handler, task::shutdown_signal};
use axum::Router;
use rust_axum_crud_sqlx_frontend::route;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let routes = route::routes();
    let server_task = web_server(routes).await;
    let signal_task = shutdown_signal();

    tokio::select! {
        result = server_task => handler::run("Webserver", result),
        result = signal_task => handler::shutdown(result),
    }?;
    Ok(())
}

async fn web_server(routes: Router) -> JoinHandle<anyhow::Result<()>> {
    tracing::info!("Starter webserver på adresse 0.0.0.0:8080");
    tokio::spawn(async move {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
        axum::serve(listener, routes).await?;
        Ok(())
    })
}
