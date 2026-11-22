//! In-process stand-in for the external user system.
//!
//! Users are created lazily (deterministically from their id) on first request and are
//! randomly mutated in the background to simulate upstream changes. A configurable share of
//! requests fails with HTTP 500 to exercise the error/backoff path.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use uuid::Uuid;

use crate::model::ExternalUser;

#[derive(Debug, Default)]
pub struct MockExternalApi {
    users: RwLock<HashMap<Uuid, (ExternalUser, u32)>>,
    failure_percent: AtomicU8,
}

impl MockExternalApi {
    pub fn new(failure_percent: u8) -> Arc<Self> {
        Arc::new(Self {
            users: RwLock::default(),
            failure_percent: AtomicU8::new(failure_percent),
        })
    }

    pub fn routes(self: &Arc<Self>) -> Router {
        Router::new()
            .route("/users/{id}", get(get_user))
            .with_state(self.clone())
    }

    pub fn set_failure_percent(&self, percent: u8) {
        self.failure_percent.store(percent, Ordering::Relaxed);
    }

    /// Returns the current upstream state of a user, creating it on first access.
    pub fn user(&self, id: Uuid) -> ExternalUser {
        if let Some((user, _)) = self.users.read().unwrap().get(&id) {
            return user.clone();
        }
        let mut users = self.users.write().unwrap();
        users
            .entry(id)
            .or_insert_with(|| (build_user(id, 0), 0))
            .0
            .clone()
    }

    /// Changes the upstream payload of a user.
    pub fn mutate(&self, id: Uuid) {
        let mut users = self.users.write().unwrap();
        let entry = users.entry(id).or_insert_with(|| (build_user(id, 0), 0));
        entry.1 += 1;
        entry.0 = build_user(id, entry.1);
    }

    /// Mutates roughly `percent` % of the known users every `every`. Runs forever.
    pub async fn run_mutator(self: Arc<Self>, every: Duration, percent: u8) {
        let mut ticker = tokio::time::interval(every);
        ticker.tick().await;
        loop {
            ticker.tick().await;
            let ids: Vec<Uuid> = self.users.read().unwrap().keys().copied().collect();
            let mut mutated = 0;
            for id in ids {
                if roll(percent) {
                    self.mutate(id);
                    mutated += 1;
                }
            }
            tracing::debug!("Mock external API mutated {mutated} users");
        }
    }
}

async fn get_user(State(api): State<Arc<MockExternalApi>>, Path(id): Path<Uuid>) -> Response {
    if roll(api.failure_percent.load(Ordering::Relaxed)) {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Simulated failure").into_response();
    }
    Json(api.user(id)).into_response()
}

fn build_user(id: Uuid, version: u32) -> ExternalUser {
    let short = &id.simple().to_string()[..8];
    ExternalUser {
        id,
        name: format!("User {short} v{version}"),
        email: format!("user-{short}+v{version}@example.com"),
    }
}

/// True with probability `percent`/100. Uses a v4 UUID as randomness source.
fn roll(percent: u8) -> bool {
    percent > 0 && (Uuid::new_v4().as_u128() % 100) < percent as u128
}
