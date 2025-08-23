// src/lib/routes/redirect.rs

// endpoint handler which provides the shortened URL to redirect to

// dependencies
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_macros::debug_handler;
use sqlx::{Error, PgPool};
use tracing::instrument;

// redirect endpoint handler
#[debug_handler]
#[instrument(name = "redirect" skip(state))]
pub async fn get_redirect(
    State(state): State<PgPool>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let url: (String,) = sqlx::query_as("SELECT url FROM urls WHERE id = $1")
        .bind(id)
        .fetch_one(&state)
        .await
        .map_err(|e| match e {
            Error::RowNotFound => {
                tracing::error!("shortened URL not found in the database...");
                StatusCode::NOT_FOUND
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;
    tracing::info!("shortened URL retreived, redirecting...");
    Ok(Redirect::permanent(&url.0))
}
