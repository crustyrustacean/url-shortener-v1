// src/lib/state.rs

// dependencies
use crate::config::AppConfig;
use sqlx::PgPool;

// struct type to represent the application state
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub pool: PgPool,
}

// methods to build the configuration
impl AppState {
    // Create a new application state instance
    pub fn new(config: AppConfig, pool: PgPool) -> Self {
        Self { config, pool }
    }
}
