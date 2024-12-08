use tantivy::{
    query::{BooleanQuery, Occur, Query, QueryParser, RangeQuery, TermQuery},
    schema::Schema,
    DateTime, Searcher, Term,
};

use crate::tantivy_file_indexer::dtos::search_params_dto::SearchParamsDTO;

/// Construct a query to retrieve files, based off of the files' schema
pub fn construct_query(
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
