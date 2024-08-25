use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ServerConfig {
    host: String,
    port: u16,
}

impl ServerConfig {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}
