use chrono::{DateTime, Utc};
use std::time::SystemTime;

/// Get timestamp in RFC3339 format
/// Coverts quickly from SystemTime to RFC3339 format using chrono
pub fn get_timestamp(sys_time: SystemTime) -> String {
    let datetime = DateTime::<Utc>::from(sys_time);
    datetime.to_rfc3339()
}
