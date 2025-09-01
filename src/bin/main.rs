// src/bin/main.rs

// binary crate for the url-shortener-v1 project

// dependencies
use shuttle_runtime::CustomError;
use sqlx::PgPool;
use url_shortener_v1_lib::config::AppConfig;
use url_shortener_v1_lib::startup::App;
use url_shortener_v1_lib::state::AppState;
use url_shortener_v1_lib::telemetry::{get_subscriber, init_subscriber};

// main function
#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    // initialize tracing
    let subscriber = get_subscriber("url-shortener-v1".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // run the database migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|err| {
            let msg = format!("Unable to run the database migrations: {}", err);
            CustomError::new(err).context(msg)
        })?;

    // Load configuration
    tracing::info!("Loading app configuration...");
    let app_config = AppConfig::default();

    // Build the application state
    tracing::info!("Building the app state...");
    let app_state = AppState::new(pool);

    // Initialize the application
    tracing::info!("Initializing the app...");
    let app = App::new(app_config, app_state);

    Ok(app.router.into())
}
