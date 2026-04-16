use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DriftwmConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_follows_mouse: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autostart: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decorations: Option<DecorationsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<EffectsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<OutputConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<BackgroundConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend: Option<BackendConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keybindings: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_rules: Option<Vec<WindowRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyboard: Option<KeyboardConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trackpad: Option<TrackpadConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouse: Option<MouseConfig>,
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
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_rate: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_delay: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_independent: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_lock: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caps_lock: Option<bool>,
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accel_profile: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MouseConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accel_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accel_profile: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub natural_scroll: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CursorConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_opacity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NavigationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trackpad_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouse_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub friction: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nudge_step: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan_step: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchors: Option<Vec<[f64; 2]>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub edge_pan: Option<EdgePanConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EdgePanConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_max: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZoomConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fit_padding: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_on_new_window: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_on_activation: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnapConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub break_force: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_edge: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DecorationsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EffectsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur_radius: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur_strength: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutputConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline: Option<OutputOutlineConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutputOutlineConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thickness: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackgroundConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shader_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackendConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_frame_completion: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_direct_scanout: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WindowRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<[i32; 2]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<[i32; 2]>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
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
