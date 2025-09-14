// src/lib/config.rs

// dependencies
use crate::types::{ShuttleCustomError, ShuttleSecretStore};
use anyhow::{Context, Result, anyhow};
use url::Url;

// struct type to represent the application configuration
pub struct AppConfig {
    base_url: Url,
}

// methods to build the configuration
impl AppConfig {
    // accessor method for the `base_url` field
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
}

// implement the TryFrom trait for the AppConfig type
impl TryFrom<&ShuttleSecretStore> for AppConfig {
    type Error = ShuttleCustomError;

    fn try_from(secrets: &ShuttleSecretStore) -> Result<Self> {
        let raw = secrets
            .get("APP_BASEURL")
            .ok_or_else(|| anyhow!("Missing required secret: APP_BASEURL"))?;

        let base_url = Url::parse(&raw)
            .with_context(|| format!("APP_BASEURL='{}' is not a valid URL", raw))?;

        Ok(Self { base_url })
    }
}
