use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LoggerConfig {
    enabled: bool,
    level: String,
}

impl LoggerConfig {
    pub fn enabled(&self) -> bool {
        self.enabled
    }


    pub fn level(&self) -> LevelFilter {
        match self.level.as_str() {
            "TRACE" => LevelFilter::Trace,
            "DEBUG" => LevelFilter::Debug,
            "INFO" => LevelFilter::Info,
            "ERROR" => LevelFilter::Error,
            "WARN" => LevelFilter::Warn,
            _ => { panic!("Log level filter is invalid, check for errors") }
        }
    }
}