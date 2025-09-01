// src/lib/config.rs

// struct type to represent the application configuration
pub struct AppConfig {}

// methods to build the configuration
impl AppConfig {
    // constructor for AppConfig
    pub fn new() -> Self {
        Self {}
    }
}

// implement the default trait for AppConfig
impl Default for AppConfig {
    // provide a default implementation that calls the constructor
    fn default() -> Self {
        Self::new()
    }
}
