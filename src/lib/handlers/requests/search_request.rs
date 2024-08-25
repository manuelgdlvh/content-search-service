use std::borrow::Cow;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Builder, Serialize, Deserialize, Debug)]
pub struct SearchRequest<'a> {
    #[serde(rename = "keyword")]
    keywords: String,
    #[serde(borrow)]
    #[serde(rename = "type")]
    entity: Cow<'a, str>,
}

impl<'a> SearchRequest<'a> {
    pub fn keywords_mut(&mut self) -> &mut str {
        self.keywords.as_mut()
    }

    pub fn entity(&self) -> &Cow<'a, str> {
        &self.entity
    }

    pub fn new(keywords: String, entity: Cow<'a, str>) -> Self {
        Self { keywords, entity }
    }
}