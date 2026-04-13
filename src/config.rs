use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DriftwmConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_follows_mouse: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autostart: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<InputConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<CursorConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation: Option<NavigationConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<ZoomConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap: Option<SnapConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<KeyboardConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trackpad: Option<TrackpadConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll: Option<ScrollConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyboardConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_rate: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_delay: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrackpadConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tap_to_click: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub natural_scroll: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tap_and_drag: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accel_speed: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScrollConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub friction: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CursorConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NavigationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nudge_step: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZoomConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fit_padding: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnapConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
}

impl DriftwmConfig {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
