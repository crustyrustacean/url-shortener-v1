// src/lib/state.rs

// dependencies
use sqlx::PgPool;

// struct type to represent the application state
#[derive(Clone, Default)]
pub struct AppState {
    pub pool: Option<PgPool>,
}

// methods to build the configuration
impl AppState {
    // Create a new application state instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool: Some(pool) }
    }
}
