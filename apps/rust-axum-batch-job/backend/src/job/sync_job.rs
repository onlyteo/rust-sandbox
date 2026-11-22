use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::{Semaphore, watch};
use tokio::task::JoinSet;
use tokio::time::{MissedTickBehavior, interval};
use uuid::Uuid;

use crate::client::ExternalApiClient;
use crate::config::SyncJobConfig;
use crate::error::DatabaseError;
use crate::model::{SyncOutcome, SyncReport};
use crate::repository::user;

/// Periodically syncs cached users with the external system.
///
/// Safe to run on many instances against the same database: each instance has its own
/// `instance_id` and only processes rows it has claimed (see [`crate::repository::user`]).
#[derive(Debug, Clone)]
pub struct SyncJob {
    pool: PgPool,
    client: ExternalApiClient,
    config: SyncJobConfig,
    instance_id: Uuid,
}

impl SyncJob {
    pub fn new(pool: PgPool, client: ExternalApiClient, config: SyncJobConfig) -> Self {
        Self::with_instance_id(pool, client, config, Uuid::new_v4())
    }

    pub fn with_instance_id(
        pool: PgPool,
        client: ExternalApiClient,
        config: SyncJobConfig,
        instance_id: Uuid,
    ) -> Self {
        Self {
            pool,
            client,
            config,
            instance_id,
        }
    }

    pub fn instance_id(&self) -> Uuid {
        self.instance_id
    }

    /// Runs until `shutdown` flips to `true`. Errors in a tick are logged and never stop the
    /// loop. A tick that is in progress when shutdown is signalled is allowed to finish.
    pub async fn run(self, mut shutdown: watch::Receiver<bool>) -> anyhow::Result<()> {
        tracing::info!("Sync job started with instance id {}", self.instance_id);
        let mut ticker = interval(self.config.tick_interval());
        ticker.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            tokio::select! {
                _ = ticker.tick() => {}
                _ = shutdown.changed() => break,
            }
            if *shutdown.borrow() {
                break;
            }
            match self.run_once().await {
                Ok(report) if report.claimed > 0 => tracing::info!("Sync tick: {report:?}"),
                Ok(_) => tracing::debug!("Sync tick: nothing due"),
                Err(e) => tracing::error!("Sync tick failed: {e}"),
            }
        }

        tracing::info!("Sync job stopped");
        Ok(())
    }

    /// Claims one batch and syncs it with bounded concurrency.
    pub async fn run_once(&self) -> Result<SyncReport, DatabaseError> {
        let ids = user::claim_batch(
            &self.pool,
            self.instance_id,
            self.config.batch_size,
            self.config.lease(),
        )
        .await?;

        let mut report = SyncReport {
            claimed: ids.len(),
            ..Default::default()
        };
        if ids.is_empty() {
            return Ok(report);
        }

        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrency.max(1)));
        let mut tasks = JoinSet::new();
        for id in ids {
            let permit = semaphore
                .clone()
                .acquire_owned()
                .await
                .expect("semaphore is never closed");
            let job = self.clone();
            tasks.spawn(async move {
                let _permit = permit;
                (id, job.sync_user(id).await)
            });
        }

        while let Some(result) = tasks.join_next().await {
            match result {
                Ok((_, Ok(outcome))) => report.record(outcome),
                Ok((id, Err(e))) => {
                    report.errors += 1;
                    tracing::error!("Sync of user {id} failed, lease will expire: {e}");
                }
                Err(e) => {
                    report.errors += 1;
                    tracing::error!("Sync task panicked or was cancelled, lease will expire: {e}");
                }
            }
        }
        Ok(report)
    }

    async fn sync_user(&self, id: Uuid) -> Result<SyncOutcome, DatabaseError> {
        match self.client.get_user(id).await {
            Ok(external_user) => {
                let result = user::complete(
                    &self.pool,
                    self.instance_id,
                    &external_user,
                    self.config.sync_interval(),
                )
                .await?;
                Ok(match result {
                    Some(true) => SyncOutcome::Changed,
                    Some(false) => SyncOutcome::Unchanged,
                    None => {
                        tracing::warn!("Lost lease for user {id}, discarding result");
                        SyncOutcome::LostLease
                    }
                })
            }
            Err(e) => {
                tracing::warn!("{e}");
                let released = user::fail(
                    &self.pool,
                    id,
                    self.instance_id,
                    &e.to_string(),
                    self.config.max_backoff(),
                )
                .await?;
                Ok(if released {
                    SyncOutcome::Failed
                } else {
                    SyncOutcome::LostLease
                })
            }
        }
    }
}
