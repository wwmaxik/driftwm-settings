use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::i18n::Language;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppSettings {
    pub language: Language,
    pub auto_validate: bool,
    pub custom_config_path: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        // Detect system language
        let language = if let Ok(lang) = std::env::var("LANG") {
            if lang.to_lowercase().starts_with("ru") {
                Language::Russian
            } else {
                Language::English
            }
        } else if let Ok(lc_all) = std::env::var("LC_ALL") {
            if lc_all.to_lowercase().starts_with("ru") {
                Language::Russian
            } else {
                Language::English
            }
        } else {
            Language::English
        };

        Self {
            language,
            auto_validate: true,
            custom_config_path: None,
        }
    }
}

impl AppSettings {
    pub fn settings_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("driftwm-settings")
            .join("settings.toml")
    }

    pub fn load() -> Self {
        let path = Self::settings_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(settings) = toml::from_str::<AppSettings>(&content) {
                    return settings;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::settings_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let content = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}
