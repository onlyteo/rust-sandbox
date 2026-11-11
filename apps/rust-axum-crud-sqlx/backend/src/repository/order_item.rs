use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{error::DatabaseError, model::OrderProduct};

pub async fn insert(
    tx: &mut Transaction<'_, Postgres>,
    id: &Uuid,
    products: &[OrderProduct],
) -> anyhow::Result<u64, DatabaseError> {
    let product_ids: Vec<Uuid> = products.iter().map(|p| p.id).collect();
    let quantities: Vec<i32> = products.iter().map(|p| p.quantity).collect();

    let insert_result = sqlx::query(
        r#"
            INSERT INTO order_items (order_id, product_id, quantity)
            SELECT $1, product_id, quantity
            FROM UNNEST($2::uuid[], $3::int[]) AS t(product_id, quantity)
            ON CONFLICT (order_id, product_id)
            DO UPDATE SET quantity = excluded.quantity
        "#,
    )
    .bind(id)
    .bind(&product_ids)
    .bind(&quantities)
    .execute(&mut **tx)
    .await
    .map_err(DatabaseError::insert_order_items_failed)?;

    Ok(insert_result.rows_affected())
}

pub async fn delete(
    tx: &mut Transaction<'_, Postgres>,
    id: &Uuid,
    username: &String,
    products: &[OrderProduct],
) -> anyhow::Result<u64, DatabaseError> {
    let product_ids: Vec<Uuid> = products.iter().map(|p| p.id).collect();

    let delete_result = sqlx::query(
        r#"
            DELETE FROM order_items oi
            USING orders o
            WHERE o.id = oi.order_id
            AND o.id = $1
            AND o.username = $2
            AND product_id != ALL($3)
        "#,
    )
    .bind(id)
    .bind(username)
    .bind(&product_ids)
    .execute(&mut **tx)
    .await
    .map_err(DatabaseError::delete_order_items_failed)?;

    Ok(delete_result.rows_affected())
}
