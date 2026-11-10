use chrono::{DateTime, Utc};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{Order, OrderProduct, OrderStatus},
};

#[derive(Debug, sqlx::FromRow)]
struct OrderRow {
    pub id: Uuid,
    pub username: String,
    pub status: OrderStatus,
    pub timestamp: DateTime<Utc>,
    pub product_id: Option<Uuid>,
    pub quantity: Option<i32>,
}

pub async fn select(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<Order>, DatabaseError> {
    let rows = sqlx::query_as::<_, OrderRow>(
        r#"
            SELECT
                o.id,
                o.username,
                o.status,
                o.timestamp AT TIME ZONE 'UTC' AS timestamp,
                oi.product_id,
                oi.quantity
            FROM orders o
            LEFT JOIN order_items oi ON oi.order_id = o.id
            ORDER BY o.timestamp
            "#,
    )
    .fetch_all(&mut **tx)
    .await
    .map_err(DatabaseError::select_orders_failed)?;

    let orders = map_orders(rows);

    Ok(orders)
}

pub async fn select_by_id(
    tx: &mut Transaction<'_, Postgres>,
    id: &Uuid,
) -> anyhow::Result<Option<Order>, DatabaseError> {
    let rows = sqlx::query_as::<_, OrderRow>(
        r#"
            SELECT
                o.id,
                o.username,
                o.status,
                o.timestamp AT TIME ZONE 'UTC' AS timestamp,
                oi.product_id,
                oi.quantity
            FROM orders o
            LEFT JOIN order_items oi ON oi.order_id = o.id
            WHERE o.id = $1
            "#,
    )
    .bind(id)
    .fetch_all(&mut **tx)
    .await
    .map_err(DatabaseError::select_orders_failed)?;

    let orders = map_orders(rows);

    if orders.len() > 1 {
        Err(DatabaseError::multiple_orders_found(id.clone()))
    } else {
        Ok(orders.first().map(|o| o.clone()))
    }
}

pub async fn insert(
    tx: &mut Transaction<'_, Postgres>,
    username: &String,
) -> anyhow::Result<Uuid, DatabaseError> {
    let id = Uuid::new_v4();
    sqlx::query(
        r#"
            INSERT INTO orders (
                id,
                username,
                status,
                timestamp
            ) VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(id)
    .bind(username)
    .bind(OrderStatus::Open)
    .bind(Utc::now())
    .execute(&mut **tx)
    .await
    .map_err(DatabaseError::insert_order_failed)?;

    Ok(id)
}

pub async fn update(
    tx: &mut Transaction<'_, Postgres>,
    id: &Uuid,
    status: &OrderStatus,
    products: &[OrderProduct],
) -> anyhow::Result<u64, DatabaseError> {
    let product_ids: Vec<Uuid> = products.iter().map(|p| p.id).collect();
    let quantities: Vec<i32> = products.iter().map(|p| p.quantity).collect();

    let delete_result = sqlx::query(
        r#"
            DELETE FROM order_items
            WHERE order_id = $1
            AND product_id != ALL($2)
        "#,
    )
    .bind(id)
    .bind(&product_ids)
    .execute(&mut **tx)
    .await
    .map_err(DatabaseError::delete_order_items_failed)?;

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

    let update_result = sqlx::query(
        r#"
            UPDATE orders
            SET status = $2
            WHERE id = $1
            AND status != $2
        "#,
    )
    .bind(id)
    .bind(&status)
    .execute(&mut **tx)
    .await
    .map_err(DatabaseError::insert_order_items_failed)?;

    Ok(delete_result.rows_affected()
        + insert_result.rows_affected()
        + update_result.rows_affected())
}

fn map_orders(rows: Vec<OrderRow>) -> Vec<Order> {
    let mut orders: Vec<Order> = Vec::new();

    for row in rows {
        let product = match (row.product_id, row.quantity) {
            (Some(id), Some(quantity)) => Some(OrderProduct { id, quantity }),
            _ => None,
        };

        match orders.last_mut() {
            Some(order) if order.id == row.id => {
                if let Some(product) = product {
                    order.products.push(product);
                }
            }
            _ => orders.push(Order {
                id: row.id,
                username: row.username,
                products: product.into_iter().collect(),
                status: row.status,
                timestamp: row.timestamp,
            }),
        }
    }

    orders
}
