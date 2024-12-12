mod core {
    pub mod tantivy_setup;
    pub mod query{
        pub mod querier;
        pub mod constructor;
        pub mod query_organizer;
    }
}
mod schemas {
    pub mod file_schema;
}
mod services {
    pub mod task_manager;
}
pub mod files_collection;
pub mod service;
pub mod tauri_exports;
