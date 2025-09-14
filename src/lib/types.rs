// src/lib/types.rs

// dependencies
use shuttle_runtime::{CustomError, Error, SecretStore};
use sqlx::PgPool;

// type declaration
pub type AppServicePool = PgPool;
pub type ShuttleCustomError = CustomError;
pub type ShuttleError = Error;
pub type ShuttleSecretStore = SecretStore;
