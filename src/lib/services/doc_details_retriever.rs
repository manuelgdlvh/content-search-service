use axum::async_trait;

use crate::models::doc_details::DocDetails;

#[async_trait]
pub trait DocDetailsRetriever {
    async fn retrieve(&self, lang: &str, limit: u64, offset: u64) -> anyhow::Result<Vec<DocDetails>>;
}
