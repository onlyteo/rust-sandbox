use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User as returned by the external system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

/// Result of writing a fetched user back to the local cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncOutcome {
    /// Payload differed from the cached one and was updated.
    Changed,
    /// Payload was identical; only sync metadata was touched.
    Unchanged,
    /// Fetch failed; row released with backoff.
    Failed,
    /// Our lease expired and another instance re-claimed the row; result discarded.
    LostLease,
}

/// Aggregated outcome of one job tick.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SyncReport {
    pub claimed: usize,
    pub changed: usize,
    pub unchanged: usize,
    pub failed: usize,
    pub lost_lease: usize,
    /// Unexpected errors (database errors, panicked tasks). The lease expires and the row is retried.
    pub errors: usize,
}

impl SyncReport {
    pub fn record(&mut self, outcome: SyncOutcome) {
        match outcome {
            SyncOutcome::Changed => self.changed += 1,
            SyncOutcome::Unchanged => self.unchanged += 1,
            SyncOutcome::Failed => self.failed += 1,
            SyncOutcome::LostLease => self.lost_lease += 1,
        }
    }
}
