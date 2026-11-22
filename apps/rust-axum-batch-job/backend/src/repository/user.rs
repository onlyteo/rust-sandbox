//! Claim / complete / fail protocol for syncing users.
//!
//! 1. `claim_batch` flips due rows to `PROCESSING` with a lease in a single short statement.
//!    `FOR UPDATE SKIP LOCKED` makes concurrent instances skip each other's candidates instead of
//!    blocking on them, so every instance gets a disjoint, non-empty batch. The row locks are held
//!    only for the duration of this statement.
//! 2. The caller fetches from the external API with no transaction open.
//! 3. `complete` / `fail` write back fenced on `claimed_by`: if the lease expired and another
//!    instance re-claimed the row, the stale write affects zero rows and is discarded.

use std::time::Duration;

use sqlx::PgPool;
use uuid::Uuid;

use crate::error::DatabaseError;
use crate::model::ExternalUser;

/// Claims up to `batch_size` due users for `instance_id`. Rows whose lease has expired (e.g. the
/// owning instance crashed) are due as well.
pub async fn claim_batch(
    pool: &PgPool,
    instance_id: Uuid,
    batch_size: i64,
    lease: Duration,
) -> Result<Vec<Uuid>, DatabaseError> {
    sqlx::query_scalar::<_, Uuid>(
        r#"
            UPDATE users
            SET status           = 'PROCESSING',
                claimed_by       = $1,
                lease_expires_at = now() + make_interval(secs => $3)
            WHERE id IN (
                SELECT id
                FROM users
                WHERE next_sync_at <= now()
                  AND (status = 'IDLE' OR lease_expires_at < now())
                ORDER BY next_sync_at
                LIMIT $2
                FOR UPDATE SKIP LOCKED
            )
            RETURNING id
            "#,
    )
    .bind(instance_id)
    .bind(batch_size)
    .bind(lease.as_secs_f64())
    .fetch_all(pool)
    .await
    .map_err(DatabaseError::claim_users_failed)
}

/// Stores the fetched payload and releases the claim.
///
/// Returns `Some(true)` if the payload changed, `Some(false)` if it was identical (only sync
/// metadata is touched, `data_updated_at` is kept), or `None` if `instance_id` no longer owns
/// the row.
pub async fn complete(
    pool: &PgPool,
    instance_id: Uuid,
    user: &ExternalUser,
    sync_interval: Duration,
) -> Result<Option<bool>, DatabaseError> {
    sqlx::query_scalar::<_, bool>(
        r#"
            WITH previous AS (
                SELECT id, (name, email) IS DISTINCT FROM ($3::text, $4::text) AS changed
                FROM users
                WHERE id = $1 AND claimed_by = $2
            )
            UPDATE users u
            SET name             = $3,
                email            = $4,
                data_updated_at  = CASE WHEN previous.changed THEN now() ELSE u.data_updated_at END,
                status           = 'IDLE',
                claimed_by       = NULL,
                lease_expires_at = NULL,
                last_synced_at   = now(),
                next_sync_at     = now() + make_interval(secs => $5),
                failure_count    = 0,
                last_error       = NULL
            FROM previous
            WHERE u.id = previous.id
              AND u.claimed_by = $2
            RETURNING previous.changed
            "#,
    )
    .bind(user.id)
    .bind(instance_id)
    .bind(&user.name)
    .bind(&user.email)
    .bind(sync_interval.as_secs_f64())
    .fetch_optional(pool)
    .await
    .map_err(DatabaseError::complete_user_failed)
}

/// Records a failed sync and releases the claim with exponential backoff
/// (`min(2^failure_count, max_backoff)` seconds). Returns `false` if `instance_id` no longer owns
/// the row.
pub async fn fail(
    pool: &PgPool,
    id: Uuid,
    instance_id: Uuid,
    error: &str,
    max_backoff: Duration,
) -> Result<bool, DatabaseError> {
    let result = sqlx::query(
        r#"
            UPDATE users
            SET status           = 'IDLE',
                claimed_by       = NULL,
                lease_expires_at = NULL,
                failure_count    = failure_count + 1,
                last_error       = $3,
                next_sync_at     = now() + make_interval(secs => LEAST(power(2, failure_count), $4))
            WHERE id = $1
              AND claimed_by = $2
            "#,
    )
    .bind(id)
    .bind(instance_id)
    .bind(error)
    .bind(max_backoff.as_secs_f64())
    .execute(pool)
    .await
    .map_err(DatabaseError::fail_user_failed)?;
    Ok(result.rows_affected() == 1)
}
