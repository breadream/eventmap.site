use crate::config::get_provider_b_api_key;
use crate::models::event::Event;
use crate::models::provider_b_api::{ApiResponse, EventTemplate, ProviderBEvent};
use crate::models::provider_a_api::RawEvent;
use crate::utils::datetime::{
    datetime_str_to_utc, get_one_week_dates, get_one_week_dates_iso8601, parse_date,
};
use crate::utils::text_processing::{
    extract_number, extract_url_from_field, get_predefined_attendance_provider_b,
};
use async_trait::async_trait;
use html_escape::decode_html_entities;

#[async_trait]
pub trait ETL {
    // Raw Events
    type EventList;
    // Transform
    type OriginalEvent;
    type ProcessedEvent;
    type ExtractError;
    type TransformError;

    async fn extract(&self) -> Result<Self::EventList, Self::ExtractError>;
    fn transform(
        &self,
        input: Self::OriginalEvent,
    ) -> Result<Self::ProcessedEvent, Self::TransformError>;
}

pub struct ProviderAIngestEvent;
pub struct ProviderBIngestEvent;

#[async_trait]
impl ETL for ProviderAIngestEvent {
    type EventList = Vec<RawEvent>;
    type OriginalEvent = Vec<RawEvent>;
    type ProcessedEvent = Vec<Event>;
    type ExtractError = Box<dyn std::error::Error>;
    type TransformError = Box<dyn std::error::Error>;

    async fn extract(&self) -> Result<Self::EventList, Self::ExtractError> {
        let (today_str, one_week_later_str) = get_one_week_dates();
        let url = format!(
            "https://redacted.invalid/api/events?startdate={today}&enddate={later}",
            today = today_str,
            later = one_week_later_str
        );
        let raw_events: Vec<RawEvent> = reqwest::get(url).await?.json().await?;
        Ok(raw_events)
    }

    fn transform(
        &self,
        raw_events: Self::OriginalEvent,
    ) -> Result<Self::ProcessedEvent, Self::TransformError> {
        raw_events
            .into_iter()
            .map(|raw| {
                let attendance = raw
                    .custom_fields
                    .iter()
                    .find(|f| f.label == "Participants")
                    .map(|f| f.value.clone())
                    .unwrap_or_else(|| "0".to_string());

                let website = raw
                    .custom_fields
                    .iter()
                    .find(|f| f.label == "Website")
                    .and_then(|f| extract_url_from_field(&f.value))
                    .unwrap_or_else(|| String::new()); // or use None if website 

                Ok(Event {
                    event_name: decode_html_entities(&raw.event_name).to_string(),
                    venue_address: decode_html_entities(&raw.venue_address).to_string(),
                    start_time: datetime_str_to_utc(&raw.start_time),
                    end_time: datetime_str_to_utc(&raw.end_time),
                    attendance: extract_number(&attendance),
                    website: Some(website),
                })
            })
            .collect()
    }
}

#[async_trait]
impl ETL for ProviderBIngestEvent {
    type EventList = Vec<EventTemplate>;
    type OriginalEvent = ProviderBEvent;
    type ProcessedEvent = EventTemplate;
    type ExtractError = Box<dyn std::error::Error>;
    type TransformError = chrono::format::ParseError;

    async fn extract(&self) -> Result<Self::EventList, Self::ExtractError> {
        let (today_str, one_week_later_str) = get_one_week_dates_iso8601();
        let mut all_events = Vec::new();
        let mut page_num = 0;
        loop {
            let url = format!(
                "https://redacted.invalid/api/events?apikey={REDACTED_PROVIDER_B_API_KEY}&scope=[REDACTED]&startDateTime={today}&endDateTime={later}&page={page_num}",
                REDACTED_PROVIDER_B_API_KEY = get_provider_b_api_key(),
                today = today_str,
                later = one_week_later_str,
                page_num = page_num
            );
            // DEBUG mode on
            // let response = reqwest::get(url).await?;
            // let raw_json = response.text().await?;
            // let parsed: serde_json::Value = serde_json::from_str(&raw_json)?;
            // println!("{:#}", parsed);
            // DEBUG mode on

            // Process events from current page
            let api_responses: ApiResponse = reqwest::get(url).await?.json().await?;
            for event in api_responses.embedded.events.into_iter() {
                // println!("{:?}", event);
                let event_info = self.transform(event)?;
                all_events.push(event_info)
            }
            // Check if we've fetched all pages
            if page_num >= api_responses.page.total_pages - 1 {
                break;
            }

            page_num += 1;
        }

        Ok(all_events)
    }

    fn transform(
        &self,
        event: Self::OriginalEvent,
    ) -> Result<Self::ProcessedEvent, Self::TransformError> {
        let start_time = parse_date(&event.dates.start.as_ref().unwrap().date_time)?;
        let end_time = parse_date(&event.dates.end.as_ref().and_then(|e| e.date_time.clone()))?;

        // Venue address: take the first venue's line1 or fallback
        let venue_address = event
            .embedded
            .as_ref()
            .and_then(|e| e.venues.get(0))
            .and_then(|v| v.address.as_ref().map(|a| a.line1.clone()))
            .unwrap_or_else(|| "Unknown Venue".to_string());

        let coordinates = event
            .embedded
            .as_ref()
            .and_then(|e| e.venues.get(0))
            .and_then(|v| v.location.as_ref())
            .map(|loc| {
                (
                    loc.latitude.parse().unwrap_or(0.0),
                    loc.longitude.parse().unwrap_or(0.0),
                )
            })
            .unwrap_or((0.0, 0.0));

        let (lat, lon) = coordinates;

        let event_url = event.url.clone().unwrap_or_else(|| String::new());
        let venue_attendance = get_predefined_attendance_provider_b(&venue_address);

        Ok(EventTemplate {
            event_name: event.name.clone(),
            venue_address,
            start_time,
            end_time,
            attendance: venue_attendance,
            latitude: lat,
            longitude: lon,
            event_url: Some(event_url),
        })
    }
}
