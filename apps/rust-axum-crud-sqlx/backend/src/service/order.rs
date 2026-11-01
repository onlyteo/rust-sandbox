use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{CreateOrderRequest, Order, OrderStatus, UpdateOrderRequest},
    repository,
};

pub async fn get_order_by_id(
    pool: PgPool,
    id: Uuid,
) -> anyhow::Result<Option<Order>, DatabaseError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(DatabaseError::transaction_failed)?;
    let optional_order = repository::order::select_by_id(&mut tx, &id).await?;
    tx.commit()
        .await
        .map_err(DatabaseError::transaction_failed)?;
    Ok(optional_order)
}

pub async fn find_orders(pool: PgPool) -> anyhow::Result<Vec<Order>, DatabaseError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(DatabaseError::transaction_failed)?;
    let orders = repository::order::select(&mut tx).await?;
    tx.commit()
        .await
        .map_err(DatabaseError::transaction_failed)?;
    Ok(orders)
}

pub async fn create_order(
    pool: PgPool,
    request: CreateOrderRequest,
) -> anyhow::Result<Option<Order>, DatabaseError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(DatabaseError::transaction_failed)?;
    let id = repository::order::insert(&mut tx, &request.username).await?;
    repository::order::update(&mut tx, &id, &OrderStatus::Open, &request.products).await?;
    let optional_order = repository::order::select_by_id(&mut tx, &id).await?;
    tx.commit()
        .await
        .map_err(DatabaseError::transaction_failed)?;
    Ok(optional_order)
}

pub async fn update_order(
    pool: PgPool,
    id: Uuid,
    request: UpdateOrderRequest,
) -> anyhow::Result<Option<Order>, DatabaseError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(DatabaseError::transaction_failed)?;
    repository::order::update(&mut tx, &id, &request.status, &request.products).await?;
    let optional_order = repository::order::select_by_id(&mut tx, &id).await?;
    tx.commit()
        .await
        .map_err(DatabaseError::transaction_failed)?;
    Ok(optional_order)
}
