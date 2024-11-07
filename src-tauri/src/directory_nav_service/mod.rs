mod dtos {
    pub mod inline_query_dto;
}
mod models {
    pub mod drive_model;
    pub mod get_files_model;
}
mod core {
    pub mod files {
        pub mod file_ops;
        pub mod file_retriever;
        pub mod user_input_validator;
        pub mod search{
            pub mod inline_search;
        }
    }
    pub mod drives {
        pub mod drive_ops;
        pub mod com {
            pub mod get_sys_drives;
        }
    }
}
mod app_state{
    pub mod files_display;
}
mod util {
    pub mod metadata_inspector;
    pub mod path_ops;
}
pub mod service;
