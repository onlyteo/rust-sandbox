use std::time::Duration;

use uuid::Uuid;

use crate::config::ExternalApiConfig;
use crate::error::ClientError;
use crate::model::ExternalUser;

#[derive(Debug, Clone)]
pub struct ExternalApiClient {
    client: reqwest::Client,
    base_url: String,
}

impl ExternalApiClient {
    pub fn new(config: &ExternalApiConfig) -> Result<Self, ClientError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.request_timeout_secs))
            .build()
            .map_err(ClientError::build)?;
        Ok(Self {
            client,
            base_url: config.base_url.trim_end_matches('/').to_string(),
        })
    }

    pub async fn get_user(&self, id: Uuid) -> Result<ExternalUser, ClientError> {
        let response = self
            .client
            .get(format!("{}/users/{}", self.base_url, id))
            .send()
            .await
            .map_err(|e| ClientError::request(id, e))?;
        let status = response.status();
        if !status.is_success() {
            return Err(ClientError::unexpected_status(id, status.as_u16()));
        }
        response
            .json::<ExternalUser>()
            .await
            .map_err(|e| ClientError::request(id, e))
    }
}
