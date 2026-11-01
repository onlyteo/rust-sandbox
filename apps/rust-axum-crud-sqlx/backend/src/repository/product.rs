use sqlx::{Postgres, Transaction};

use crate::{error::DatabaseError, model::Product};

pub async fn select(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<Product>, DatabaseError> {
    let rows = sqlx::query_as::<_, Product>(
        r#"
            SELECT
                id,
                name,
                description,
                packing,
                price,
                stock
            FROM products
            ORDER BY name
            "#,
    )
    .fetch_all(&mut **tx)
    .await
    .map_err(DatabaseError::select_products_failed)?;
    Ok(rows)
}
