use std::time::Duration;

use observability::init_observability;
use rust_axum_batch_job_backend::client::ExternalApiClient;
use rust_axum_batch_job_backend::config::{
    load_database_config, load_external_api_config, load_mock_external_api_config,
    load_observability_config, load_server_config, load_sync_job_config,
};
use rust_axum_batch_job_backend::job::SyncJob;
use rust_axum_batch_job_backend::mock::MockExternalApi;
use rust_axum_batch_job_backend::route;
use server::axum::axum_server;
use server::task::{self, shutdown_signal};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::watch;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_config = load_server_config()?;
    let observability_config = load_observability_config()?;
    let database_config = load_database_config()?;
    let external_api_config = load_external_api_config()?;
    let mock_config = load_mock_external_api_config()?;
    let sync_job_config = load_sync_job_config()?;

    init_observability(&observability_config)?;

    let pool = PgPoolOptions::new()
        .max_connections(database_config.max_connections)
        .connect(&database_config.url)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Stand-in for the external system
    let mock = MockExternalApi::new(mock_config.failure_percent);
    tokio::spawn(mock.clone().run_mutator(
        Duration::from_secs(mock_config.mutation_interval_secs),
        mock_config.mutation_percent,
    ));
    let mock_task = axum_server(mock.routes(), mock_config.server_config()).await;

    // Sync job
    let client = ExternalApiClient::new(&external_api_config)?;
    let job = SyncJob::new(pool, client, sync_job_config);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let mut job_task = tokio::spawn(job.run(shutdown_rx));

    let server_task = axum_server(route::routes(), server_config).await;
    let signal_task = shutdown_signal();

    let result = tokio::select! {
        result = server_task => task::run("Webserver", result),
        result = mock_task => task::run("Mock external API", result),
        result = &mut job_task => return task::run("Sync job", result),
        result = signal_task => task::shutdown(result),
    };

    // Let the job finish its in-flight batch before exiting
    let _ = shutdown_tx.send(true);
    task::run("Sync job", job_task.await)?;
    result
}
