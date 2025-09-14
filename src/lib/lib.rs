// src/lib/lib.rs

// this is the library crate for the url-shortener-v1 project

// module declarations
pub mod config;
pub mod errors;
pub mod middleware;
pub mod response;
pub mod routes;
pub mod service;
pub mod state;
pub mod telemetry;
pub mod types;

// re-exports
pub use config::*;
pub use errors::*;
pub use middleware::*;
pub use response::*;
pub use service::*;
pub use state::*;
pub use telemetry::*;
pub use types::*;
