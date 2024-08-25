use std::env;
use std::sync::LazyLock;

use serde::{Deserialize, Serialize};

use crate::config::database_config::DatabaseConfig;
use crate::config::indexer_runner_config::IndexerRunnerConfig;
use crate::config::logger_config::LoggerConfig;
use crate::config::server_config::ServerConfig;

pub mod server_config;
pub mod database_config;
mod indexer_runner_config;
mod logger_config;

pub const CONFIG_PATH_ENV: &str = "CONFIG_PATH";

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    Config::get().expect("Failed loading configuration")
});

#[derive(Deserialize, Serialize)]
pub struct Config {
    database: DatabaseConfig,
    server: ServerConfig,
    indexer_runner: IndexerRunnerConfig,
    logger: LoggerConfig,
}


impl Config {
    fn get() -> anyhow::Result<Self> {
        let config_path = env::var(CONFIG_PATH_ENV)?;
        let config = config::Config::builder()
            .add_source(config::File::with_name(config_path.as_str()))
            .build()?;
        Ok(config.try_deserialize::<Self>()?)
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn database_mut(&mut self) -> &mut DatabaseConfig {
        &mut self.database
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn indexer_runner(&self) -> &IndexerRunnerConfig {
        &self.indexer_runner
    }

    pub fn logger(&self) -> &LoggerConfig {
        &self.logger
    }
}
