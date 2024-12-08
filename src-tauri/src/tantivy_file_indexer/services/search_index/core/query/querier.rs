use super::constructor;
use std::collections::HashSet;

use tantivy::{
    collector::TopDocs, query::Query, schema::Schema, DocAddress, Searcher, TantivyDocument,
};

use crate::tantivy_file_indexer::{
    converters::doc_to_dto::doc_to_tantivy_file_model, dtos::search_params_dto::SearchParamsDTO,
    models::tantivy_file_model::TantivyFileModel,
};

/// Where `min_results` indicated how many documents should initially be fetched, and max_results is the value to work up to
///
/// `step_size`: the number of events that will get emitted in total
pub async fn advanced_query_streamed<EmitFn>(
    schema: Schema,
    searcher: Searcher,
    search_params: SearchParamsDTO,
    emit: EmitFn,
    step_size: usize,
    min_results: usize,
) where
    EmitFn: Fn(Vec<TantivyFileModel>),
{
    let query = constructor::construct_query(&schema, &searcher, &search_params)
        .expect("Query could not be constructed");

    let max_results = search_params.num_results as usize;

    let step_by = (max_results - min_results) / step_size;
    let mut current_out_amt = min_results;
    let mut prev_docs: HashSet<DocAddress> = HashSet::new();

    for _ in 0..step_size {
        match execute_query(&searcher, current_out_amt, &query) {
            Ok(top_docs) => {
                let mut output_docs: Vec<TantivyFileModel> = Vec::new();
                for (_score, address) in top_docs {
                    if prev_docs.insert(address) {
                        // Value got inserted, proceed
                        if let Ok(doc) = searcher.doc(address) {
                            output_docs.push(doc_to_tantivy_file_model(doc, &schema, _score));
                        } else {
                            println!("Failed to retrieve document for address {:?}", address);
                        }
                    }
                }
                // Send the batch of documents
                emit(output_docs);
                current_out_amt += step_by;
            }
            Err(err) => {
                println!("Error executing query: {}", err);
            }
        }
    }
}

pub fn advanced_query(
    schema: &Schema,
    searcher: &Searcher,
    search_params: &SearchParamsDTO,
) -> tantivy::Result<Vec<TantivyFileModel>> {
    let query = constructor::construct_query(schema, searcher, search_params)
        .expect("Query could not be constructed");

    // Execute the query and collect the results
    let top_docs = execute_query(searcher, search_params.num_results as usize, &query)?;

    let results: Vec<TantivyFileModel> = top_docs
        .into_iter()
        .map(|(_score, doc_address)| {
            let doc: TantivyDocument = searcher.doc(doc_address).unwrap();
            doc_to_tantivy_file_model(doc, schema, _score)
        })
        .collect();

    Ok(results)
}

/// Execute a standard query, applying a popularity bias to the results
fn execute_query<Q>(
    searcher: &Searcher,
    num_results: usize,
    query: &Q,
) -> tantivy::Result<Vec<(f64, tantivy::DocAddress)>>
where
    Q: Query + Sized,
{
    searcher.search(
        query,
        &TopDocs::with_limit(num_results).tweak_score(|segment_reader: &tantivy::SegmentReader| {
            let popularity_field = segment_reader
                .fast_fields()
                .f64("popularity")
                .expect("Failed to access popularity field");
            move |doc, original_score| {
                // Default to 1 if no popularity
                let pop_score = popularity_field.first(doc).unwrap_or(1.0);
                apply_popularity(original_score, pop_score)
            }
        }),
    )
}

fn apply_popularity(existing_score: f32, popularity_score: f64) -> f64 {
    (existing_score as f64) + popularity_score.log(10.0)
}
