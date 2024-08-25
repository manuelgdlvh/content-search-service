use std::{env, fs, mem, thread};
use std::borrow::Cow;
use std::cell::Cell;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::process::{Command, ExitStatus, Output};
use std::sync::{LazyLock, mpsc, Mutex};
use std::time::Duration;

use ctor::{ctor, dtor};
use reqwest::{Client, StatusCode};
use sqlx::postgres;
use testcontainers::{Container, GenericImage, Image, ImageExt};
use testcontainers::core::{ContainerPort, ExecCommand, IntoContainerPort, Mount, WaitFor};
use testcontainers::core::logs::LogSource;
use testcontainers::core::wait::{HealthWaitStrategy, LogWaitStrategy};
use testcontainers::core::WaitFor::Healthcheck;
use testcontainers::runners::SyncRunner;
use tokio::runtime;
use tokio::sync::oneshot;
use tokio::time::sleep;

use lib::config::{CONFIG, Config, CONFIG_PATH_ENV};
use lib::handlers::requests::search_request::SearchRequest;
use lib::infrastructure::app_runner::AppRunner;
use lib::infrastructure::di_container::DIContainer;
use lib::infrastructure::http_server::HttpServer;
use lib::services::indexer_runner::IndexerRunner;

pub const CONFIG_FOLDER_PATH: &'static str = "/tests/integration/config";
pub const INIT_SQL_PATH: &'static str = "/db/init.sql";
pub const CONFIG_FILE_PATH: &'static str = "/Config-Test.toml";


static POSTGRES_CONTAINER: LazyLock<Container<GenericImage>> = LazyLock::new(|| {
    let current_dir = env::current_dir().unwrap();

    GenericImage::new("postgres", "latest")
        .with_wait_for(WaitFor::Log(LogWaitStrategy::new(LogSource::StdOut, "PostgreSQL init process complete; ready for start up.")))
        .with_wait_for(WaitFor::Duration { length: Duration::from_secs(1) })
        .with_exposed_port(5432.tcp())
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .with_env_var("POSTGRES_DB", "postgres")
        .with_mount(Mount::bind_mount(format!("{}{CONFIG_FOLDER_PATH}{INIT_SQL_PATH}", current_dir.to_str().expect("Current Dir retrieved"))
                                      , "/docker-entrypoint-initdb.d/init.sql"))
        .start().expect("Postgres started")
});


#[ctor]
fn init() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    let current_dir = env::current_dir().unwrap();
    let config_path = format!("{}{CONFIG_FOLDER_PATH}{CONFIG_FILE_PATH}", current_dir.to_str().unwrap());
    env::set_var(CONFIG_PATH_ENV, &config_path);

    let (tx, rx) = oneshot::channel();

    let pg_container_port = (&*POSTGRES_CONTAINER).get_host_port_ipv4(5432).expect("Postgres Container Port retrieved");
    let pg_container_host = (&*POSTGRES_CONTAINER).get_host().expect("Postgres Container Port retrieved").to_string();
    change_db_config(&config_path, pg_container_host, pg_container_port).expect("Postgres Port changed");

    thread::spawn(move || {
        runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let _ = AppRunner::run(Some(tx)).await;
            });
    });

    log::info!("waiting to web server to be fully initialized...");
    rx.blocking_recv().expect("Received started signal");
}

fn change_db_config(config_path: &str, host: String, port: u16) -> anyhow::Result<()> {
    let content = fs::read_to_string(config_path)?;

    let mut config: Config = toml::from_str(&content)?;
    config.database_mut().set_port(port);
    config.database_mut().set_host(host);

    let new_content = toml::to_string(&config)?;
    fs::write(config_path, new_content)?;

    Ok(())
}

#[dtor]
fn destroy() {
    (&*POSTGRES_CONTAINER).stop().unwrap();
}


pub async fn check_post<I, O>(endpoint: &str, input: &I, status_code: StatusCode) -> anyhow::Result<O>
where
    I: serde::ser::Serialize,
    O: for<'de> serde::Deserialize<'de>,
{
    let client = Client::new();
    let response = client
        .post(format!("http://localhost:8080{endpoint}"))
        .json(input)
        .send()
        .await?;
    assert_eq!(response.status(), status_code);

    let result: O = response.json().await?;

    Ok(result)
}

pub async fn no_output_check_post<I>(endpoint: &str, input: &I, status_code: StatusCode) -> anyhow::Result<()>
where
    I: serde::ser::Serialize,
{
    let client = Client::new();
    let response = client
        .post(format!("http://localhost:8080{endpoint}"))
        .json(input)
        .send()
        .await?;
    assert_eq!(response.status(), status_code);
    Ok(())
}







