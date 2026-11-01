use std::sync::Arc;

use crate::{error::DatabaseError, model::Product, repository};

use sqlx::PgPool;

pub struct ProductService {
    pool: PgPool,
}

impl ProductService {
    pub fn new(pool: PgPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    pub async fn find_products(&self) -> anyhow::Result<Vec<Product>, DatabaseError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(DatabaseError::transaction_failed)?;
        let products = repository::product::select(&mut tx).await?;
        tx.commit()
            .await
            .map_err(DatabaseError::transaction_failed)?;
        Ok(products)
    }
}
