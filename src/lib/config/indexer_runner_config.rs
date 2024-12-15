use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct IndexerRunnerConfig {
    batch_size: u64,
    interval: u64,
    await_initialized: bool
}

impl IndexerRunnerConfig {
    pub fn batch_size(&self) -> u64 {
        self.batch_size
    }

    pub fn interval(&self) -> u64 {
        self.interval
    }

    pub fn await_initialized(&self) -> bool {
        self.await_initialized
    }
}