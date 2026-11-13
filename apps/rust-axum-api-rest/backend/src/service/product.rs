use std::sync::Arc;

use crate::{error::RepositoryError, model::Product, repository::product::ProductRepository};

pub struct ProductService {
    repository: ProductRepository,
}

impl ProductService {
    pub fn new(repository: ProductRepository) -> Arc<Self> {
        Arc::new(Self { repository })
    }

    pub async fn find_products(&self) -> Result<Vec<Product>, RepositoryError> {
        Ok(self.repository.select())
    }
}
