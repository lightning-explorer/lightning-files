mod core {
    pub mod querier;
    pub mod tantivy_setup;
}
mod schemas {
    pub mod file_schema;
}
mod services{
    pub mod task_manager;
}
pub mod files_collection;
pub mod service;
pub mod tauri_exports;
