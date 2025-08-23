// src/lib/lib.rs

// this is the library crate for the url-shortener-v1 project

// module declarations
pub mod routes;
pub mod startup;
pub mod telemetry;

// re-exports
pub use startup::*;
pub use telemetry::*;
