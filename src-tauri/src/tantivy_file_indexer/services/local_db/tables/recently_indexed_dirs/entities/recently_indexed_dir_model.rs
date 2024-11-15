use sqlx::prelude::FromRow;

#[derive(FromRow)]
pub struct RecentlyIndexedDirModel {
    path: String,
    indexed_at: i64, // unix timestamp
}
