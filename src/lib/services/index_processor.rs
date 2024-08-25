use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Mutex;

use dashmap::DashMap;
use dashmap::mapref::one::{Ref, RefMut};
use tantivy::{doc, DocAddress, Index, IndexBuilder, IndexReader, IndexWriter as TantivyIndexWriter, Score, TantivyDocument};
use tantivy::collector::TopDocs;
use tantivy::query::{BooleanQuery, Occur, Query, RegexQuery};
use tantivy::schema::{Field, OwnedValue, Schema, STORED, TEXT};

use crate::models::doc_details::DocDetails;
use crate::models::language::Language;

const TITLE_FIELD: &'static str = "title";
const ID_FIELD: &'static str = "id";
const LIMIT_RESULT_SIZE: usize = 75;
const MEMORY_BUDGET_BYTES: usize = 100_000_000;

// Structs
pub struct IndexProcessor {
    inner: DashMap<Language, Inner>,
}

struct Inner {
    pub index: Index,
    pub index_writer: Mutex<TantivyIndexWriter>,
    pub index_reader: IndexReader,
    pub fields: HashMap<String, Field>,
}


// Impls
impl Inner {
    fn new() -> anyhow::Result<Inner> {
        let mut schema_builder = Schema::builder();
        let title = schema_builder.add_text_field(TITLE_FIELD, TEXT);
        let id = schema_builder.add_u64_field(ID_FIELD, STORED);
        let schema = schema_builder.build();

        let index = IndexBuilder::create_in_ram(Index::builder().schema(schema.clone()))?;
        let index_writer = index.writer(MEMORY_BUDGET_BYTES)?;
        let index_reader = index.reader()?;

        let mut fields = HashMap::new();
        fields.insert(TITLE_FIELD.to_string(), title);
        fields.insert(ID_FIELD.to_string(), id);

        Ok(Inner {
            index,
            index_writer: Mutex::new(index_writer),
            index_reader,
            fields,
        })
    }
    fn write_all(&self, data: &[DocDetails]) -> anyhow::Result<()> {
        let id_field = self.id();
        let title_field = self.title();
        if let Ok(mut writer) = self.index_writer.lock() {
            for doc in data.iter() {
                writer.add_document(doc!(
            title_field => doc.title(),
            id_field => doc.id()))?;
            }
            writer.commit()?;
        }

        self.index_reader.reload()?;
        Ok(())
    }

    fn search(&self, tokens: &[&str]) -> anyhow::Result<Vec<u64>> {
        let title = self.title();
        let id = self.id();

        let last_index = tokens.len() - 1;
        let mut current_index = 0;
        let mut subqueries = Vec::new();
        for token in tokens {
            let mut current = Cow::from(*token);
            if current_index == last_index {
                current.to_mut().push_str(".*");
            }

            subqueries.push((Occur::Must, Box::new(RegexQuery::from_pattern(current.as_ref(), title)?)
                as Box<dyn Query>));
            current_index += 1;
        }

        let query = BooleanQuery::new(subqueries);
        let searcher = self.index_reader.searcher();

        let mut result = Vec::new();
        let top_docs: Vec<(Score, DocAddress)> = searcher.search(&query, &TopDocs::with_limit(LIMIT_RESULT_SIZE))?;
        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc::<TantivyDocument>(doc_address)?;

            if let Some(owned_value) = retrieved_doc.get_first(id) {
                match owned_value {
                    OwnedValue::U64(value) => {
                        result.push(*value)
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }

        Ok(result)
    }

    fn title(&self) -> Field {
        *self.fields.get(TITLE_FIELD).unwrap()
    }

    fn id(&self) -> Field {
        *self.fields.get(ID_FIELD).unwrap()
    }
}
impl IndexProcessor {
    pub fn new() -> anyhow::Result<Self> {
        let indexers = DashMap::new();
        for lang in Language::all() {
            indexers.insert(lang, Inner::new()?);
        }

        Ok(Self {
            inner: indexers
        })
    }

    fn inner<'a>(&'a self, language: &'a Language) -> Ref<'a, Language, Inner> {
        self.inner.get(&language).unwrap()
    }

    fn inner_mut<'a>(&'a self, language: &'a Language) -> RefMut<'a, Language, Inner> {
        self.inner.get_mut(language).unwrap()
    }
}


// Traits

pub trait IndexWriter {
    fn write_all(&self, lang: Language, data: &[DocDetails]) -> anyhow::Result<()>;

    fn swap_index(&self, lang: Language, data: &[DocDetails]) -> anyhow::Result<()>;
}

pub trait IndexSearcher {
    fn search(&self, lang: Language, tokens: &[&str]) -> anyhow::Result<Vec<u64>>;
}

impl IndexSearcher for IndexProcessor {
    fn search(&self, lang: Language, tokens: &[&str]) -> anyhow::Result<Vec<u64>> {
        let inner = self.inner(&lang);
        inner.search(tokens)
    }
}

impl IndexWriter for IndexProcessor {
    fn write_all(&self, lang: Language, data: &[DocDetails]) -> anyhow::Result<()> {
        let inner = self.inner(&lang);
        inner.write_all(data)
    }

    fn swap_index(&self, lang: Language, data: &[DocDetails]) -> anyhow::Result<()> {
        let inner = Inner::new()?;
        inner.write_all(data)?;

        *self.inner_mut(&lang) = inner;
        Ok(())
    }
}