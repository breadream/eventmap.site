use reqwest::Client;
use std::sync::OnceLock;

pub const GPT_ENDPOINT: &str = "https://redacted.invalid/v1/redacted";

pub struct Config {
    gpt_api_key: String,
    provider_b_api_key: String,
    http_client: Client,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

fn init_config() -> Config {
    Config {
        gpt_api_key: "[REDACTED_API_KEY]".to_string(),
        provider_b_api_key: "[REDACTED_API_KEY]".to_string(),
        http_client: Client::new(),
    }
}

pub fn config() -> &'static Config {
    CONFIG.get_or_init(init_config)
}

pub fn get_gpt_api_key() -> &'static str {
    &config().gpt_api_key
}
pub fn get_provider_b_api_key() -> &'static str {
    &config().provider_b_api_key
}
pub fn get_http_client() -> &'static Client {
    &config().http_client
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_api_key() {
        let gpt_api_key = get_gpt_api_key();
        let provider_b_api_key = get_provider_b_api_key();

        assert_eq!(gpt_api_key, "[REDACTED_API_KEY]");
        assert_eq!(provider_b_api_key, "[REDACTED_API_KEY]");
    }

    #[test]
    fn test_get_http_client() {
        let client_1 = get_http_client();
        let client_2 = get_http_client();

        assert!(std::ptr::eq(client_1, client_2));
    }
}
