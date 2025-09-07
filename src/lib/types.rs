// src/lib/types.rs

// dependencies
use shuttle_runtime::CustomError;
use sqlx::PgPool;

// type declaration
pub type AppServicePool = PgPool;
pub type ShuttleError = CustomError;
