use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use anyhow::Context;

use crate::infrastructure::di_container::{DIContainer, GAME_INDEX_PROCESSOR_DEP, MOVIE_INDEX_PROCESSOR_DEP, RECIPE_INDEX_PROCESSOR_DEP, TV_INDEX_PROCESSOR_DEP};
use crate::models::entity::Entity;
use crate::models::language::Language;
use crate::services::index_processor::IndexSearcher;

type IndexSearcherType = Arc<dyn IndexSearcher>;
pub struct SearchServiceImpl<T>
where
    T: IndexSearcher,
{
    searchers: HashMap<Entity, Arc<dyn IndexSearcher>>,
    phantom_data: PhantomData<T>,
}

impl<T> SearchServiceImpl<T>
where
    T: IndexSearcher + Sync + Send + 'static,
{
    pub fn new(di_container: &DIContainer) -> Self {
        let mut searchers = HashMap::new();

        let movie_index_processor = di_container.get::<T>(MOVIE_INDEX_PROCESSOR_DEP)
            as IndexSearcherType ;
        let tv_index_processor = di_container.get::<T>(TV_INDEX_PROCESSOR_DEP)
            as IndexSearcherType;
        let recipe_index_processor = di_container.get::<T>(RECIPE_INDEX_PROCESSOR_DEP)
            as IndexSearcherType;
        let game_index_processor = di_container.get::<T>(GAME_INDEX_PROCESSOR_DEP)
            as IndexSearcherType;

        searchers.insert(Entity::Movie, movie_index_processor);
        searchers.insert(Entity::Tv, tv_index_processor);
        searchers.insert(Entity::Recipe, recipe_index_processor);
        searchers.insert(Entity::Game, game_index_processor);

        Self { searchers, phantom_data: Default::default() }
    }
}

pub trait SearchService {
    fn search(&self, keywords: &mut str, lang: Language, entity: Entity) -> anyhow::Result<Vec<u64>>;
}
impl<T> SearchService for SearchServiceImpl<T>
where
    T: IndexSearcher,
{
    fn search(&self, keywords: &mut str, lang: Language, entity: Entity) -> anyhow::Result<Vec<u64>> {
        let searcher = self.searchers.get(&entity).unwrap();

        keywords.make_ascii_lowercase();
        let tokens = keywords.split_whitespace().collect::<Vec<&str>>();
        searcher.search(lang, &tokens)
    }
}