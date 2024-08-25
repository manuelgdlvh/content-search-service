use std::sync::Arc;

use crate::models::language::Language;
use crate::services::doc_details_retriever::DocDetailsRetriever;
use crate::services::index_processor::IndexWriter;

pub struct IndexTask<T1, T2>
where
    T1: DocDetailsRetriever + Sync + Send,
    T2: IndexWriter + Send + Sync,
{
    data_retriever: T1,
    index_writer: Arc<T2>,
    limit: u64,
}

impl<T1, T2> IndexTask<T1, T2>
where
    T1: DocDetailsRetriever + Sync + Send,
    T2: IndexWriter + Send + Sync,
{
    pub fn new(limit: u64, data_retriever: T1, index_writer: Arc<T2>) -> Self {
        Self {
            data_retriever,
            index_writer,
            limit,
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        for lang in Language::all() {
            let mut results = Vec::new();
            let mut offset = 0;
            loop {
                let entries = self.data_retriever.retrieve(lang.into(), self.limit, offset).await?;
                if entries.is_empty() {
                    break;
                }
                offset += self.limit;
                results.extend(entries);
            }

            self.index_writer.swap_index(lang, &results)?;
        }

        Ok(())
    }
}


