use chrono::format::ParseError;
use chrono::{DateTime, Days, Duration, Local, NaiveDateTime, TimeZone, Utc};
use chrono_tz::US::Pacific;

pub fn datetime_str_to_utc(datetime_str: &str) -> DateTime<Utc> {
    let naive_dt = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%dT%H:%M:%S")
        .expect("Failed to parse datetime string");

    Pacific
        .from_local_datetime(&naive_dt)
        .single()
        .expect("Ambiguous or invalid local time")
        .with_timezone(&Utc)
}

pub fn get_one_week_dates() -> (String, String) {
    let today = Local::now().date_naive();
    let one_week_later = today + Days::new(7);

    let today_str = today.format("%Y%m%d").to_string();
    let one_week_later_str = one_week_later.format("%Y%m%d").to_string();
    (today_str, one_week_later_str)
}

pub fn get_one_week_dates_iso8601() -> (String, String) {
    let now = Utc::now();
    let one_week_later = now + Duration::days(7);

    let start = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let end = one_week_later.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    (start, end)
}

pub fn parse_date(date_opt: &Option<String>) -> Result<Option<DateTime<Utc>>, ParseError> {
    if let Some(date_str) = date_opt {
        date_str.parse::<DateTime<Utc>>().map(Some)
    } else {
        Ok(None)
    }
}
