use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub event_name: String,
    pub venue_address: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub attendance: i32,
    pub website: Option<String>,
}
