use std::sync::Arc;

use crate::{
    model::BootstrapPrincipal,
    service::{order::OrderService, product::ProductService},
};

#[derive(Clone)]
pub struct AppState {
    principal: Arc<BootstrapPrincipal>,
    orders_service: Arc<OrderService>,
    product_service: Arc<ProductService>,
}

impl AppState {
    pub fn new(
        principal: Arc<BootstrapPrincipal>,
        orders_service: Arc<OrderService>,
        product_service: Arc<ProductService>,
    ) -> Arc<Self> {
        Arc::new(Self {
            principal: principal,
            orders_service,
            product_service,
        })
    }

    pub fn principal(&self) -> Arc<BootstrapPrincipal> {
        self.principal.clone()
    }

    pub fn orders_service(&self) -> Arc<OrderService> {
        self.orders_service.clone()
    }

    pub fn product_service(&self) -> Arc<ProductService> {
        self.product_service.clone()
    }
}
