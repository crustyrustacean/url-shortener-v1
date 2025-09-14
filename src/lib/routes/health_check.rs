// src/lib/routes/health_check.rs

// dependencies
use crate::response::ApiResponse;

// health_check handler
pub async fn health_check() -> ApiResponse<()> {
    ApiResponse::success(())
}
