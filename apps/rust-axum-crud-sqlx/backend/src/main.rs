use async_handler::{handler, task::shutdown_signal};
use axum::Router;
use observability::init_observability;
use rust_axum_crud_sqlx_backend::config::{
    load_database_config, load_observability_config, load_server_config,
};
use rust_axum_crud_sqlx_backend::route;
use rust_axum_crud_sqlx_backend::service::order::OrderService;
use rust_axum_crud_sqlx_backend::service::product::ProductService;
use rust_axum_crud_sqlx_backend::state::AppState;
use sqlx::postgres::PgPoolOptions;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let observability_config = load_observability_config()?;
    init_observability(&observability_config)?;

    let database_config = load_database_config()?;

    let pool = PgPoolOptions::new()
        .max_connections(database_config.max_connections)
        .connect(&database_config.url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let order_service = OrderService::new(pool.clone());
    let product_service = ProductService::new(pool.clone());

    let state = AppState::new(order_service, product_service);
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
