mod dtos {
    pub mod inline_query_dto;
}
mod models {
    pub mod drive_model;
    pub mod get_files_model;
    pub mod system_info_model;
}
mod core;
mod app_state {
    pub mod sys_info;
    pub mod files_display;
    pub mod manager;
}
mod util {
    pub mod metadata_inspector;
    pub mod path_ops;
}
pub mod tauri_exports;
pub mod state;