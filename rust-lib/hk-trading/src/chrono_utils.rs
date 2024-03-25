use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

// impl From<i64> for DateTime<Utc> {
pub fn convert_i64_to_datetime_utc(i: i64) -> DateTime<Utc> {
    let native_time = NaiveDateTime::from_timestamp_opt(i, 0).unwrap();
    Utc.from_utc_datetime(&native_time)
}
