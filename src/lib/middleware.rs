// src/lib/middleware.rs

// dependencies
use crate::state::AppState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn check_api_key(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let api_key = state.config.api_key().to_string();

    let provided_api_key = request
        .headers()
        .get("x-api-key")
        .and_then(|h| h.to_str().ok());

    match provided_api_key {
        Some(k) if k == api_key => next.run(request).await,
        _ => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    }
}
