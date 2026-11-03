use serde::{Deserialize, Serialize};

use crate::config::AppUserConfig;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub username: String,
}

impl User {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }
}

impl From<AppUserConfig> for User {
    fn from(config: AppUserConfig) -> Self {
        Self {
            name: config.name,
            username: config.username,
        }
    }
}
