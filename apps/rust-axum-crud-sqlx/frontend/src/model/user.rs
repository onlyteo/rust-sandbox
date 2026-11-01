use serde::{Deserialize, Serialize};

use crate::config::AppUserConfig;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub username: String,
}

impl From<AppUserConfig> for User {
    fn from(config: AppUserConfig) -> Self {
        Self {
            name: config.name,
            username: config.username,
        }
    }
}
