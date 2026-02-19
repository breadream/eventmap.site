# eventmap.site

Portfolio monorepo for an event mapping pipeline and web map experience.

This repository is intentionally **redacted for public sharing**. Sensitive values, provider-specific identifiers, SQL payloads, infrastructure details, and operational configs have been replaced with placeholders.

## Repository Structure

- `event_collector/`
  - Rust-based ingestion/ETL service.
  - Handles event normalization, geocoding requests, and DB write flow.
  - Core areas:
    - `src/services/` for ingestion, transform, geocoding, and persistence services.
    - `src/models/` for API/data models.
    - `src/utils/` for datetime and text helpers.

- `gps_api_server/`
  - Rust API service (Actix Web) exposing coordinate/event data to the frontend.
  - Includes CORS setup, DB connectivity, and response shaping.

- `map_visualizer/`
  - React + Vite frontend for rendering map markers and event details.
  - Includes date filters and map UI interactions.

- `terraform/`
  - Infrastructure-as-code folder.
  - Contents are currently redacted for public portfolio safety.

## High-Level Data Flow

1. Collector pulls event data from external providers.
2. Collector transforms/normalizes records.
3. Geocoding step derives coordinate outputs.
4. Data persistence layer writes to database.
5. API server reads records and serves map-ready JSON.
6. Frontend fetches API data and renders an interactive map.

## Redaction Notes

This public version has placeholders like:

- `[REDACTED_API_KEY]`
- `[REDACTED_TOKEN]`
- `[REDACTED_SQL_STATEMENT]`
- `redacted.invalid`

As a result, this repository is intended for **architecture/code review** and **portfolio presentation**, not direct deployment.

## Tech Stack

- Rust (Tokio, Actix Web, tokio-postgres)
- React + Vite
- Terraform (redacted in this public snapshot)

## License

MIT (see `LICENSE`).
