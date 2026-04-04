//! Anthropic API model discovery.
//!
//! Detects the `ANTHROPIC_API_KEY` environment variable, verifies the key
//! against the Anthropic API, and fetches the list of available models.

use crate::config::{ModelBackend, ModelEntry};

const ANTHROPIC_BASE_URL: &str = "https://api.anthropic.com";
const ANTHROPIC_VERSION: &str = "2023-06-01";

/// A model returned by the Anthropic list models API.
#[derive(Debug, Clone)]
pub struct AnthropicModel {
    pub id: String,
    pub display_name: String,
}

/// Check for `ANTHROPIC_API_KEY` in the environment.
pub fn detect_api_key() -> Option<String> {
    std::env::var("ANTHROPIC_API_KEY")
        .ok()
        .filter(|s| !s.is_empty())
}

/// Fetch available models from the Anthropic API.
/// Returns the models sorted by recency (API default).
pub async fn list_models(api_key: &str) -> Result<Vec<AnthropicModel>, String> {
    let client = reqwest::Client::builder()
        .connect_timeout(std::time::Duration::from_secs(5))
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let url = format!("{ANTHROPIC_BASE_URL}/v1/models?limit=100");
    let resp = client
        .get(&url)
        .header("x-api-key", api_key)
        .header("anthropic-version", ANTHROPIC_VERSION)
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("API returned {status}: {body}"));
    }

    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("JSON parse error: {e}"))?;

    let data = body
        .get("data")
        .and_then(|v| v.as_array())
        .ok_or("Missing 'data' array in response")?;

    let models: Vec<AnthropicModel> = data
        .iter()
        .filter_map(|entry| {
            let id = entry.get("id")?.as_str()?;
            let display_name = entry.get("display_name")?.as_str()?;
            Some(AnthropicModel {
                id: id.to_string(),
                display_name: display_name.to_string(),
            })
        })
        .collect();

    Ok(models)
}

/// Convert discovered Anthropic models into config entries.
pub fn to_model_entries(api_key: &str, models: &[AnthropicModel]) -> Vec<ModelEntry> {
    models
        .iter()
        .map(|m| ModelEntry {
            name: m.display_name.clone(),
            backend: ModelBackend::Api {
                base_url: ANTHROPIC_BASE_URL.to_string(),
                api_key: api_key.to_string(),
                model: m.id.clone(),
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_api_key_reads_env() {
        // This test just verifies the function doesn't panic.
        // Actual env var presence depends on the environment.
        let _ = detect_api_key();
    }

    #[test]
    fn to_model_entries_converts_correctly() {
        let models = vec![
            AnthropicModel {
                id: "claude-sonnet-4-20250514".into(),
                display_name: "Claude Sonnet 4".into(),
            },
            AnthropicModel {
                id: "claude-opus-4-20250514".into(),
                display_name: "Claude Opus 4".into(),
            },
        ];

        let entries = to_model_entries("sk-test-key", &models);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "Claude Sonnet 4");
        assert_eq!(entries[1].name, "Claude Opus 4");

        match &entries[0].backend {
            ModelBackend::Api {
                base_url,
                api_key,
                model,
            } => {
                assert_eq!(base_url, "https://api.anthropic.com");
                assert_eq!(api_key, "sk-test-key");
                assert_eq!(model, "claude-sonnet-4-20250514");
            }
            _ => panic!("Expected Api backend"),
        }
    }
}
