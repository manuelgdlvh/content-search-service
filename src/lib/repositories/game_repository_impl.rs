use std::sync::Arc;

use axum::async_trait;
use sqlx::{Pool, Postgres, query, Row};
use crate::entities::game::Game;
use crate::infrastructure::di_container::{DB_POOL_DEP, DIContainer};

pub struct GameRepositoryImpl {
    db_pool: Arc<Pool<Postgres>>,
}

impl GameRepositoryImpl {
    pub fn new(di_container: &DIContainer) -> anyhow::Result<Self> {
        Ok(Self { db_pool: di_container.get::<Pool<Postgres>>(DB_POOL_DEP) })
    }
}

#[async_trait]
pub trait GameRepository {
    async fn find_games_by_lang_and_limit_offset(&self, lang: &str, limit: u64, offset: u64) -> anyhow::Result<Vec<Game>>;
}


#[async_trait]
impl GameRepository for GameRepositoryImpl {
    async fn find_games_by_lang_and_limit_offset(&self, lang: &str, limit: u64, offset: u64) -> anyhow::Result<Vec<Game>> {
        let rows = query("SELECT game_id, name FROM game.game LIMIT $2 OFFSET $3")
            .bind(lang)
            .bind(limit as i64)
            .bind(offset as i64)
            .fetch_all(&*self.db_pool)
            .await?;

        let mut result = Vec::new();
        for row in rows {
            let id: u64 = row.try_get::<i64, &str>("game_id")? as u64;
            let name: String = row.try_get("name")?;
            result.push(Game::new(id, name));
        }

        Ok(result)
    }
}