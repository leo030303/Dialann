use chrono::NaiveDateTime;

pub fn convert_text_to_timestamp(date_string: String) -> i64 {
    NaiveDateTime::parse_from_str(&format!("{} 01:01:01", date_string), "%Y-%m-%d %H:%M:%S")
        .unwrap()
        .and_utc()
        .timestamp()
}
