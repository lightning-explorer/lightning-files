use std::{future::Future, time::Duration};

use sea_orm::DbErr;

/**
Retry the asynchronous database operation until it succeeds or the retry amount is met. If an error unrelated to a database lock is returned, then the retries will automatically be stopped
*/
pub async fn retry_on_locked<F, Fut, T>(operation: F, retry_amount: usize) -> Result<T, DbErr>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, DbErr>>,
{
    for _ in 0..retry_amount {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(DbErr::Exec(_)) => {
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
            Err(e) => return Err(e),
        }
    }
    Err(DbErr::Custom(format!(
        "Database is locked after {} failed attempts",
        retry_amount
    )))
}
