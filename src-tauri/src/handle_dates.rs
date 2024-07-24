use chrono::{DateTime, NaiveDateTime};

pub fn convert_text_to_timestamp(date_string: String) -> i64 {
    NaiveDateTime::parse_from_str(&format!("{} 01:01:01", date_string), "%Y-%m-%d %H:%M:%S")
        .unwrap()
        .and_utc()
        .timestamp()
}

pub fn convert_timestamp_to_text(timestamp: i64) -> String {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string()
}
