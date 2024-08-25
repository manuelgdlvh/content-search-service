use std::sync::Arc;

use axum::async_trait;
use sqlx::{Pool, Postgres, query, Row};

use crate::entities::recipe::Recipe;
use crate::infrastructure::di_container::{DB_POOL_DEP, DIContainer};

pub struct RecipeRepositoryImpl {
    db_pool: Arc<Pool<Postgres>>,
}

impl RecipeRepositoryImpl {
    pub fn new(di_container: &DIContainer) -> anyhow::Result<Self> {
        Ok(Self { db_pool: di_container.get::<Pool<Postgres>>(DB_POOL_DEP) })
    }
}

#[async_trait]
pub trait RecipeRepository {
    async fn find_recipes_by_lang_and_limit_offset(&self, lang: &str, limit: u64, offset: u64) -> anyhow::Result<Vec<Recipe>>;
}


#[async_trait]
impl RecipeRepository for RecipeRepositoryImpl {
    async fn find_recipes_by_lang_and_limit_offset(&self, lang: &str, limit: u64, offset: u64) -> anyhow::Result<Vec<Recipe>> {
        let rows = query("SELECT recipe_id, title FROM recipe.recipe_details WHERE language = $1 LIMIT $2 OFFSET $3")
            .bind(lang)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&*self.db_pool)
            .await?;

        let mut result = Vec::new();
        for row in rows {
            let id: u64 = row.try_get::<i64, &str>("recipe_id")? as u64;
            let name: String = row.try_get("title")?;
            result.push(Recipe::new(id, name));
        }

        Ok(result)
    }
}