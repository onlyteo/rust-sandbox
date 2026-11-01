use std::sync::Arc;

use crate::service::{order::OrderService, product::ProductService};

#[derive(Clone)]
pub struct AppState {
    orders_service: Arc<OrderService>,
    product_service: Arc<ProductService>,
}

impl AppState {
    pub fn new(
        orders_service: Arc<OrderService>,
        product_service: Arc<ProductService>,
    ) -> Arc<Self> {
        Arc::new(Self {
            orders_service,
            product_service,
        })
    }

    pub fn orders_service(&self) -> Arc<OrderService> {
        self.orders_service.clone()
    }

    pub fn product_service(&self) -> Arc<ProductService> {
        self.product_service.clone()
    }
}
