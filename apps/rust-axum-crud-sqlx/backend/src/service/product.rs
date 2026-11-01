use crate::{error::DatabaseError, model::Product, repository};

use sqlx::PgPool;

pub async fn find_products(pool: PgPool) -> anyhow::Result<Vec<Product>, DatabaseError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(DatabaseError::transaction_failed)?;
    let products = repository::product::select(&mut tx).await?;
    tx.commit()
        .await
        .map_err(DatabaseError::transaction_failed)?;
    Ok(products)
}
