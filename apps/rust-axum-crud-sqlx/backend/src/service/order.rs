use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::DatabaseError,
    model::{CreateOrderRequest, Order, OrderStatus, UpdateOrderRequest},
    repository,
};

pub struct OrderService {
    pool: PgPool,
}

impl OrderService {
    pub fn new(pool: PgPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    pub async fn get_order_by_id(
        &self,
        id: &Uuid,
        username: &String,
    ) -> anyhow::Result<Option<Order>, DatabaseError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(DatabaseError::transaction_failed)?;
        let optional_order = repository::order::select_by_id(&mut tx, id, username).await?;
        tx.commit()
            .await
            .map_err(DatabaseError::transaction_failed)?;
        Ok(optional_order)
    }

    pub async fn find_orders(
        &self,
        username: &String,
    ) -> anyhow::Result<Vec<Order>, DatabaseError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(DatabaseError::transaction_failed)?;
        let statuses = vec![
            OrderStatus::Open,
            OrderStatus::Completed,
            OrderStatus::Cancelled,
        ];
        let orders = repository::order::select_by_statuses(&mut tx, &username, &statuses).await?;
        tx.commit()
            .await
            .map_err(DatabaseError::transaction_failed)?;
        Ok(orders)
    }

    pub async fn create_order(
        &self,
        username: &String,
        request: &CreateOrderRequest,
    ) -> anyhow::Result<Option<Order>, DatabaseError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(DatabaseError::transaction_failed)?;
        let statuses = vec![OrderStatus::Open];
        let orders = repository::order::select_by_statuses(&mut tx, &username, &statuses).await?;
        if orders.is_empty() {
            let id = repository::order::insert(&mut tx, &username).await?;
            repository::order_item::insert(&mut tx, &id, &request.products).await?;
            let optional_order = repository::order::select_by_id(&mut tx, &id, &username).await?;
            tx.commit()
                .await
                .map_err(DatabaseError::transaction_failed)?;
            Ok(optional_order)
        } else {
            Err(DatabaseError::order_already_exists(OrderStatus::Open))
        }
    }

    pub async fn update_order(
        &self,
        id: &Uuid,
        username: &String,
        request: &UpdateOrderRequest,
    ) -> anyhow::Result<Option<Order>, DatabaseError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(DatabaseError::transaction_failed)?;
        repository::order_item::delete(&mut tx, &id, &username, &request.products).await?;
        repository::order_item::insert(&mut tx, &id, &request.products).await?;
        repository::order::update(&mut tx, &id, &username, &request.status).await?;
        let optional_order = repository::order::select_by_id(&mut tx, &id, &username).await?;
        tx.commit()
            .await
            .map_err(DatabaseError::transaction_failed)?;
        Ok(optional_order)
    }
}
