use tantivy::{query::TermQuery, Searcher, TantivyDocument, Term};

use crate::{
    shared::models::sys_file_model::SystemFileModel,
    tantivy_file_indexer::services::search_index::models::file::TantivyFileModel,
};

/// Helper function to map errors into strings
pub fn map_err<T, E: ToString>(result: Result<T, E>) -> Result<T, String> {
    result.map_err(|err| err.to_string())
}

/// Helper function to get stale models. The `models` parameter represents the new models and `children` are the old ones
pub fn classify_stale_models(
    children: &[SystemFileModel],
    models: &[TantivyFileModel],
) -> Vec<SystemFileModel> {
    let mut stale = Vec::new();

    for child in children.iter() {
        if !models.iter().any(|file| file.file_path == child.file_path) {
            stale.push(child.clone());
        }
    }
    stale
}

fn search_by_term(searcher: Searcher, term: Term) -> tantivy::Result<Vec<TantivyDocument>> {
    let query = TermQuery::new(term, tantivy::schema::IndexRecordOption::Basic);
    // Search using the query
    let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(1_000_000))?;

    // Collect the document addresses. Use flatten for now since 'searcher.doc' can return an error
    let doc_addresses: Vec<TantivyDocument> = top_docs
        .into_iter()
        .map(|(_, doc_addr)| searcher.doc(doc_addr))
        .flatten()
        .collect();

    Ok(doc_addresses)
}

pub fn search_by_parent_directory(
    searcher: Searcher,
    model: &TantivyFileModel,
) -> tantivy::Result<Vec<TantivyDocument>> {
    // TODO: do something abou this. Accessing the field directly here is kinda arbitrary
    let term = TantivyFileModel::make_term("parent_directory", &model.parent_directory).unwrap();
    search_by_term(searcher, term)
}

pub fn search_by_path(searcher: Searcher, file_path: &str) -> tantivy::Result<Option<TantivyDocument>> {
    let term = TantivyFileModel::make_term("path", file_path).unwrap();
    let mut results = search_by_term(searcher, term)?;
    match results.len() {
        0 => Ok(None),
        1 => Ok(Some(results.pop().unwrap())),
        val => {
            panic!("Duplicate keys exist in index. Found {} entries", val)
        }
    }
}
