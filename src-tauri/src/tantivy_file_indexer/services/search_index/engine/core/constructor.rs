use tantivy::{
    query::{BooleanQuery, Occur, Query, QueryParser, RangeQuery, TermQuery},
    schema::Schema,
    DateTime, Searcher, TantivyError, Term,
};

use crate::tantivy_file_indexer::dtos::search_params_dto::{DateRange, SearchParamsDTO};

/// Schema + Searcher
struct SS<'a> {
    schema: &'a Schema,
    searcher: &'a Searcher,
}

/// Construct a query to retrieve files, based off of the files' schema
pub fn construct_query(
    schema: &Schema,
    searcher: &Searcher,
    search_params: &SearchParamsDTO,
) -> tantivy::Result<BooleanQuery> {
    let mut queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();
    let ss = SS { schema, searcher };

    if let Some(file_path) = &search_params.file_path {
        let query = create_standard_query("path", &file_path, &ss, Occur::Should)?;
        queries.push(query);
    }

    if let Some(query_str) = &search_params.name {
        let query = create_standard_query("name", &query_str, &ss, Occur::Should)?;
        queries.push(query);
    }

    if let Some(date_range) = &search_params.date_modified_range {
        queries.push(create_date_query("date_modified", date_range, Occur::Must));
    }

    if let Some(date_range) = &search_params.date_created_range {
        queries.push(create_date_query("date_created", date_range, Occur::Must));
    }

    if let Some(metadata) = &search_params.metadata {
        let query = create_term_query("metadata", &metadata, &ss, Occur::Must)?;
        queries.push(query);
    }

    // Combine all the queries into a BooleanQuery
    Ok(BooleanQuery::new(queries))
}

fn create_standard_query(
    field_name: &str,
    query: &str,
    ss: &SS,
    occur: Occur,
) -> Result<(Occur, Box<dyn Query>), TantivyError> {
    let field = ss.schema.get_field(field_name)?;
    let mut query_parser = QueryParser::for_index(ss.searcher.index(), vec![field]);
    // TODO: look into this. idk what it does hyet
    query_parser.set_conjunction_by_default();
    let query = query_parser.parse_query(query)?;
    Ok((occur, Box::new(query)))
}

fn create_date_query(field_name: &str, range: &DateRange, occur: Occur) -> (Occur, Box<dyn Query>) {
    let start_date = DateTime::from_utc(range.start);
    let end_date = DateTime::from_utc(range.end);
    let query = RangeQuery::new_date(field_name.to_string(), start_date..end_date);
    (occur, Box::new(query))
}

/// More rigid than a standard query. Checks for exact matches
fn create_term_query(
    field_name: &str,
    query: &str,
    ss: &SS,
    occur: Occur,
) -> Result<(Occur, Box<dyn Query>), TantivyError> {
    let field = ss.schema.get_field(field_name)?;
    let term = Term::from_field_text(field, query);
    let query = TermQuery::new(term, tantivy::schema::IndexRecordOption::WithFreqs);
    Ok((occur, Box::new(query)))
}
