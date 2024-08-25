use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchResponse<> {
    results: Vec<u64>,
}

impl SearchResponse {
    pub fn results(&self) -> &[u64] {
        &self.results[..]
    }

    pub fn new(results: Vec<u64>) -> Self {
        Self { results }
    }
}