use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{NaiveDateTime, Utc};
use tantivy::{time::{OffsetDateTime, UtcOffset}, DateTime};


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

pub fn tantivy_time_to_chrono_datetime(tantivy_datetime:DateTime)->chrono::DateTime<Utc>{
    let offset = tantivy_datetime.into_offset(UtcOffset::UTC);

    let unix_timestamp = offset.unix_timestamp(); // Get seconds since UNIX epoch
    let nanos = offset.nanosecond(); // Get nanoseconds past the second

    // Create a NaiveDateTime from the timestamp
    let naive_datetime = NaiveDateTime::from_timestamp(unix_timestamp, nanos);

    // Convert NaiveDateTime to DateTime<Utc>
    chrono::DateTime::<Utc>::from_utc(naive_datetime, Utc)
}