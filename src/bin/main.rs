// src/bin/main.rs

// binary crate for the url-shortener-v1 project

// dependencies
use url_shortener_v1_lib::config::AppConfig;
use url_shortener_v1_lib::service::AppService;
use url_shortener_v1_lib::state::AppState;
use url_shortener_v1_lib::telemetry::{get_subscriber, init_subscriber};
use url_shortener_v1_lib::types::{AppServicePool, ShuttleCustomError, ShuttleSecretStore};

// main function
#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: AppServicePool,
     #[shuttle_runtime::Secrets] secrets: ShuttleSecretStore
) -> Result<AppService, shuttle_runtime::Error> {
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
            ShuttleCustomError::new(err).context(msg)
        })?;

    // Load configuration
    tracing::info!("Loading service configuration...");
    let app_config = AppConfig::try_from(&secrets)?;

    // Build the application state
    tracing::info!("Building the app state...");
    let app_state = AppState::new(pool);

    // Initialize the application
    tracing::info!("Initializing the app...");
    let app_service = AppService::new(app_config, app_state);

    Ok(app_service)
}
