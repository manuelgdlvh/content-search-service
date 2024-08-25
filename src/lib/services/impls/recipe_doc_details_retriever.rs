use std::sync::Arc;

use axum::async_trait;

use crate::infrastructure::di_container::{DIContainer, RECIPE_REPOSITORY_IMPL_DEP};
use crate::models::doc_details::DocDetails;
use crate::repositories::recipe_repository_impl::RecipeRepository;
use crate::services::doc_details_retriever::DocDetailsRetriever;

pub struct RecipeDocDetailsRetriever<T>
where
    T: RecipeRepository + Send + Sync,
{
    recipe_repository: Arc<T>,
}


impl<T> RecipeDocDetailsRetriever<T>
where
    T: RecipeRepository + Send + Sync + 'static,
{
    pub fn new(di_container: &DIContainer) -> Self {
        Self {
            recipe_repository: di_container.get::<T>(RECIPE_REPOSITORY_IMPL_DEP)
        }
    }
}

#[async_trait]
impl<T> DocDetailsRetriever for RecipeDocDetailsRetriever<T>
where
    T: RecipeRepository + Send + Sync,
{
    async fn retrieve(&self, lang: &str, limit: u64, offset: u64) -> anyhow::Result<Vec<DocDetails>> {
        let result = self.recipe_repository.find_recipes_by_lang_and_limit_offset(lang, limit, offset).await?
            .iter().map(|v| DocDetails::new(v.id(), v.title().to_string())).collect::<Vec<_>>();
        Ok(result)
    }
}