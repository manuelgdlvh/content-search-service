use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context;

use crate::infrastructure::di_container::{DIContainer, GAME_INDEX_PROCESSOR_DEP, MOVIE_INDEX_PROCESSOR_DEP, RECIPE_INDEX_PROCESSOR_DEP, TV_INDEX_PROCESSOR_DEP};
use crate::models::entity::Entity;
use crate::models::language::Language;
use crate::services::index_processor::{IndexProcessor, IndexSearcher};

pub struct SearchServiceImpl {
    searchers: HashMap<Entity, Arc<dyn IndexSearcher + Send + Sync>>,
}

impl SearchServiceImpl {
    pub fn new(di_container: &DIContainer) -> Self {
        let mut searchers = HashMap::new();

        let movie_index_processor = di_container.get::<IndexProcessor>(MOVIE_INDEX_PROCESSOR_DEP)
            as Arc<dyn IndexSearcher + Send + Sync>;
        let tv_index_processor = di_container.get::<IndexProcessor>(TV_INDEX_PROCESSOR_DEP)
            as Arc<dyn IndexSearcher + Send + Sync>;
        let recipe_index_processor = di_container.get::<IndexProcessor>(RECIPE_INDEX_PROCESSOR_DEP)
            as Arc<dyn IndexSearcher + Send + Sync>;
        let game_index_processor = di_container.get::<IndexProcessor>(GAME_INDEX_PROCESSOR_DEP)
            as Arc<dyn IndexSearcher + Send + Sync>;

        searchers.insert(Entity::Movie, movie_index_processor);
        searchers.insert(Entity::Tv, tv_index_processor);
        searchers.insert(Entity::Recipe, recipe_index_processor);
        searchers.insert(Entity::Game, game_index_processor);

        Self { searchers }
    }
}

pub trait SearchService {
    fn search(&self, keywords: &mut str, lang: Language, entity: Entity) -> anyhow::Result<Vec<u64>>;
}
impl SearchService for SearchServiceImpl {
    fn search(&self, keywords: &mut str, lang: Language, entity: Entity) -> anyhow::Result<Vec<u64>> {
        let searcher = self.searchers.get(&entity).context("SEARCHER NOT FOUND")?;

        keywords.make_ascii_lowercase();
        let tokens = keywords.trim().split_whitespace().collect::<Vec<&str>>();
        searcher.search(lang, &tokens)
    }
}