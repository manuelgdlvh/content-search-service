use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DatabaseConfig<> {
    db_name: String,
    username: String,
    password: String,
    host: String,
    port: u16,
    max_connections: u32,
    min_connections: u32,
}

impl DatabaseConfig<> {
    pub fn db_name(&self) -> &str {
        &self.db_name
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }


    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }

    pub fn min_connections(&self) -> u32 {
        self.min_connections
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
}

