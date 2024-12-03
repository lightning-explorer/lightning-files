pub mod service_container;
mod schemas {
    pub mod file_schema;
}
mod dtos {
    pub mod add_to_crawler_queue;
    pub mod vector_search_result;
}
mod converters {
    pub mod date_converter;
    pub mod doc_to_dto;
}
pub mod services {
    pub mod app_save;
    pub mod local_crawler;
    pub mod local_db;
    pub mod search_index;
    pub mod vector_db;
}
mod models {
    pub mod search_params_model;
    pub mod tantivy_file_model;
    pub mod vector_search_params_model;
    pub mod interal_system_file;
}
pub mod shared;