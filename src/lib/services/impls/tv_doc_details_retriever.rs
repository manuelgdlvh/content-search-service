use std::sync::Arc;

use axum::async_trait;

use crate::infrastructure::di_container::{DIContainer, TV_REPOSITORY_IMPL_DEP};
use crate::models::doc_details::DocDetails;
use crate::repositories::tv_repository_impl::TvRepository;
use crate::services::doc_details_retriever::DocDetailsRetriever;

pub struct TvDocDetailsRetriever<T>
where
    T: TvRepository + Send + Sync,
{
    tv_repository: Arc<T>,
}


impl<T> TvDocDetailsRetriever<T>
where
    T: TvRepository + Send + Sync + 'static,
{
    pub fn new(di_container: &DIContainer) -> Self {
        Self {
            tv_repository: di_container.get::<T>(TV_REPOSITORY_IMPL_DEP)
        }
    }
}

#[async_trait]
impl<T> DocDetailsRetriever for TvDocDetailsRetriever<T>
where
    T: TvRepository + Send + Sync,
{
    async fn retrieve(&self, lang: &str, limit: u64, offset: u64) -> anyhow::Result<Vec<DocDetails>> {
        let result = self.tv_repository.find_tvs_by_lang_and_limit_offset(lang, limit, offset).await?
            .iter().map(|v| DocDetails::new(v.id(), v.title().to_string())).collect::<Vec<_>>();
        Ok(result)
    }
}