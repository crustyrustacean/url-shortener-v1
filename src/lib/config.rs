// src/lib/config.rs

// dependencies
use crate::types::{ShuttleCustomError, ShuttleSecretStore};
use anyhow::{Context, Result, anyhow};
use url::Url;
use uuid::Uuid;

// struct type to represent the application configuration
#[derive(Clone, Debug)]
pub struct AppConfig {
    base_url: Url,
    api_key: Uuid,
}

// methods to build the configuration
impl AppConfig {
    // accessor method for the `base_url` field
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    // accessor for the `api_key` field
    pub fn api_key(&self) -> &Uuid {
        &self.api_key
    }
}

// implement the TryFrom trait for the AppConfig type
impl TryFrom<&ShuttleSecretStore> for AppConfig {
    type Error = ShuttleCustomError;

    fn try_from(secrets: &ShuttleSecretStore) -> Result<Self> {
        let raw_base_url = secrets
            .get("APP_BASEURL")
            .ok_or_else(|| anyhow!("Missing required secret: APP_BASEURL"))?;

        let base_url = Url::parse(&raw_base_url)
            .with_context(|| format!("APP_BASEURL='{}' is not a valid URL", raw_base_url))?;

        let raw_api_key = secrets
            .get("API_KEY")
            .ok_or_else(|| anyhow!("Missing required secret: API_KEY"))?;

        let api_key = Uuid::parse_str(&raw_api_key)
            .with_context(|| format!("API_KEY='{}' is not a valid UUID", raw_api_key))?;

        Ok(Self { base_url, api_key })
    }
}
