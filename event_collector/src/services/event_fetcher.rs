use crate::config::get_provider_b_api_key;
use crate::models::provider_b_api::{ApiResponse, EventTemplate, ProviderBEvent};
use crate::models::provider_a_api::RawEvent;
use crate::utils::datetime::{get_one_week_dates, get_one_week_dates_iso8601};
use chrono::format::ParseError;
use chrono::{DateTime, Utc};

pub struct EventFetcher;

impl EventFetcher {
    pub async fn fetch_events() -> Result<Vec<RawEvent>, Box<dyn std::error::Error>> {
        let (today_str, one_week_later_str) = get_one_week_dates();
        let url = format!(
            "https://redacted.invalid/api/events?startdate={today}&enddate={later}",
            today = today_str,
            later = one_week_later_str
        );
        let raw_events: Vec<RawEvent> = reqwest::get(url).await?.json().await?;
        Ok(raw_events)
    }

    pub async fn fetch_provider_b_events() -> Result<Vec<EventTemplate>, Box<dyn std::error::Error>> {
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
            for event in api_responses.embedded.events.iter() {
                // println!("{:?}", event);
                let event_info = Self::extract_event_info(event)?;
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

    fn extract_event_info(event: &ProviderBEvent) -> Result<EventTemplate, chrono::format::ParseError> {
        let start_time = Self::parse_date(&event.dates.start.as_ref().unwrap().date_time)?;
        let end_time =
            Self::parse_date(&event.dates.end.as_ref().and_then(|e| e.date_time.clone()))?;

        // Venue address: take the first venue's line1 or fallback
        let venue_address = event
            .embedded
            .as_ref()
            .and_then(|e| e.venues.get(0))
            .map(|v| v.address.line1.clone())
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

        Ok(EventTemplate {
            event_name: event.name.clone(),
            venue_address,
            start_time,
            end_time,
            attendance: Some(0), // ProviderB API does not provide attendance
            latitude: lat,
            longitude: lon,
            event_url: Some(event_url),
        })
    }

    fn parse_date(date_opt: &Option<String>) -> Result<Option<DateTime<Utc>>, ParseError> {
        if let Some(date_str) = date_opt {
            date_str.parse::<DateTime<Utc>>().map(Some)
        } else {
            Ok(None)
        }
    }
}
