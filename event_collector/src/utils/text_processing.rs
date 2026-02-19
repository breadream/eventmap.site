use regex::Regex;

pub fn extract_number(s: &str) -> i32 {
    // Match a group of digits that may contain commas (e.g., "1,000", "12,345")
    let re = Regex::new(r"\d{1,3}(?:,\d{3})*").unwrap();

    // Find the first match and remove commas before parsing
    re.find(s)
        .and_then(|m| m.as_str().replace(",", "").parse::<i32>().ok())
        .unwrap_or(i32::MAX)
}

pub fn extract_url_from_field(value: &str) -> Option<String> {
    // First decode Unicode escapes
    let decoded = value.replace(r#"\u003C"#, "<").replace(r#"\u003E"#, ">");

    // Decode HTML entities
    let html_decoded = decoded.replace("&#8230;", "…"); // ellipsis character

    // Extract URL using regex - look for href attribute
    let re = Regex::new(r#"href="([^"]+)""#).unwrap();
    if let Some(captures) = re.captures(&html_decoded) {
        Some(captures[1].to_string())
    } else {
        None
    }
}

pub fn get_predefined_attendance_provider_b(venue_address: &str) -> Option<i32> {
    if venue_address.contains("800 Occidental Ave S") {
        // Lumen Field
        Some(68740)
    } else if venue_address.contains("1250 1st Ave S") {
        // T-Mobile park
        Some(47929)
    } else if venue_address.contains("334 1st Ave N") {
        // Climate Pledge Arena
        Some(18300)
    } else if venue_address.contains("University of Washington") {
        Some(70138)
    } else {
        Some(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_number() {
        let raw_attendance: &str = "5,208 for all markets";
        let processed_attendance = extract_number(raw_attendance);
        assert_eq!(5208, processed_attendance);
    }

    #[test]
    fn test_extract_url_from_field() {
        let raw_url: &str = "\u{003Ca} href=\"https://redacted.invalid/\"
				target=\"_blank\"rel=\"noopener\"\u{003E}www.slumarket.com\u{003C}/a\u{003E}";
        let expected_result: Option<String> = Some(String::from("https://redacted.invalid/"));
        let processed_url = extract_url_from_field(raw_url);
        println!("{:?}", processed_url);
        assert_eq!(expected_result, processed_url);
    }
}
