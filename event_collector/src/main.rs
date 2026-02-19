mod config;
mod models;
mod services;
mod utils;

use chrono::Local;
use futures::future::join_all;
use models::event::Event;
use models::provider_b_api::EventTemplate;
use services::{
    database::DatabaseService,
    event_etl::{ETL, ProviderBIngestEvent, ProviderAIngestEvent},
    geocoding::GeocodingService,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Local::now();
    println!(
        "{} -- Started event collecting...",
        start.format("%Y-%m-%d %H:%M:%S")
    );

    // Fetch events from ProviderA
    let provider_a = ProviderAIngestEvent;
    let provider_a_raw_events = provider_a.extract().await?;
    if provider_a_raw_events.is_empty() {
        println!("No events found from ProviderA...");
    }

    // Transform events
    let provider_a_events = provider_a.transform(provider_a_raw_events)?;

    // Setup database
    let db = Arc::new(
        DatabaseService::new("host=[REDACTED_HOST] user=[REDACTED_USER] dbname=[REDACTED_DB]")
            .await?,
    );
    // Process events (geocoding + database insertion)
    process_events(provider_a_events, db.clone()).await?;

    // Fetch events from ProviderB
    let provider_b = ProviderBIngestEvent;
    let provider_b_events = provider_b.extract().await?;
    process_provider_b_events(provider_b_events, db).await?;

    let end = Local::now();
    println!(
        "{} -- Finished event collecting",
        end.format("%Y-%m-%d %H:%M:%S")
    );
    println!();

    Ok(())
}

async fn process_events(
    events: Vec<Event>,
    db: Arc<DatabaseService>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting geocoding for {} events...", events.len());

    let geocoding_tasks: Vec<_> = events
        .iter()
        .map(|event| GeocodingService::geocode_address(&event.event_name))
        .collect();

    let coordinates_results = join_all(geocoding_tasks).await;
    println!("Geocoding completed!");

    let insert_tasks: Vec<_> = events
        .into_iter()
        .zip(coordinates_results.into_iter())
        .filter_map(|(event, coord_result)| {
            let db = Arc::clone(&db);
            match coord_result {
                Ok(coord) => match GeocodingService::parse_to_postgis_point(&coord) {
                    Ok(postgis_data) => {
                        Some(async move { db.insert_event(&event, &postgis_data).await })
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to parse coordinates for {}: {}",
                            event.event_name, e
                        );
                        None
                    }
                },
                Err(e) => {
                    eprintln!("Geocoding failed for {}: {}", event.event_name, e);
                    None
                }
            }
        })
        .collect();

    println!(
        "Starting database insert for {} events...",
        insert_tasks.len()
    );
    let insert_results = join_all(insert_tasks).await;

    let mut successful_inserts = 0;
    for result in insert_results {
        match result {
            Ok(_) => successful_inserts += 1,
            Err(e) => eprintln!("Database insert failed: {}", e),
        }
    }

    println!(
        "Successfully inserted {} events into database",
        successful_inserts
    );

    Ok(())
}

async fn process_provider_b_events(
    events: Vec<EventTemplate>,
    db: Arc<DatabaseService>,
) -> Result<(), Box<dyn std::error::Error>> {
    let insert_tasks: Vec<_> = events
        .into_iter()
        .map(|event| {
            let db = Arc::clone(&db);
            async move { db.insert_provider_b_event(&event).await }
        })
        .collect();

    println!(
        "Starting database insert for {} events...",
        insert_tasks.len()
    );
    let insert_results = join_all(insert_tasks).await;

    let mut successful_inserts = 0;
    for result in insert_results {
        match result {
            Ok(_) => successful_inserts += 1,
            Err(e) => eprintln!("Database insert failed: {}", e),
        }
    }

    println!(
        "Successfully inserted {} events into database",
        successful_inserts
    );

    Ok(())
}
