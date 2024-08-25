use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::marker::PhantomData;
use std::ops::Deref;
use std::pin::Pin;
use std::process::Output;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum::{async_trait, BoxError, middleware, RequestExt, Router, routing::{get, post}};
use axum::body::{Body, Bytes, to_bytes};
use axum::extract::{FromRequest, Request};
use axum::handler::Handler;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::post_service;
use axum_extra::handler::HandlerCallWithExtractors;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tokio::join;
use tokio::net::TcpListener;

use crate::config::CONFIG;
use crate::config::database_config::DatabaseConfig;
use crate::config::server_config::ServerConfig;
use crate::handlers;
use crate::infrastructure::di_container::{DIContainer, SEARCH_SERVICE_IMPL_DEP};
use crate::models::entity::Entity;
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
            .with_state(di_container.get::<SearchServiceImpl>(SEARCH_SERVICE_IMPL_DEP));

        let tcp_addr = format!("{}:{}", &*CONFIG.server().host(), &CONFIG.server().port());

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
        let app = self.app.take().unwrap();
        let listener = self.listener.take().unwrap();
        axum::serve(listener, app).await.unwrap();
        Ok(())
    }
}


