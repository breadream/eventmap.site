use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "_embedded")]
    pub embedded: Embedded,
    pub page: Page,
}

#[derive(Debug, Deserialize)]
pub struct Embedded {
    pub events: Vec<ProviderBEvent>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderBEvent {
    pub name: String,
    #[serde(rename = "_embedded")]
    pub embedded: Option<EventEmbedded>,
    pub dates: Dates,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EventEmbedded {
    pub venues: Vec<Venue>,
}

#[derive(Debug, Deserialize)]
pub struct Venue {
    pub address: Option<Address>,
    pub location: Option<GpsLocation>,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    pub line1: String,
}

#[derive(Debug, Deserialize)]
pub struct GpsLocation {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Debug, Deserialize)]
pub struct Dates {
    pub start: Option<DateTimeInfo>,
    pub end: Option<DateTimeInfo>,
}

#[derive(Debug, Deserialize)]
pub struct DateTimeInfo {
    #[serde(rename = "dateTime")]
    pub date_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Page {
    #[serde(rename = "totalPages")]
    pub total_pages: u32,
}

#[derive(Debug, Deserialize)]
pub struct EventTemplate {
    pub event_name: String,
    pub venue_address: String,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub attendance: Option<i32>,
    pub latitude: f64,
    pub longitude: f64,
    pub event_url: Option<String>,
}
