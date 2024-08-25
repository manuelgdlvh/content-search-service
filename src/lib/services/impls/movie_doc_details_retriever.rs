use std::sync::Arc;

use axum::async_trait;

use crate::infrastructure::di_container::{DIContainer, MOVIE_REPOSITORY_IMPL_DEP};
use crate::models::doc_details::DocDetails;
use crate::repositories::movie_repository_impl::MovieRepository;
use crate::services::doc_details_retriever::DocDetailsRetriever;

pub struct MovieDocDetailsRetriever<T>
where
    T: MovieRepository + Send + Sync,
{
    movie_repository: Arc<T>,
}


impl<T> MovieDocDetailsRetriever<T>
where
    T: MovieRepository + Send + Sync + 'static,
{
    pub fn new(di_container: &DIContainer) -> Self {
        Self {
            movie_repository: di_container.get::<T>(MOVIE_REPOSITORY_IMPL_DEP)
        }
    }
}

#[async_trait]
impl<T> DocDetailsRetriever for MovieDocDetailsRetriever<T>
where
    T: MovieRepository + Send + Sync,
{
    async fn retrieve(&self, lang: &str, limit: u64, offset: u64) -> anyhow::Result<Vec<DocDetails>> {
        let result = self.movie_repository.find_movies_by_lang_and_limit_offset(lang, limit, offset).await?
            .iter().map(|v| DocDetails::new(v.id(), v.title().to_string())).collect::<Vec<_>>();
        Ok(result)
    }
}