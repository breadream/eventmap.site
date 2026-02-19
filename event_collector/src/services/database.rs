use crate::models::event::Event;
use crate::models::provider_b_api::EventTemplate;
use postgis::ewkb;
use std::sync::Arc;
use tokio;
use tokio_postgres::{Client, NoTls};

#[derive(Debug, Clone, PartialEq)]
pub enum DataSource {
    ProviderB,
    ProviderA,
}

impl DataSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            DataSource::ProviderB => "provider_b",
            DataSource::ProviderA => "provider_a",
        }
    }
}

pub struct DatabaseService {
    client: Arc<Client>,
}

impl DatabaseService {
    pub async fn new(connection_string: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;
        let client = Arc::new(client);

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self { client })
    }

    pub async fn insert_event(
        &self,
        event: &Event,
        location: &ewkb::Point,
    ) -> Result<u64, tokio_postgres::Error> {
        let insert_stmt = "[REDACTED_SQL_STATEMENT]";

        self.client
            .execute(
                insert_stmt,
                &[],
            )
            .await
    }

    pub async fn insert_provider_b_event(
        &self,
        event: &EventTemplate,
    ) -> Result<u64, tokio_postgres::Error> {
        let insert_stmt = "[REDACTED_SQL_STATEMENT]";

        self.client
            .execute(
                insert_stmt,
                &[],
            )
            .await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_new_valid_connection() {
        let conn_str = "host=[REDACTED_HOST] user=[REDACTED_USER] password=[REDACTED_PASSWORD] dbname=[REDACTED_DB]";
        let service = DatabaseService::new(conn_str).await;
        assert!(
            service.is_err(),
            "Expected successful connection to the database"
        );
    }

    #[tokio::test]
    async fn test_new_invalid_connection() {
        let invalid_conn_str = "what is this...";
        let service = DatabaseService::new(invalid_conn_str).await;
        assert!(
            service.is_err(),
            "Expected error for invalid connection string"
        );
    }
}
