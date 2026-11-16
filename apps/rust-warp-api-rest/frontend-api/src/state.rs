use std::sync::Arc;

use reqwest::Client;

use crate::config::AppConfig;

/// Everything the proxy needs to call the backend.
pub struct AppState {
    pub client: Client,
    pub backend_url: String,
}

impl AppState {
    pub fn new(config: AppConfig) -> Arc<Self> {
        Arc::new(Self {
            client: Client::new(),
            backend_url: config.backend.url.trim_end_matches('/').to_string(),
        })
    }
}
