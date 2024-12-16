use super::core::{constructor::QueryConstructor, executor, organizer};
use std::{collections::HashSet, sync::Arc};

use tantivy::{schema::Schema, DocAddress, IndexReader, TantivyDocument};

use crate::tantivy_file_indexer::{
    dtos::search_params_dto::SearchParamsDTO, services::search_index::models::file::TantivyFileModel, shared::search_index::tantivy_traits::FromDocument
};

pub struct Querier {
    schema: Schema,
    reader:Arc<IndexReader>,
    constructor: Arc<QueryConstructor>
}

impl Querier {
    pub fn new(schema: Schema, reader:Arc<IndexReader>, constructor: Arc<QueryConstructor>) -> Self {
        Self { schema, reader, constructor }
    }
    /// Where `min_results` indicated how many documents should initially be fetched, and max_results is the value to work up to
    ///
    /// `step_size`: the number of events that will get emitted in total
    pub async fn advanced_query_streamed<EmitFn>(
        &self,
        search_params: SearchParamsDTO,
        emit: EmitFn,
        step_size: usize,
        min_results: usize,
    ) where
        EmitFn: Fn(Vec<TantivyFileModel>),
    {
        let searcher = self.reader.searcher();
        let query = self.constructor.construct_query(&search_params)
            .expect("Query could not be constructed");

        let max_results = search_params.num_results as usize;

        let step_by = (max_results - min_results) / step_size;
        let mut current_out_amt = min_results;
        let mut prev_docs: HashSet<DocAddress> = HashSet::new();

        for _ in 0..step_size {
            match executor::execute_query(&searcher, current_out_amt, &query) {
                Ok(top_docs) => {
                    let mut output_docs: Vec<TantivyFileModel> = Vec::new();
                    for (_score, address) in top_docs {
                        if prev_docs.insert(address) {
                            // Value got inserted, proceed
                            if let Ok(doc) = searcher.doc(address) {
                                output_docs.push(TantivyFileModel::from_doc(
                                    doc,
                                    &self.schema,
                                    _score,
                                ));
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

    /// Whereas the other query functions just return the items however they were presented in the index, this function adds an extra post-processing
    /// step where files belonging to the same drive, folders, sharing the same extension, etc. are grouped together.
    ///
    /// ### NOTE:
    /// this function slightly differs from `advanced_query_streamed` in the fact that the emit function will emit all of the organized files
    /// that get accumulated, meaning that the frontend needs to REPLACE its list of files with whatever gets emitted, as opposed to appending the
    /// emitted result.
    pub async fn organized_query_streamed<EmitFn>(
        &self,
        search_params: SearchParamsDTO,
        emit: EmitFn,
        step_size: usize,
        min_results: usize,
    ) where
        EmitFn: Fn(&[TantivyFileModel]),
    {
        let searcher = self.reader.searcher();
        let query =  self.constructor.construct_query(&search_params)
            .expect("Query could not be constructed");

        let max_results = search_params.num_results as usize;

        let step_by = (max_results - min_results) / step_size;
        let mut current_out_amt = min_results;
        let mut accumulated_docs: Vec<TantivyFileModel> = Vec::new();

        for _ in 0..step_size {
            match executor::execute_query(&searcher, current_out_amt, &query) {
                Ok(top_docs) => {
                    for (_score, address) in top_docs {
                        if let Ok(doc) = searcher.doc(address) {
                            accumulated_docs.push(TantivyFileModel::from_doc(
                                doc,
                                &self.schema,
                                _score,
                            ));
                        } else {
                            println!("Failed to retrieve document for address {:?}", address);
                        }
                    }
                    // Send the batch of documents
                    Self::organize_accumulated_docs(&mut accumulated_docs);
                    emit(&accumulated_docs);
                    current_out_amt += step_by;
                }
                Err(err) => {
                    println!("Error executing query: {}", err);
                }
            }
        }
    }

    fn organize_accumulated_docs(docs: &mut Vec<TantivyFileModel>) {
        let mut grouped = Vec::new();
        let groups = organizer::group_files(docs.to_vec());
        for (_grouping, mut files) in groups.into_iter() {
            grouped.append(&mut files);
        }
        *docs = grouped;
    }

    pub fn advanced_query(
        &self,
        search_params: &SearchParamsDTO,
    ) -> tantivy::Result<Vec<TantivyFileModel>> {
        let searcher = self.reader.searcher();
        let query =  self.constructor.construct_query(search_params)
            .expect("Query could not be constructed");

        // Execute the query and collect the results
        let top_docs = executor::execute_query(&searcher, search_params.num_results as usize, &query)?;

        let results: Vec<TantivyFileModel> = top_docs
            .into_iter()
            .map(|(_score, doc_address)| {
                let doc: TantivyDocument = searcher.doc(doc_address).unwrap();
                TantivyFileModel::from_doc(doc, &self.schema, _score)
            })
            .collect();

        Ok(results)
    }
}
