use std::any::Any;
use std::sync::Arc;

use dashmap::DashMap;

// Repositories
pub const DB_POOL_DEP: &str = "db_pool";
pub const MOVIE_REPOSITORY_IMPL_DEP: &str = "movie_repository_impl";
pub const TV_REPOSITORY_IMPL_DEP: &str = "tv_repository_impl";
pub const RECIPE_REPOSITORY_IMPL_DEP: &str = "recipe_repository_impl";
pub const GAME_REPOSITORY_IMPL_DEP: &str = "game_repository_impl";

// Services
pub const SEARCH_SERVICE_IMPL_DEP: &str = "search_service_impl";

// Indexers
pub const MOVIE_INDEX_PROCESSOR_DEP: &str = "movie_index_processor";
pub const TV_INDEX_PROCESSOR_DEP: &str = "tv_index_processor";
pub const RECIPE_INDEX_PROCESSOR_DEP: &str = "recipe_index_processor";
pub const GAME_INDEX_PROCESSOR_DEP: &str = "game_index_processor";


type GenericType = Arc<dyn Any + Send + Sync>;
pub struct DIContainer {
    deps: DashMap<String, GenericType>,
}

impl DIContainer {
    pub fn new() -> Arc<Self> {
        Arc::new(Self { deps: DashMap::new() })
    }


    pub fn add<T: Sync + Send + 'static>(&self, key: &str, dependency: T) {
        self.deps.insert(key.to_string(), Arc::new(dependency) as Arc<dyn Any + Send + Sync>);
    }

    pub fn get<T: Sync + Send + 'static>(&self, key: &str) -> Arc<T> {
        let dep = self.deps.get(key)
            .unwrap_or_else(|| panic!("#{key} dependency not found in dependency injection container"));

        dep.clone().downcast::<T>().unwrap_or_else(|_| panic!("invalid type for #{key} dependency"))
    }
}


