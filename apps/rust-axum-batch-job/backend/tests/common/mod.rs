#![allow(dead_code)]

use std::sync::Arc;
use std::time::Duration;

use postgres_testcontainer::postgres::setup_postgres_container;
use rust_axum_batch_job_backend::client::ExternalApiClient;
use rust_axum_batch_job_backend::config::{ExternalApiConfig, SyncJobConfig};
use rust_axum_batch_job_backend::job::SyncJob;
use rust_axum_batch_job_backend::mock::MockExternalApi;
use sqlx::PgPool;
use uuid::Uuid;

/// Returns a pool connected to an isolated schema in the shared Postgres
/// testcontainer, with all migrations applied.
pub async fn setup_pool() -> anyhow::Result<PgPool> {
    let guard = setup_postgres_container().await?;
    sqlx::migrate!("./migrations").run(&guard.pg_pool).await?;
    Ok(guard.pg_pool)
}

/// Replaces the seeded users with `count` fresh, due users.
pub async fn reset_users(pool: &PgPool, count: usize) -> anyhow::Result<Vec<Uuid>> {
    sqlx::query("DELETE FROM users").execute(pool).await?;
    let ids: Vec<Uuid> = (0..count).map(|_| Uuid::new_v4()).collect();
    sqlx::query("INSERT INTO users (id) SELECT unnest($1::uuid[])")
        .bind(&ids)
        .execute(pool)
        .await?;
    Ok(ids)
}

/// Starts the mock external API on an ephemeral port and returns it with its base URL.
pub async fn start_mock(failure_percent: u8) -> anyhow::Result<(Arc<MockExternalApi>, String)> {
    let mock = MockExternalApi::new(failure_percent);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let base_url = format!("http://{}", listener.local_addr()?);
    let routes = mock.routes();
    tokio::spawn(async move { axum::serve(listener, routes).await });
    Ok((mock, base_url))
}

pub fn job_config(batch_size: i64, lease: Duration, sync_interval: Duration) -> SyncJobConfig {
    SyncJobConfig {
        tick_interval_secs: 1,
        batch_size,
        lease_secs: lease.as_secs(),
        sync_interval_secs: sync_interval.as_secs(),
        max_concurrency: 4,
        max_backoff_secs: 60,
    }
}

pub fn sync_job(pool: &PgPool, base_url: &str, config: SyncJobConfig) -> SyncJob {
    let client = ExternalApiClient::new(&ExternalApiConfig {
        base_url: base_url.to_string(),
        request_timeout_secs: 5,
    })
    .expect("client");
    SyncJob::new(pool.clone(), client, config)
}
