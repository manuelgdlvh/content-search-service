use std::sync::{Arc, LazyLock};

use axum::extract::State;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::JsonDeserializer;

use crate::handlers::requests::search_request::SearchRequest;
use crate::models::entity::Entity;
use crate::models::language::Language;
use crate::services::search_service_impl::SearchService;

pub const LANGUAGE_HEADER: &str = "Language";
pub const DEFAULT_LANGUAGE: &str = "EN";
pub static DEFAULT_LANGUAGE_HEADER: LazyLock<HeaderValue> = LazyLock::new(|| {
    HeaderValue::try_from(DEFAULT_LANGUAGE).unwrap()
});


pub async fn search<S>(State(search_service): State<Arc<S>>
                       , headers: HeaderMap
                       , payload: JsonDeserializer<SearchRequest<'_>>) -> Result<Json<Vec<u64>>, Response>
where
    S: SearchService,
{
    let mut input = payload.deserialize().map_err(|err| err.into_response())?;

    let language;
    if let Some(value) = headers.get(LANGUAGE_HEADER) {
        language = value.to_str().unwrap();
    } else {
        language = DEFAULT_LANGUAGE;
    }

    log::info!("received search request with language: {language}, input: {:?}", input);

    let entity: Entity = Entity::try_from(input.entity().as_ref()).map_err(|_| StatusCode::BAD_REQUEST.into_response())?;

    let language: Language = Language::try_from(language).map_err(|_| StatusCode::BAD_REQUEST.into_response())?;

    let keywords = input.keywords_mut();
    match search_service.search(keywords, language, entity) {
        Ok(value) => {
            Ok(Json(value))
        }
        Err(_) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}