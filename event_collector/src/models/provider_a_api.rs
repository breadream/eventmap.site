use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CustomField {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct RawEvent {
    #[serde(rename = "title")]
    pub event_name: String,
    #[serde(rename = "location")]
    pub venue_address: String,
    #[serde(rename = "startDateTime")]
    pub start_time: String,
    #[serde(rename = "endDateTime")]
    pub end_time: String,
    #[serde(rename = "customFields")]
    pub custom_fields: Vec<CustomField>,
}
