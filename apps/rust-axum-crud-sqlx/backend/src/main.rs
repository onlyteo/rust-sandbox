use observability::init_observability;
use rust_axum_crud_sqlx_backend::config::{
    load_database_config, load_observability_config, load_server_config,
};
use rust_axum_crud_sqlx_backend::route;
use rust_axum_crud_sqlx_backend::service::order::OrderService;
use rust_axum_crud_sqlx_backend::service::product::ProductService;
use rust_axum_crud_sqlx_backend::state::AppState;
use server::axum::axum_server;
use server::task::{self, shutdown_signal};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_config = load_server_config()?;
    let observability_config = load_observability_config()?;
    let database_config = load_database_config()?;

    init_observability(&observability_config)?;

    let pool = PgPoolOptions::new()
        .max_connections(database_config.max_connections)
        .connect(&database_config.url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let order_service = OrderService::new(pool.clone());
    let product_service = ProductService::new(pool.clone());

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
