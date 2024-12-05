mod dtos {
    pub mod inline_query_dto;
}
mod models {
    pub mod drive_model;
    pub mod get_files_model;
}
mod core;
mod app_state {
    pub mod files_display;
}
mod util {
    pub mod metadata_inspector;
    pub mod path_ops;
}
pub mod tauri_exports;
