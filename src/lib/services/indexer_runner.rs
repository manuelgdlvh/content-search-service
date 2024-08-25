use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::Receiver;
use tokio::time;

use crate::config::CONFIG;
use crate::infrastructure::di_container::{DIContainer, GAME_INDEX_PROCESSOR_DEP, MOVIE_INDEX_PROCESSOR_DEP, RECIPE_INDEX_PROCESSOR_DEP, TV_INDEX_PROCESSOR_DEP};
use crate::repositories::game_repository_impl::GameRepositoryImpl;
use crate::repositories::movie_repository_impl::MovieRepositoryImpl;
use crate::repositories::recipe_repository_impl::RecipeRepositoryImpl;
use crate::repositories::tv_repository_impl::TvRepositoryImpl;
use crate::services::impls::game_doc_details_retriever::GameDocDetailsRetriever;
use crate::services::impls::movie_doc_details_retriever::MovieDocDetailsRetriever;
use crate::services::impls::recipe_doc_details_retriever::RecipeDocDetailsRetriever;
use crate::services::impls::tv_doc_details_retriever::TvDocDetailsRetriever;
use crate::services::index_processor::IndexProcessor;
use crate::services::index_task::IndexTask;

pub struct IndexerRunner;


impl IndexerRunner {
    pub fn new() -> Self {
        Self {}
    }
    pub fn run(&self, di_container: &DIContainer) -> Receiver<()> {
        let (tx, rv) = mpsc::channel::<()>(1);
        let batch_size = *&CONFIG.indexer_runner().batch_size();
        let interval = *&CONFIG.indexer_runner().interval();

        let movie_indexer = IndexTask::<MovieDocDetailsRetriever<MovieRepositoryImpl>
            , IndexProcessor>::new(batch_size, MovieDocDetailsRetriever::new(di_container), di_container.get::<IndexProcessor>(MOVIE_INDEX_PROCESSOR_DEP));
        let tv_indexer = IndexTask::<TvDocDetailsRetriever<TvRepositoryImpl>
            , IndexProcessor>::new(batch_size, TvDocDetailsRetriever::new(di_container), di_container.get::<IndexProcessor>(TV_INDEX_PROCESSOR_DEP));
        let recipe_indexer = IndexTask::<RecipeDocDetailsRetriever<RecipeRepositoryImpl>
            , IndexProcessor>::new(batch_size, RecipeDocDetailsRetriever::new(di_container), di_container.get::<IndexProcessor>(RECIPE_INDEX_PROCESSOR_DEP));
        let game_indexer = IndexTask::<GameDocDetailsRetriever<GameRepositoryImpl>
            , IndexProcessor>::new(batch_size, GameDocDetailsRetriever::new(di_container), di_container.get::<IndexProcessor>(GAME_INDEX_PROCESSOR_DEP));

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(interval));
            interval.tick().await;
            loop {
                log::info!("starting reindex of all content...");
                let _ = movie_indexer.start().await;
                let _ = tv_indexer.start().await;
                let _ = recipe_indexer.start().await;
                let _ = game_indexer.start().await;
                let _ = tx.send(()).await;
                interval.tick().await;
            }
        });

        rv
    }
}