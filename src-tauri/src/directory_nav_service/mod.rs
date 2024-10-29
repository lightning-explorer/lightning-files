mod dtos {
    pub mod file_dto;
}
mod models {
    pub mod drive_model;
}
mod core {
    pub mod files {
        pub mod file_ops;
        pub mod file_retriever;
        pub mod user_input_validator;
    }
    pub mod drives {
        pub mod drive_ops;
        pub mod com {
            pub mod get_sys_drives;
        }
    }
}
mod util {
    pub mod metadata_inspector;
    pub mod path_ops;
}
pub mod service;
