use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct IndexerRunnerConfig {
    batch_size: u64,
    interval: u64,
    wait_until_index: bool
}

impl IndexerRunnerConfig {
    pub fn batch_size(&self) -> u64 {
        self.batch_size
    }

    pub fn interval(&self) -> u64 {
        self.interval
    }

    pub fn wait_until_index(&self) -> bool {
        self.wait_until_index
    }
}