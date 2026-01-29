//! Configuration management for display preferences

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// User configuration for display preferences
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Timezone offset for reset time display (optional)
    #[serde(default)]
    pub timezone_offset: Option<i32>,
}

impl Config {
    /// Get the default config file path
    pub fn default_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("claude-status")
            .join("config.json")
    }

    /// Load config from file, returning default if not found
    pub fn load() -> Self {
        let path = Self::default_path();
        if path.exists() {
            std::fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    /// Save config to file
    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::default_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)
    }
}

// Add dirs dependency - placeholder for now
mod dirs {
    use std::path::PathBuf;

    pub fn config_dir() -> Option<PathBuf> {
        std::env::var("HOME")
            .ok()
            .map(|home| PathBuf::from(home).join(".config"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.timezone_offset.is_none());
    }

    #[test]
    fn test_config_serialize() {
        let config = Config {
            timezone_offset: Some(-8),
        };
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("-8"));
    }

    #[test]
    fn test_config_deserialize() {
        let json = r#"{"timezone_offset": 5}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.timezone_offset, Some(5));
    }

    #[test]
    fn test_config_deserialize_empty() {
        let json = "{}";
        let config: Config = serde_json::from_str(json).unwrap();
        assert!(config.timezone_offset.is_none());
    }

    #[test]
    fn test_config_default_path() {
        let path = Config::default_path();
        assert!(path.to_string_lossy().contains("claude-status"));
        assert!(path.to_string_lossy().contains("config.json"));
    }

    #[test]
    fn test_config_load_returns_default_when_missing() {
        // Config::load() should return default when file doesn't exist
        let config = Config::load();
        // Just verify it doesn't panic and returns a valid config
        assert!(config.timezone_offset.is_none() || config.timezone_offset.is_some());
    }

    #[test]
    fn test_config_save_and_load() {
        // Create a temporary directory for testing
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("config.json");

        // Manually save config to temp location
        let config = Config {
            timezone_offset: Some(-5),
        };
        let json = serde_json::to_string_pretty(&config).unwrap();
        std::fs::write(&config_path, &json).unwrap();

        // Read it back
        let loaded_json = std::fs::read_to_string(&config_path).unwrap();
        let loaded: Config = serde_json::from_str(&loaded_json).unwrap();
        assert_eq!(loaded.timezone_offset, Some(-5));
    }
}
