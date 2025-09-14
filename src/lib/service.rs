// src/lib/startup.rs

// contains all the startup and configuration logic for the application

// dependencies
use crate::middleware::check_api_key;
use crate::routes::{get_redirect, health_check, post_shorten};
use crate::state::AppState;
use crate::telemetry::MakeRequestUuid;
use axum::{
    Router,
    http::HeaderName,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use shuttle_runtime::Service;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{PropagateRequestIdLayer, SetRequestIdLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

// struct type to represent the application
pub struct AppService {
    pub router: Router,
}

// methods to build the application
impl AppService {
    // create a new application instance
    pub fn new(state: AppState) -> Self {
        let router = Self::build_router(state);
        Self { router }
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

        let secure_api = Router::new()
            .route("/{id}", get(get_redirect))
            .route("/", post(post_shorten))
            .route_layer(from_fn_with_state(state.clone(), check_api_key));

        // build the application router
        Router::new()
            .route("/health_check", get(health_check))
            .merge(secure_api)
            .with_state(state)
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

    // run the application until stopped (utility function to faciliate local integration testing)
    pub async fn run_until_stopped(self, listener: TcpListener) -> Result<(), anyhow::Error> {
        axum::serve(
            listener,
            self.router
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        Ok(())
    }
}

// implement the Shuttle `Service` trait for the `AppService` type
#[shuttle_runtime::async_trait]
impl Service for AppService {
    async fn bind(mut self, addr: SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = self.router;

        let listener = TcpListener::bind(addr).await?;
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

        Ok(())
    }
}
