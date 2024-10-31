#[derive(Debug, sqlx::FromRow)]
pub struct FileModel {
    pub path: String,
}