use std::sync::{Mutex, MutexGuard};

use chrono::Utc;
use uuid::Uuid;

use crate::{
    error::RepositoryError,
    model::{Order, OrderProduct, OrderStatus},
};

#[derive(Default)]
pub struct OrderRepository {
    orders: Mutex<Vec<Order>>,
}

impl OrderRepository {
    pub fn new() -> Self {
        Self::default()
    }

    fn lock(&self) -> Result<MutexGuard<'_, Vec<Order>>, RepositoryError> {
        self.orders
            .lock()
            .map_err(|_| RepositoryError::lock_poisoned())
    }

    pub fn select_by_username(&self, username: &str) -> Result<Vec<Order>, RepositoryError> {
        let orders = self.lock()?;
        let mut found: Vec<Order> = orders
            .iter()
            .filter(|o| o.username == username)
            .cloned()
            .collect();
        found.sort_by_key(|o| o.timestamp);
        Ok(found)
    }

    pub fn select_by_id(
        &self,
        id: &Uuid,
        username: &str,
    ) -> Result<Option<Order>, RepositoryError> {
        let orders = self.lock()?;
        Ok(orders
            .iter()
            .find(|o| o.id == *id && o.username == username)
            .cloned())
    }

    pub fn insert(
        &self,
        username: &str,
        products: &[OrderProduct],
    ) -> Result<Order, RepositoryError> {
        let mut orders = self.lock()?;
        let has_open = orders
            .iter()
            .any(|o| o.username == username && o.status == OrderStatus::Open);
        if has_open {
            return Err(RepositoryError::order_already_exists(OrderStatus::Open));
        }
        let order = Order {
            id: Uuid::new_v4(),
            username: username.to_string(),
            products: sync_products(&[], products),
            status: OrderStatus::Open,
            timestamp: Utc::now(),
        };
        orders.push(order.clone());
        Ok(order)
    }

    pub fn update(
        &self,
        id: &Uuid,
        username: &str,
        status: OrderStatus,
        products: &[OrderProduct],
    ) -> Result<Option<Order>, RepositoryError> {
        let mut orders = self.lock()?;
        Ok(orders
            .iter_mut()
            .find(|o| o.id == *id && o.username == username)
            .map(|order| {
                order.products = sync_products(&order.products, products);
                order.status = status;
                order.clone()
            }))
    }
}

fn sync_products(existing: &[OrderProduct], requested: &[OrderProduct]) -> Vec<OrderProduct> {
    let mut result: Vec<OrderProduct> = existing
        .iter()
        .filter(|e| requested.iter().any(|r| r.id == e.id))
        .cloned()
        .collect();
    for item in requested {
        match result.iter_mut().find(|p| p.id == item.id) {
            Some(p) => p.quantity = item.quantity,
            None => result.push(item.clone()),
        }
    }
    result
}
