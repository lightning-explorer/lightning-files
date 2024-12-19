pub mod files {
    pub mod inline_search {
        pub mod tauri_cmds;
    }
    pub mod operations {
        pub mod file_reader;
        pub mod metadata;
        pub mod common;
        pub mod tauri_cmds;
    }
    pub mod retrieve {
        mod helper;
        mod file_retriever;
        mod file_sorter;
        pub mod tauri_cmds;
    }
    pub mod user_input_validator {
        pub mod tauri_cmds;
    }
}
pub mod drives {
    pub mod tauri_cmds;
    mod com {
        pub mod get_sys_drives;
    }
}
pub mod sys{
    mod info;
    pub mod tauri_cmds;
}