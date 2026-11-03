use std::sync::Arc;

use crate::{
    model::User,
    service::{order::OrderService, product::ProductService},
};

#[derive(Clone)]
pub struct AppState {
    user: Arc<User>,
    orders_service: Arc<OrderService>,
    product_service: Arc<ProductService>,
}

impl AppState {
    pub fn new(
        user: Arc<User>,
        orders_service: Arc<OrderService>,
        product_service: Arc<ProductService>,
    ) -> Arc<Self> {
        Arc::new(Self {
            user,
            orders_service,
            product_service,
        })
    }

    pub fn user(&self) -> Arc<User> {
        self.user.clone()
    }

    pub fn orders_service(&self) -> Arc<OrderService> {
        self.orders_service.clone()
    }

    pub fn product_service(&self) -> Arc<ProductService> {
        self.product_service.clone()
    }
}
