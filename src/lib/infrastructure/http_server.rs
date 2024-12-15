use axum::Router;
use axum::routing::post;
use tokio::net::TcpListener;

use crate::config::CONFIG;
use crate::handlers;
use crate::infrastructure::di_container::{DIContainer, SEARCH_SERVICE_IMPL_DEP};
use crate::services::index_processor::IndexProcessor;
use crate::services::search_service_impl::SearchServiceImpl;

pub struct HttpServer {
    app: Option<Router>,
    listener: Option<TcpListener>,
    started: bool,
}

impl HttpServer {
    pub async fn build(di_container: &DIContainer) -> anyhow::Result<Self> {
        let routes = Router::new()
            .route("/run", post(handlers::search_handler::search))
            .with_state(di_container.get::<SearchServiceImpl<IndexProcessor>>(SEARCH_SERVICE_IMPL_DEP));

        let tcp_addr = format!("{}:{}", CONFIG.server().host(), CONFIG.server().port());

        Ok(Self {
            app: Some(routes),
            listener: Some(
                TcpListener::bind(tcp_addr).await?,
            ),
            started: false,
        })
    }
    pub async fn start(&mut self) -> anyhow::Result<()> {
        log::info!("http server initializing for listen incoming requests...");
        if self.started {
            return Ok(());
        }
        self.started = true;
        let app = self.app.take().expect("Take router web server");
        let listener = self.listener.take().expect("Take TCP listener");
        axum::serve(listener, app).await.expect("Init web server");
        Ok(())
    }
}


