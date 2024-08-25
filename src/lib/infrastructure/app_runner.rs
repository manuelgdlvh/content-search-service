use std::sync::Arc;

use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tokio::sync::{oneshot};

use crate::config::CONFIG;
use crate::infrastructure::di_container::{DB_POOL_DEP, DIContainer, GAME_INDEX_PROCESSOR_DEP, GAME_REPOSITORY_IMPL_DEP, MOVIE_INDEX_PROCESSOR_DEP, MOVIE_REPOSITORY_IMPL_DEP, RECIPE_INDEX_PROCESSOR_DEP, RECIPE_REPOSITORY_IMPL_DEP, SEARCH_SERVICE_IMPL_DEP, TV_INDEX_PROCESSOR_DEP, TV_REPOSITORY_IMPL_DEP};
use crate::infrastructure::http_server::HttpServer;
use crate::repositories::game_repository_impl::GameRepositoryImpl;
use crate::repositories::movie_repository_impl::MovieRepositoryImpl;
use crate::repositories::recipe_repository_impl::RecipeRepositoryImpl;
use crate::repositories::tv_repository_impl::TvRepositoryImpl;
use crate::services::index_processor::IndexProcessor;
use crate::services::indexer_runner::IndexerRunner;
use crate::services::search_service_impl::SearchServiceImpl;

pub struct AppRunner;


impl AppRunner {
    pub async fn run(signal: Option<oneshot::Sender<()>>) -> anyhow::Result<()> {
        Self::logger_init();
        let db_pool = Self::database_init().await?;
        let di_container = Self::dependency_injection_init(db_pool)?;
        Self::background_jobs(&di_container).await;
        let mut http_server = HttpServer::build(&di_container).await?;

        if let Some(signal) = signal {
            signal.send(()).expect("Signal Sent");
        }

        http_server.start().await
    }

    async fn database_init() -> anyhow::Result<Pool<Postgres>> {
        log::info!("initializing database connection pool...");

        let conn_info = format!(
            "postgres://{}:{}@{}:{}/{}",
            CONFIG.database().username(),
            CONFIG.database().password(),
            CONFIG.database().host(),
            CONFIG.database().port(),
            CONFIG.database().db_name()
        );
        let db_pool = PgPoolOptions::new()
            .max_connections(CONFIG.database().max_connections())
            .min_connections(CONFIG.database().min_connections())
            .connect(conn_info.as_str())
            .await?;

        log::info!("initialized database connection pool successfully");

        Ok(db_pool)
    }

    fn logger_init() {
        if CONFIG.logger().enabled() {
            env_logger::builder()
                .filter_level(CONFIG.logger().level())
                .init();
        }
    }

    fn dependency_injection_init(db_pool: Pool<Postgres>) -> anyhow::Result<Arc<DIContainer>> {
        let di_container = DIContainer::new();

        // Repositories
        di_container.add(DB_POOL_DEP, db_pool);
        di_container.add(MOVIE_REPOSITORY_IMPL_DEP, MovieRepositoryImpl::new(&di_container)?);
        di_container.add(TV_REPOSITORY_IMPL_DEP, TvRepositoryImpl::new(&di_container)?);
        di_container.add(RECIPE_REPOSITORY_IMPL_DEP, RecipeRepositoryImpl::new(&di_container)?);
        di_container.add(GAME_REPOSITORY_IMPL_DEP, GameRepositoryImpl::new(&di_container)?);

        // Indexers
        di_container.add(MOVIE_INDEX_PROCESSOR_DEP, IndexProcessor::new()?);
        di_container.add(TV_INDEX_PROCESSOR_DEP, IndexProcessor::new()?);
        di_container.add(GAME_INDEX_PROCESSOR_DEP, IndexProcessor::new()?);
        di_container.add(RECIPE_INDEX_PROCESSOR_DEP, IndexProcessor::new()?);

        // Services
        di_container.add(SEARCH_SERVICE_IMPL_DEP, SearchServiceImpl::new(&di_container));

        Ok(di_container)
    }

    async fn background_jobs(di_container: &DIContainer) {
        let indexer_runner: IndexerRunner = Default::default();
        let mut signal = indexer_runner.run(di_container);

        if CONFIG.indexer_runner().wait_until_index() {
            signal.recv().await;
        }
    }
}