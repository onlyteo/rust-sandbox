use std::sync::Arc;

use uuid::Uuid;

use crate::{
    error::RepositoryError,
    model::{CreateOrderRequest, Order, UpdateOrderRequest},
    repository::order::OrderRepository,
};

pub struct OrderService {
    repository: OrderRepository,
}

impl OrderService {
    pub fn new(repository: OrderRepository) -> Arc<Self> {
        Arc::new(Self { repository })
    }

    pub async fn get_order_by_id(
        &self,
        id: &Uuid,
        username: &str,
    ) -> Result<Option<Order>, RepositoryError> {
        self.repository.select_by_id(id, username)
    }

    pub async fn find_orders(&self, username: &str) -> Result<Vec<Order>, RepositoryError> {
        self.repository.select_by_username(username)
    }

    pub async fn create_order(
        &self,
        username: &str,
        request: &CreateOrderRequest,
    ) -> Result<Order, RepositoryError> {
        self.repository.insert(username, &request.products)
    }

    pub async fn update_order(
        &self,
        id: &Uuid,
        username: &str,
        request: &UpdateOrderRequest,
    ) -> Result<Option<Order>, RepositoryError> {
        self.repository
            .update(id, username, request.status, &request.products)
    }
}
