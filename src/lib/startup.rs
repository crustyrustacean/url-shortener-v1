// src/lib/startup.rs

// contains all the startup and configuration logic for the application

// dependencies
use crate::config::AppConfig;
use crate::routes::{get_redirect, health_check, post_shorten};
use crate::state::AppState;
use crate::telemetry::MakeRequestUuid;
use axum::{
    Router,
    http::HeaderName,
    routing::{get, post},
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_http::{
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

// struct type to represent the application
pub struct App {
    pub config: AppConfig,
    pub router: Router,
}

// methods to build the application
impl App {
    // create a new application instance
    pub fn new(config: AppConfig, state: AppState) -> Self {
        let router = Self::build_router(state);
        Self { config, router }
    }

    // build the application router with all routes and middleware layers
    pub fn build_router(state: AppState) -> Router {
        // define the tracing layer
        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new()
                    .include_headers(true)
                    .level(Level::INFO),
            )
            .on_response(DefaultOnResponse::new().include_headers(true));
        let x_request_id = HeaderName::from_static("x-request-id");

        let governor_conf = GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .finish()
            .unwrap();

        // build the application router
        Router::new()
            .route("/health_check", get(health_check))
            .route("/{id}", get(get_redirect))
            .route("/", post(post_shorten))
            .with_state(state)
            .layer(GovernorLayer::new(governor_conf))
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::new(
                        x_request_id.clone(),
                        MakeRequestUuid,
                    ))
                    .layer(trace_layer)
                    .layer(PropagateRequestIdLayer::new(x_request_id)),
            )
    }

    /// run the application until stopped (utility function to faciliate local integration testing)
    pub async fn run_until_stopped(self, listener: TcpListener) -> Result<(), anyhow::Error> {
        axum::serve(listener, self.router).await?;
        Ok(())
    }
}
