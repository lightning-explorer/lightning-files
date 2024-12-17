use std::collections::HashSet;

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
    // Build a HashSet of file paths from `models`
    let model_paths: HashSet<_> = models.iter().map(|file| &file.file_path).collect();

    // Filter out stale `children` whose file paths are not in `model_paths`
    children
        .iter()
        .filter(|child| !model_paths.contains(&child.file_path))
        .cloned()
        .collect()
}

fn search_by_term(searcher: Searcher, term: Term) -> tantivy::Result<Vec<TantivyDocument>> {
    let query = TermQuery::new(term, tantivy::schema::IndexRecordOption::Basic);
    // Search using the query
    let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(1_000_000))?;

    // Collect the document addresses. Use flat map since 'searcher.doc' can return an error
    let doc_addresses: Vec<TantivyDocument> = top_docs
        .into_iter()
        .flat_map(|(_, doc_addr)| searcher.doc(doc_addr))
        .collect();

    Ok(doc_addresses)
}

/// `model` is expected to be a directory. A search will be made to find all models whose `parent_path` matches the given directory file path
pub fn search_by_directory(
    searcher: Searcher,
    model: &TantivyFileModel,
) -> tantivy::Result<Vec<TantivyDocument>> {
    let term = TantivyFileModel::make_term("parent_directory", &model.file_path).unwrap();
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
