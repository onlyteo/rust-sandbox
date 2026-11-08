use serde::{Deserialize, Serialize};

use crate::config::AppConfig;

#[derive(Clone, Serialize, Deserialize)]
pub struct BootstrapPrincipal {
    pub name: String,
    pub username: String,
    pub password: String,
}

impl BootstrapPrincipal {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn username(&self) -> String {
        self.username.clone()
    }

    pub fn password(&self) -> String {
        self.password.clone()
    }
}

impl From<AppConfig> for BootstrapPrincipal {
    fn from(config: AppConfig) -> Self {
        Self {
            name: config.user.name,
            username: config.user.username,
            password: config.user.password,
        }
    }
}
