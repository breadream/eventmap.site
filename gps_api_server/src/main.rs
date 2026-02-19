use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Result, get, web};
use chrono::{DateTime, Utc};
use chrono_tz::US::Pacific;
use num_format::{Locale, ToFormattedString};
use postgis::Point;
use postgis::ewkb;
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventInfo {
    pub latitude: f64,
    pub longitude: f64,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub label: Option<String>,
    pub attendance: Option<String>,
    pub website: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub coordinates: Vec<EventInfo>,
    pub total: usize,
}

async fn fetch_coordinates_from_db(
    client: &tokio_postgres::Client,
) -> Result<Vec<EventInfo>, tokio_postgres::Error> {
    let rows = client
        .query(
            "[REDACTED_SQL_STATEMENT]",
            &[],
        )
        .await?;

    let mut coords = Vec::new();
    for row in rows {
        let event_name: String = row.get("event_name");
        let point: ewkb::Point = row.get("venue_location");
        let start_time: Option<DateTime<Utc>> = row.get("start_time");
        let end_time: Option<DateTime<Utc>> = row.get("end_time");
        let attendance: i32 = row.get("attendance");
        let website: String = row.get("website");

        coords.push(EventInfo {
            latitude: point.y(),
            longitude: point.x(),
            start_time: start_time.map(|dt| {
                let local_time = dt.with_timezone(&Pacific);
                local_time.format("%Y-%m-%d · %I:%M %p").to_string()
            }),
            end_time: end_time.map(|dt| {
                let local_time = dt.with_timezone(&Pacific);
                local_time.format("%Y-%m-%d · %I:%M %p").to_string()
            }),
            label: Some(event_name),
            attendance: Some(format!(
                "{}",
                match attendance {
                    0 => "not specified".to_string(),
                    n => n.to_formatted_string(&Locale::en) + " (estimated)",
                }
            )),
            website: Some(website),
        });
    }
    Ok(coords)
}

#[get("/api/coordinates")]
async fn get_coordinates(db: web::Data<tokio_postgres::Client>) -> Result<HttpResponse> {
    let coordinates = fetch_coordinates_from_db(&db).await.map_err(|e| {
        eprintln!("Database query failed: {}", e);
        actix_web::error::ErrorInternalServerError("Failed to fetch coordinates")
    })?;

    let response = ApiResponse {
        total: coordinates.len(),
        coordinates,
    };

    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (client, connection) =
        tokio_postgres::connect(
            "host=[REDACTED_HOST] user=[REDACTED_USER] dbname=[REDACTED_DB]",
            NoTls,
        )
            .await
            .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let app_client = web::Data::new(client);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://redacted.invalid")
            .allowed_origin("https://redacted-api.invalid")
            .allowed_origin("https://redacted-web.invalid")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["content-type", "x-redacted-auth", "Accept"])
            .max_age(3600);

        App::new()
            .app_data(app_client.clone())
            .service(get_coordinates)
            .wrap(cors)
    })
    .bind("0.0.0.0:0000")?
    .run()
    .await
}
