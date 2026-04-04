//! Application configuration: model registry and persistence.
//!
//! The config file lives at `~/.settl/config.toml` and stores a list of
//! AI model entries (llamafiles and API endpoints) that the user can manage
//! from the Settings screen.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A single model configuration in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEntry {
    /// User-visible name, e.g. "Bonsai 1.7B" or "Claude Sonnet".
    pub name: String,
    /// What kind of backend this model uses.
    pub backend: ModelBackend,
}

/// The backend type for a model entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelBackend {
    /// A llamafile to download and run locally.
    Llamafile {
        /// Download URL for the .llamafile binary.
        url: String,
        /// Filename on disk (inside `~/.settl/llamafiles/`).
        filename: String,
    },
    /// A remote API endpoint (Anthropic Messages API compatible).
    Api {
        /// Base URL, e.g. "https://api.anthropic.com".
        base_url: String,
        /// API key (empty string means no auth / local server).
        api_key: String,
        /// Model identifier sent in the request, e.g. "claude-sonnet-4-20250514".
        model: String,
    },
}

/// Top-level application config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Registered AI models.
    pub models: Vec<ModelEntry>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            models: vec![
                ModelEntry {
                    name: "Bonsai 1.7B (fast)".into(),
                    backend: ModelBackend::Llamafile {
                        url: "https://huggingface.co/mozilla-ai/llamafile_0.10.0/resolve/main/Bonsai-1.7B.llamafile?download=true".into(),
                        filename: "Bonsai-1.7B.llamafile".into(),
                    },
                },
                ModelEntry {
                    name: "Bonsai 8B (smart)".into(),
                    backend: ModelBackend::Llamafile {
                        url: "https://huggingface.co/mozilla-ai/llamafile_0.10.0/resolve/main/Bonsai-8B.llamafile?download=true".into(),
                        filename: "Bonsai-8B.llamafile".into(),
                    },
                },
            ],
        }
    }
}

/// Return the path to `~/.settl/config.toml`.
pub fn config_path() -> PathBuf {
    let home = std::env::var("HOME")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| ".".into());
    PathBuf::from(home).join(".settl").join("config.toml")
}

/// Load config from disk, falling back to defaults if missing or malformed.
pub fn load_config() -> Config {
    let path = config_path();
    match std::fs::read_to_string(&path) {
        Ok(data) => toml::from_str(&data).unwrap_or_default(),
        Err(_) => Config::default(),
    }
}

/// Save config to disk.
pub fn save_config(config: &Config) -> Result<(), String> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("mkdir: {e}"))?;
    }
    let toml = toml::to_string_pretty(config).map_err(|e| format!("serialize: {e}"))?;
    std::fs::write(&path, toml).map_err(|e| format!("write: {e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_two_models() {
        let config = Config::default();
        assert_eq!(config.models.len(), 2);
        assert!(config.models[0].name.contains("1.7B"));
        assert!(config.models[1].name.contains("8B"));
    }

    #[test]
    fn roundtrip_toml() {
        let config = Config {
            models: vec![
                ModelEntry {
                    name: "Test Llamafile".into(),
                    backend: ModelBackend::Llamafile {
                        url: "https://example.com/model.llamafile".into(),
                        filename: "model.llamafile".into(),
                    },
                },
                ModelEntry {
                    name: "Test API".into(),
                    backend: ModelBackend::Api {
                        base_url: "https://api.example.com".into(),
                        api_key: "sk-test".into(),
                        model: "test-model".into(),
                    },
                },
            ],
        };

        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.models.len(), 2);
        assert_eq!(parsed.models[0].name, "Test Llamafile");
        assert_eq!(parsed.models[1].name, "Test API");

        match &parsed.models[0].backend {
            ModelBackend::Llamafile { url, filename } => {
                assert_eq!(url, "https://example.com/model.llamafile");
                assert_eq!(filename, "model.llamafile");
            }
            _ => panic!("Expected Llamafile backend"),
        }

        match &parsed.models[1].backend {
            ModelBackend::Api {
                base_url,
                api_key,
                model,
            } => {
                assert_eq!(base_url, "https://api.example.com");
                assert_eq!(api_key, "sk-test");
                assert_eq!(model, "test-model");
            }
            _ => panic!("Expected Api backend"),
        }
    }
}
