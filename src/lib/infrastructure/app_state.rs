use std::sync::Arc;

use derive_builder::Builder;

use crate::services::search_service_impl::SearchService;

#[derive(Builder)]
pub struct AppState<S>
where
    S: SearchService,
{
    search_service: Arc<S>,
}

impl<S> AppState<S>
where
    S: SearchService,
{
    pub fn search_service(&self) -> &Arc<S> {
        &self.search_service
    }
}