// tests/api/helpers.rs

// types and functions used across all integration tests

// dependencies
use anyhow::{Context, Result, anyhow};
use reqwest::Client;
use shuttle_common::secrets::Secret;
use sqlx::{
    Connection, Executor, PgConnection, PgPool,
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
};
use std::collections::BTreeMap;
use std::env::var;
use std::fs;
use std::io::{sink, stdout};
use std::sync::LazyLock;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};
use tokio::net::TcpListener;
use tokio::sync::OnceCell;
use toml::Value;
use url_shortener_v1_lib::config::AppConfig;
use url_shortener_v1_lib::service::AppService;
use url_shortener_v1_lib::state::AppState;
use url_shortener_v1_lib::telemetry::{get_subscriber, init_subscriber};
use url_shortener_v1_lib::types::ShuttleSecretStore;
use uuid::Uuid;

// static constant which creates one instance of tracing
static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, sink);
        init_subscriber(subscriber);
    }
});

// static constant which creates one instance of the test database container
static POSTGRES_CONTAINER: OnceCell<ContainerAsync<Postgres>> = OnceCell::const_new();

// function to create the Postgres container
async fn get_postgres_container() -> &'static ContainerAsync<Postgres> {
    POSTGRES_CONTAINER
        .get_or_init(|| async {
            Postgres::default()
                .start()
                .await
                .expect("Failed to start Postgres testcontainer")
        })
        .await
}

// struct type to represent the test database settings
#[derive(Clone, Debug)]
struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

// methods for the DatabaseSettings type
impl DatabaseSettings {
    pub fn new(host_port: u16) -> Self {
        DatabaseSettings {
            username: "postgres".into(),
            password: "postgres".into(),
            port: host_port,
            host: "localhost".into(),
            database_name: Uuid::new_v4().to_string(),
        }
    }

    pub fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(PgSslMode::Disable)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

// function to configure the testing database
async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(None)
        .max_lifetime(None)
        .connect_with(config.with_db())
        .await
        .expect("Unable to create database connection pool.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}

// Load Shuttle secrets for tests from Secrets.dev.toml (preferred) or Secrets.toml.
fn load_test_secret_store() -> Result<ShuttleSecretStore> {
    let path = if fs::metadata("Secrets.dev.toml").is_ok() {
        "Secrets.dev.toml"
    } else if fs::metadata("Secrets.toml").is_ok() {
        "Secrets.toml"
    } else {
        return Err(anyhow!(
            "Neither Secrets.dev.toml nor Secrets.toml found in project root"
        ));
    };

    let txt = fs::read_to_string(path).with_context(|| format!("Reading {}", path))?;
    let val: Value = toml::from_str(&txt).with_context(|| format!("Parsing {}", path))?;
    let table = val
        .as_table()
        .ok_or_else(|| anyhow!("Root of {} must be a TOML table", path))?;

    let mut map: BTreeMap<String, Secret<String>> = BTreeMap::new();
    for (k, v) in table {
        let s = v
            .as_str()
            .ok_or_else(|| anyhow!("Secret {k} must be a string in {}", path))?;
        map.insert(k.clone(), Secret::new(s.to_owned()));
    }

    Ok(ShuttleSecretStore::new(map))
}

// struct type which models a test application
#[allow(dead_code)]
pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub pool: PgPool,
    pub client: Client,
    pub api_key: Uuid,
}

// helper function which builds and returns a test application
pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    // create the database test container
    let container = get_postgres_container().await;

    // get the container port
    let host_port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("Unable to obtain a host port for the database test container.");

    // build the test database configuration
    let db_config = DatabaseSettings::new(host_port);

    // configure and return a database connection pool
    let pool = configure_database(&db_config).await;

    // load secrets and build AppConfig
    let secrets = load_test_secret_store().expect("Failed to load Shuttle secrets for tests.");
    let app_config =
        AppConfig::try_from(&secrets).expect("Failed to build AppConfig from Shuttle secrets.");

    // build the AppState using AppConfig and the created test database pool
    let app_state = AppState::new(app_config, pool.clone());

    // get the api_key from config
    let api_key = *app_state.config.api_key();

    // create the test application
    let app_service = AppService::new(app_state);

    // create a listener
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind port.");

    // get the address and port from the listener
    let addr = listener
        .local_addr()
        .expect("Unable to obtain the address of the listener.");
    let port = addr.port();

    // spawn the application
    tokio::spawn(app_service.run_until_stopped(listener));

    // build a client to make requests
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        port,
        pool,
        client,
        api_key,
    }
}
