use std::sync::Arc;

use axum::async_trait;

use crate::infrastructure::di_container::{DIContainer, GAME_REPOSITORY_IMPL_DEP};
use crate::models::doc_details::DocDetails;
use crate::repositories::game_repository_impl::GameRepository;
use crate::services::doc_details_retriever::DocDetailsRetriever;

pub struct GameDocDetailsRetriever<T>
where
    T: GameRepository + Send + Sync,
{
    game_repository: Arc<T>,
}


impl<T> GameDocDetailsRetriever<T>
where
    T: GameRepository + Send + Sync + 'static,
{
    pub fn new(di_container: &DIContainer) -> Self {
        Self {
            game_repository: di_container.get::<T>(GAME_REPOSITORY_IMPL_DEP)
        }
    }
}

#[async_trait]
impl<T> DocDetailsRetriever for GameDocDetailsRetriever<T>
where
    T: GameRepository + Send + Sync,
{
    async fn retrieve(&self, lang: &str, limit: u64, offset: u64) -> anyhow::Result<Vec<DocDetails>> {
        let result = self.game_repository.find_games_by_lang_and_limit_offset(lang, limit, offset).await?
            .iter().map(|v| DocDetails::new(v.id(), v.title().to_string())).collect::<Vec<_>>();
        Ok(result)
    }
}