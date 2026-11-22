//! End-to-end tests of the sync job against Postgres and the in-process mock external API.

mod common;

use std::time::Duration;

use chrono::{DateTime, Utc};
use rust_axum_batch_job_backend::model::SyncReport;
use sqlx::PgPool;
use tokio::sync::watch;
use uuid::Uuid;

use common::{job_config, reset_users, setup_pool, start_mock, sync_job};

const LEASE: Duration = Duration::from_secs(60);

async fn data_updated_at(pool: &PgPool, id: Uuid) -> anyhow::Result<Option<DateTime<Utc>>> {
    Ok(
        sqlx::query_scalar("SELECT data_updated_at FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?,
    )
}

#[tokio::test]
async fn sync_is_idempotent() -> anyhow::Result<()> {
    let pool = setup_pool().await?;
    let ids = reset_users(&pool, 5).await?;
    let (mock, base_url) = start_mock(0).await?;
    // Zero sync interval: rows are due again immediately
    let job = sync_job(&pool, &base_url, job_config(10, LEASE, Duration::ZERO));

    let first = job.run_once().await?;
    assert_eq!(first.claimed, 5);
    assert_eq!(first.changed, 5);
    let before = data_updated_at(&pool, ids[0]).await?;
    assert!(before.is_some());

    // Unchanged upstream: nothing is rewritten
    let second = job.run_once().await?;
    assert_eq!(second.claimed, 5);
    assert_eq!(second.unchanged, 5);
    assert_eq!(data_updated_at(&pool, ids[0]).await?, before);

    // One upstream change is picked up
    mock.mutate(ids[0]);
    let third = job.run_once().await?;
    assert_eq!((third.changed, third.unchanged), (1, 4));
    assert!(data_updated_at(&pool, ids[0]).await? > before);
    let name: String = sqlx::query_scalar("SELECT name FROM users WHERE id = $1")
        .bind(ids[0])
        .fetch_one(&pool)
        .await?;
    assert_eq!(name, mock.user(ids[0]).name);
    Ok(())
}

#[tokio::test]
async fn failure_backs_off_and_recovers() -> anyhow::Result<()> {
    let pool = setup_pool().await?;
    reset_users(&pool, 3).await?;
    let (mock, base_url) = start_mock(100).await?;
    let job = sync_job(&pool, &base_url, job_config(10, LEASE, LEASE));

    let report = job.run_once().await?;
    assert_eq!((report.claimed, report.failed), (3, 3));

    type Row = (String, Option<Uuid>, i32, Option<String>, bool);
    let rows: Vec<Row> = sqlx::query_as(
        "SELECT status, claimed_by, failure_count, last_error, next_sync_at > now() FROM users",
    )
    .fetch_all(&pool)
    .await?;
    for (status, claimed_by, failure_count, last_error, backed_off) in rows {
        assert_eq!(status, "IDLE");
        assert_eq!(claimed_by, None);
        assert_eq!(failure_count, 1);
        assert!(last_error.unwrap().contains("500"));
        assert!(backed_off);
    }

    // Not due during backoff (2^0 = 1s)
    assert_eq!(job.run_once().await?.claimed, 0);

    mock.set_failure_percent(0);
    tokio::time::sleep(Duration::from_millis(1200)).await;
    let report = job.run_once().await?;
    assert_eq!((report.claimed, report.changed), (3, 3));
    let max_failures: i32 = sqlx::query_scalar("SELECT max(failure_count) FROM users")
        .fetch_one(&pool)
        .await?;
    assert_eq!(max_failures, 0);
    Ok(())
}

#[tokio::test]
async fn concurrent_instances_split_the_work() -> anyhow::Result<()> {
    let pool = setup_pool().await?;
    reset_users(&pool, 100).await?;
    let (_mock, base_url) = start_mock(0).await?;

    let jobs: Vec<_> = (0..3)
        .map(|_| sync_job(&pool, &base_url, job_config(20, LEASE, LEASE)))
        .collect();

    // Keep ticking all instances concurrently until nothing is due
    let mut total = SyncReport::default();
    loop {
        let handles: Vec<_> = jobs
            .iter()
            .cloned()
            .map(|job| tokio::spawn(async move { job.run_once().await }))
            .collect();
        let mut claimed = 0;
        for handle in handles {
            let report = handle.await??;
            claimed += report.claimed;
            total.claimed += report.claimed;
            total.changed += report.changed;
            total.lost_lease += report.lost_lease;
            total.errors += report.errors;
        }
        if claimed == 0 {
            break;
        }
    }

    // Every user processed exactly once
    assert_eq!(total.claimed, 100);
    assert_eq!(total.changed, 100);
    assert_eq!((total.lost_lease, total.errors), (0, 0));
    Ok(())
}

#[tokio::test]
async fn run_stops_on_shutdown() -> anyhow::Result<()> {
    let pool = setup_pool().await?;
    reset_users(&pool, 3).await?;
    let (_mock, base_url) = start_mock(0).await?;
    let job = sync_job(&pool, &base_url, job_config(10, LEASE, LEASE));

    let (tx, rx) = watch::channel(false);
    let handle = tokio::spawn(job.run(rx));
    tokio::time::sleep(Duration::from_millis(300)).await;
    tx.send(true)?;
    tokio::time::timeout(Duration::from_secs(5), handle).await???;

    // The first (immediate) tick synced everything
    let synced: i64 = sqlx::query_scalar("SELECT count(*) FROM users WHERE name IS NOT NULL")
        .fetch_one(&pool)
        .await?;
    assert_eq!(synced, 3);
    Ok(())
}
