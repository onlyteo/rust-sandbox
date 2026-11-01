use std::sync::Arc;

use crate::{client::HttpClient, model::User};

#[derive(Clone)]
pub struct AppState {
    user: User,
    http_client: Arc<HttpClient>,
}

impl AppState {
    pub fn new(user: User, http_client: Arc<HttpClient>) -> Arc<Self> {
        Arc::new(Self { user, http_client })
    }

    pub fn user(&self) -> User {
        self.user.clone()
    }

    pub fn http_client(&self) -> Arc<HttpClient> {
        self.http_client.clone()
    }
}
