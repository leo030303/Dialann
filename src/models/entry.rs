use serde::{Deserialize, Serialize};

use super::mood::Mood;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub eid: Option<i64>,
    pub date_created: i64,
    pub content: String,
    pub mood: i8,
}

impl Entry {
    pub fn new(date_created: i64) -> Self {
        Self {
            eid: None,
            date_created,
            content: String::new(),
            mood: Mood::Okay.to_int(),
        }
    }
}
