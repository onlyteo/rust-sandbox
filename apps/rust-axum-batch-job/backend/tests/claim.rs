//! Tests of the claim / complete / fail protocol in the repository layer.

mod common;

use std::collections::HashSet;
use std::time::Duration;

use rust_axum_batch_job_backend::model::ExternalUser;
use rust_axum_batch_job_backend::repository::user;
use uuid::Uuid;

use common::{reset_users, setup_pool};

const LEASE: Duration = Duration::from_secs(60);
const SHORT_LEASE: Duration = Duration::from_secs(1);

#[tokio::test]
async fn claim_is_exclusive_across_concurrent_instances() -> anyhow::Result<()> {
    let pool = setup_pool().await?;
    let ids = reset_users(&pool, 200).await?;

    let claims = (0..4).map(|_| {
        let pool = pool.clone();
        tokio::spawn(async move { user::claim_batch(&pool, Uuid::new_v4(), 50, LEASE).await })
    });
    let mut batches = Vec::new();
    for claim in claims {
        batches.push(claim.await??);
    }

    // SKIP LOCKED: every instance gets a full batch instead of blocking on another's candidates
    for batch in &batches {
        assert_eq!(batch.len(), 50);
    }
    let claimed: HashSet<Uuid> = batches.iter().flatten().copied().collect();
    assert_eq!(
        claimed.len(),
        200,
        "a row was claimed by more than one instance"
    );
    assert_eq!(claimed, ids.into_iter().collect());

    // Nothing left to claim
    assert!(
        user::claim_batch(&pool, Uuid::new_v4(), 50, LEASE)
            .await?
            .is_empty()
    );
    Ok(())
}

#[tokio::test]
async fn expired_lease_is_reclaimed() -> anyhow::Result<()> {
    let pool = setup_pool().await?;
    let ids = reset_users(&pool, 10).await?;
    let (a, b) = (Uuid::new_v4(), Uuid::new_v4());

    assert_eq!(
        user::claim_batch(&pool, a, 10, SHORT_LEASE).await?.len(),
        10
    );
    assert!(user::claim_batch(&pool, b, 10, LEASE).await?.is_empty());

    // Instance `a` "crashes"; once the lease expires `b` picks the rows up
    tokio::time::sleep(SHORT_LEASE + Duration::from_millis(200)).await;
    let reclaimed: HashSet<Uuid> = user::claim_batch(&pool, b, 10, LEASE)
        .await?
        .into_iter()
        .collect();
    assert_eq!(reclaimed, ids.into_iter().collect());
    Ok(())
}

#[tokio::test]
async fn stale_owner_cannot_write_back() -> anyhow::Result<()> {
    let pool = setup_pool().await?;
    let id = reset_users(&pool, 1).await?[0];
    let (a, b) = (Uuid::new_v4(), Uuid::new_v4());
    let fetched = ExternalUser {
        id,
        name: "Name".into(),
        email: "name@example.com".into(),
    };

    user::claim_batch(&pool, a, 1, SHORT_LEASE).await?;
    tokio::time::sleep(SHORT_LEASE + Duration::from_millis(200)).await;
    user::claim_batch(&pool, b, 1, LEASE).await?;

    // `a` wakes up after its lease expired: both write paths are fenced off
    assert_eq!(user::complete(&pool, a, &fetched, LEASE).await?, None);
    assert!(!user::fail(&pool, id, a, "boom", LEASE).await?);

    // The current owner succeeds and releases the row
    assert_eq!(user::complete(&pool, b, &fetched, LEASE).await?, Some(true));
    let (status, claimed_by): (String, Option<Uuid>) =
        sqlx::query_as("SELECT status, claimed_by FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&pool)
            .await?;
    assert_eq!(status, "IDLE");
    assert_eq!(claimed_by, None);
    Ok(())
}
