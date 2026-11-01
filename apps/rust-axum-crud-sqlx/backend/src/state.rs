use std::sync::Arc;

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }
}
