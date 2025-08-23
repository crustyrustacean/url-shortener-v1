// src/lib/routes/shorten.rs

// endpoint handler which takes a URL to shorten, shortens it, and inserts it into the database

// dependencies
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::{headers::Host, TypedHeader};
use axum_macros::debug_handler;
use sqlx::PgPool;
use tracing::instrument;
use url::Url;

// redirect endpoint handler
#[debug_handler]
#[instrument(name = "shorten" skip(state))]
pub async fn post_shorten(
    State(state): State<PgPool>,
    TypedHeader(header): TypedHeader<Host>,
    url: String,
) -> Result<impl IntoResponse, StatusCode> {
    let id = &nanoid::nanoid!(6);
    let p_url = Url::parse(&url).map_err(|_| {
        tracing::error!("Unable to parse URL");
        StatusCode::UNPROCESSABLE_ENTITY
    })?;
    let host = header.hostname();
    sqlx::query("INSERT INTO urls (id, url) VALUES ($1, $2)")
        .bind(id)
        .bind(p_url.as_str())
        .execute(&state)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let response_body = format!("https://{}/{}\n", host, id);

    tracing::info!("URL shortened and saved successfully...");
    Ok((StatusCode::OK, response_body).into_response())
}
