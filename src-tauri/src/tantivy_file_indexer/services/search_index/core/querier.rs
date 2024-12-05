use std::collections::HashSet;

use tantivy::{
    collector::TopDocs,
    query::{BooleanQuery, Occur, Query, QueryParser, RangeQuery, TermQuery},
    schema::Schema,
    DateTime, DocAddress, Searcher, TantivyDocument, Term,
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
    let query = construct_query(&schema, &searcher, &search_params)
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
    let query =
        construct_query(schema, searcher, search_params).expect("Query could not be constructed");

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

/// Construct a query to retrieve files, based off of the files' schema
fn construct_query(
    schema: &Schema,
    searcher: &Searcher,
    search_params: &SearchParamsDTO,
) -> tantivy::Result<BooleanQuery> {
    let mut queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();

    if let Some(file_path) = &search_params.file_path {
        let field = schema.get_field("path").unwrap();
        let query_parser = QueryParser::for_index(searcher.index(), vec![field]);
        let query = query_parser.parse_query(file_path)?;
        queries.push((Occur::Should, Box::new(query)));
    }

    if let Some(query_str) = &search_params.name {
        let field = schema.get_field("name").unwrap();
        let query_parser = QueryParser::for_index(searcher.index(), vec![field]);
        let query = query_parser.parse_query(query_str)?;
        queries.push((Occur::Should, Box::new(query)));
    }

    if let Some(date_range) = &search_params.date_modified_range {
        let start_date = DateTime::from_utc(date_range.start);
        let end_date = DateTime::from_utc(date_range.end);
        let query = RangeQuery::new_date("date_modified".to_string(), start_date..end_date);
        queries.push((Occur::Must, Box::new(query)));
    }

    if let Some(date_range) = &search_params.date_created_range {
        let start_date = DateTime::from_utc(date_range.start);
        let end_date = DateTime::from_utc(date_range.end);
        let query = RangeQuery::new_date("date_created".to_string(), start_date..end_date);
        queries.push((Occur::Must, Box::new(query)));
    }

    if let Some(metadata) = &search_params.metadata {
        let field = schema.get_field("metadata").unwrap();
        let term = Term::from_field_text(field, metadata);
        let query = TermQuery::new(term, tantivy::schema::IndexRecordOption::Basic);
        queries.push((Occur::Must, Box::new(query)));
    }

    // Combine all the queries into a BooleanQuery
    Ok(BooleanQuery::new(queries))
}

fn apply_popularity(existing_score: f32, popularity_score: f64) -> f64 {
    (existing_score as f64) + popularity_score.log(10.0)
}
