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

pub async fn select_by_statuses(
    tx: &mut Transaction<'_, Postgres>,
    username: &String,
    statuses: &[OrderStatus],
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
            WHERE o.username = $1 AND o.status = ANY($2)
            ORDER BY o.timestamp
            "#,
    )
    .bind(username)
    .bind(statuses)
    .fetch_all(&mut **tx)
    .await
    .map_err(DatabaseError::select_orders_failed)?;

    let orders = map_orders(rows);

    Ok(orders)
}

pub async fn select_by_id(
    tx: &mut Transaction<'_, Postgres>,
    id: &Uuid,
    username: &String,
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
            WHERE o.id = $1 AND o.username = $2
            "#,
    )
    .bind(id)
    .bind(username)
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
    username: &String,
    status: &OrderStatus,
) -> anyhow::Result<u64, DatabaseError> {
    let update_result = sqlx::query(
        r#"
            UPDATE orders
            SET status = $3
            WHERE id = $1
            AND username = $2
            AND status != $3
        "#,
    )
    .bind(id)
    .bind(username)
    .bind(&status)
    .execute(&mut **tx)
    .await
    .map_err(DatabaseError::insert_order_items_failed)?;

    Ok(update_result.rows_affected())
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
