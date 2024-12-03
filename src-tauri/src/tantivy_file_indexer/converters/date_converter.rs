use std::time::{SystemTime, UNIX_EPOCH};
use tantivy::DateTime;

pub fn unix_time_to_tantivy_datetime(timestamp: u64) -> DateTime {
    let system_time = UNIX_EPOCH + std::time::Duration::from_secs(timestamp);

    let duration_since_epoch = system_time.duration_since(UNIX_EPOCH).unwrap();
    let timestamp_secs = duration_since_epoch.as_secs(); // Unix timestamp in seconds
    DateTime::from_timestamp_secs(timestamp_secs as i64)
}

pub fn chrono_time_to_tantivy_datetime(chrono_dt: chrono::DateTime<chrono::Utc>) -> DateTime {
    let timestamp_secs = chrono_dt.timestamp();
    DateTime::from_timestamp_secs(timestamp_secs)
}

pub fn system_time_to_chrono_datetime(system_time: SystemTime) -> chrono::DateTime<chrono::Utc> {
    // Convert the SystemTime to a duration since the UNIX epoch
    let duration_since_epoch = system_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    
    // Extract seconds and nanoseconds
    let secs = duration_since_epoch.as_secs() as i64;
    let nanos = duration_since_epoch.subsec_nanos();

    // Create a NaiveDateTime from seconds and nanoseconds
    let naive_datetime = chrono::NaiveDateTime::from_timestamp(secs, nanos);

    // Convert NaiveDateTime to DateTime<Utc>
    chrono::DateTime::<chrono::Utc>::from_utc(naive_datetime, chrono::Utc)
}