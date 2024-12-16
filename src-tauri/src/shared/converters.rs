use std::time::{SystemTime, UNIX_EPOCH};

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
