use crate::config::{GPT_ENDPOINT, get_http_client};
use crate::models::gpt_api::{Message, OutputContent, ResponseBody, ResponseRequest};
use postgis::ewkb;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

pub struct GeocodingService;

impl GeocodingService {
    pub async fn geocode_address(address_info: &str) -> Result<String, Box<dyn std::error::Error>> {
        let system_instruction = "[REDACTED_SYSTEM_PROMPT]";

        let request_body = ResponseRequest {
            model: "gpt-4o-mini".to_string(),
            input: vec![
                Message {
                    role: "system".into(),
                    content: system_instruction.into(),
                },
                Message {
                    role: "user".into(),
                    content: address_info.into(),
                },
            ],
            max_output_tokens: 50,
            temperature: 0.0,
        };

        let client = get_http_client();

        let res = client
            .post(GPT_ENDPOINT)
            .header(AUTHORIZATION, "Bearer [REDACTED_TOKEN]")
            .header(CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .await?;

        // DEBUG
        // let status = res.status();
        // let text = res.text().await?;
        // eprintln!("status={status}, body={text}");
        // if !status.is_success() {
        //     return Err(format!("OpenAI error {status}: {text}").into());
        // }
        // let parsed: ResponseBody = serde_json::from_str(&text)?;

        let parsed: ResponseBody = res.json().await?;
        let coord = parsed
            .output
            .into_iter()
            .flat_map(|o| o.content.unwrap_or_default())
            .filter_map(|c| match c {
                OutputContent::OutputText { text } => Some(text),
            })
            .next()
            .unwrap_or_default();

        Ok(coord)
    }

    pub fn parse_to_postgis_point(coord_str: &str) -> Result<ewkb::Point, &'static str> {
        let (lat, lon) = Self::parse_coordinates(coord_str)?;
        Ok(ewkb::Point::new(lon, lat, Some(4326)))
    }

    fn parse_coordinates(coord_str: &str) -> Result<(f64, f64), &'static str> {
        let parts: Vec<&str> = coord_str.split(',').map(str::trim).collect();

        if parts.len() != 2 {
            return Err("Expected two values separated by a comma");
        }

        let lat = parts[0].parse().map_err(|_| "Failed to parse latitude")?;
        let lon = parts[1].parse().map_err(|_| "Failed to parse longitude")?;

        Ok((lat, lon))
    }
}
