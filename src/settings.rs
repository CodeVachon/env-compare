use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

/// User preferences, persisted as JSON in the platform config directory.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub dark_mode: bool,
    /// Custom font family. Empty means "use the theme's monospace font".
    pub font_family: String,
}

impl Settings {
    fn path() -> Option<PathBuf> {
        Some(
            dirs::config_dir()?
                .join("env-compare")
                .join("settings.json"),
        )
    }

    /// Load saved settings, falling back to defaults if missing or unreadable.
    pub fn load() -> Self {
        Self::path()
            .and_then(|path| fs::read_to_string(path).ok())
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default()
    }

    /// Best-effort save; a failure only means preferences aren't remembered.
    pub fn save(&self) {
        let Some(path) = Self::path() else { return };
        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_fields_fall_back_to_defaults() {
        let settings: Settings = serde_json::from_str(r#"{"dark_mode": true}"#).unwrap();
        assert!(settings.dark_mode);
        assert_eq!(settings.font_family, "");
    }
}
