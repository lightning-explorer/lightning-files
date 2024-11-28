#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SystemFileModel{
    pub name:String,
    pub file_path: String,
    pub date_modified: String,
    pub is_directory: bool,
}