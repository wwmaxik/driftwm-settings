use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use toml_edit::{Array, DocumentMut, Item, Table, Value};

/// Root driftwm configuration structure.
/// Matches driftwm 0.19+ schema.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DriftwmConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_follows_mouse: Option<bool>,

    /// Placement mode for newly mapped windows: "center" (default), "cursor", "auto"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_placement: Option<String>,

    /// Where a centering navigation parks the focused window in viewport:
    /// "center" (default), "top", "bottom", "left", "right", "top-left", "top-right", "bottom-left", "bottom-right"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_placement: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub autostart: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,

    #[serde(default)]
    pub session: SessionConfig,

    #[serde(default)]
    pub input: InputConfig,

    #[serde(default)]
    pub cursor: CursorConfig,

    #[serde(default)]
    pub navigation: NavigationConfig,

    #[serde(default)]
    pub zoom: ZoomConfig,

    #[serde(default)]
    pub snap: SnapConfig,

    #[serde(default)]
    pub decorations: DecorationsConfig,

    #[serde(default)]
    pub effects: EffectsConfig,

    #[serde(default)]
    pub background: BackgroundConfig,

    #[serde(default)]
    pub output: OutputConfig,

    #[serde(default)]
    pub backend: BackendConfig,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_rules: Option<Vec<WindowRule>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SessionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend_on_close: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_windows: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_camera: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_bookmarks: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct InputConfig {
    #[serde(default)]
    pub keyboard: KeyboardConfig,

    #[serde(default)]
    pub trackpad: TrackpadConfig,

    #[serde(default)]
    pub mouse: MouseConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remember_layout_per_window: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_while_typing: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_on_external_mouse: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MouseConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accel_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accel_profile: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub natural_scroll: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_handed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CursorConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_opacity: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NavigationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_navigate_on_close: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_navigate_on_click: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nudge_step: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_step: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan_step: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trackpad_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouse_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub touch_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchors: Option<Vec<[f64; 2]>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmarks: Option<HashMap<String, [f64; 2]>>,

    #[serde(default)]
    pub edge_pan: EdgePanConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgePanConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_max: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_pan: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_zone: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ZoomConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fit_padding: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_on_new_window: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_on_activation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interact_min: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trackpad_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouse_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub touch_speed: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_gap: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub break_force: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub corners: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub centers: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DecorationsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fg_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color_focused: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity_focused: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_bar_height: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_align: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EffectsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur_radius: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur_strength: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animate_blur_fps: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_speed: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_scale: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BackgroundConfig {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror_tile: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_shader: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transparent_shader: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_budget_mb: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animate_fps: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OutputConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline: Option<OutputOutlineConfig>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OutputOutlineConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thickness: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BackendConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_frame_completion: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_direct_scanout: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_hardware_cursor: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capture_fps: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
    pub fullscreen: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_on_open: Option<bool>,

    #[serde(default)]
    pub widget: bool,

    #[serde(default)]
    pub pinned_to_screen: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend_on_close: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_windows: Option<bool>,

    #[serde(default)]
    pub preserve_aspect_ratio: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoration: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color_focused: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_order: Option<i32>,
}

impl DriftwmConfig {
    /// Resolve default path to ~/.config/driftwm/config.toml
    pub fn default_path() -> PathBuf {
        if let Some(p) = std::env::var_os("DRIFTWM_CONFIG") {
            let s = p.to_string_lossy();
            if let Some(rest) = s.strip_prefix("~/")
                && let Some(home) = dirs::home_dir()
            {
                return home.join(rest);
            }
            return PathBuf::from(s.as_ref());
        }

        if let Some(cfg) = dirs::config_dir() {
            cfg.join("driftwm/config.toml")
        } else {
            PathBuf::from(".config/driftwm/config.toml")
        }
    }

    /// Load config from file. Returns (DriftwmConfig, raw_document_for_preserving_comments)
    pub fn load(path: &Path) -> Result<(Self, DocumentMut)> {
        if !path.exists() {
            let default_doc = Self::create_default_document();
            let config = Self::default_with_presets();
            return Ok((config, default_doc));
        }

        let content = std::fs::read_to_string(path)?;
        let doc: DocumentMut = content.parse().unwrap_or_else(|_| DocumentMut::new());
        let config: Self = toml::from_str(&content).unwrap_or_default();
        Ok((config, doc))
    }

    /// Save config while preserving existing user comments and ordering in DocumentMut.
    pub fn save(&self, doc: &mut DocumentMut, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        self.apply_to_document(doc);
        std::fs::write(path, doc.to_string())?;
        Ok(())
    }

    /// Default config populated with driftwm standard presets
    pub fn default_with_presets() -> Self {
        Self {
            mod_key: Some("super".to_string()),
            focus_follows_mouse: Some(false),
            window_placement: Some("center".to_string()),
            focus_placement: Some("center".to_string()),
            session: SessionConfig {
                suspend_on_close: Some(false),
                restore_windows: Some(false),
                restore_camera: Some(false),
                restore_bookmarks: Some(false),
            },
            input: InputConfig {
                keyboard: KeyboardConfig {
                    layout: Some("us".to_string()),
                    variant: Some("".to_string()),
                    options: Some("".to_string()),
                    model: Some("".to_string()),
                    repeat_rate: Some(25),
                    repeat_delay: Some(200),
                    layout_independent: Some(true),
                    num_lock: Some(true),
                    caps_lock: Some(false),
                    remember_layout_per_window: Some(false),
                },
                trackpad: TrackpadConfig {
                    tap_to_click: Some(true),
                    natural_scroll: Some(true),
                    tap_and_drag: Some(true),
                    accel_speed: Some(0.0),
                    accel_profile: Some("adaptive".to_string()),
                    click_method: Some("none".to_string()),
                    disable_while_typing: Some(true),
                    enable: Some(true),
                    disable_on_external_mouse: Some(false),
                },
                mouse: MouseConfig {
                    accel_speed: Some(0.0),
                    accel_profile: Some("flat".to_string()),
                    natural_scroll: Some(false),
                    left_handed: Some(false),
                },
            },
            cursor: CursorConfig {
                theme: Some("none".to_string()),
                size: Some(0),
                inactive_opacity: Some(0.5),
            },
            navigation: NavigationConfig {
                camera_speed: Some(0.3),
                auto_navigate_on_close: Some(true),
                auto_navigate_on_click: Some(false),
                nudge_step: Some(20),
                resize_step: Some(20),
                pan_step: Some(100.0),
                trackpad_speed: Some(1.5),
                mouse_speed: Some(1.0),
                touch_speed: Some(1.0),
                drift: Some(0.5),
                anchors: Some(vec![]),
                bookmarks: Some(HashMap::from([
                    ("1".to_string(), [-1750.0, 1750.0]),
                    ("2".to_string(), [1750.0, 1750.0]),
                    ("3".to_string(), [1750.0, -1750.0]),
                    ("4".to_string(), [-1750.0, -1750.0]),
                ])),
                edge_pan: EdgePanConfig {
                    zone: Some(100.0),
                    speed_min: Some(4.0),
                    speed_max: Some(10.0),
                    cursor_pan: Some(false),
                    cursor_zone: Some(20.0),
                    latency_ms: Some(120),
                },
            },
            zoom: ZoomConfig {
                step: Some(1.1),
                fit_padding: Some(80.0),
                reset_on_new_window: Some(true),
                reset_on_activation: Some(true),
                interact_min: Some(0.0),
                trackpad_speed: Some(1.0),
                mouse_speed: Some(1.0),
                touch_speed: Some(1.0),
            },
            snap: SnapConfig {
                enabled: Some(true),
                gap: Some(12.0),
                outer_gap: Some(0.0),
                distance: Some(24.0),
                break_force: Some(32.0),
                corners: Some(false),
                centers: Some(false),
            },
            decorations: DecorationsConfig {
                bg_color: Some("#303030".to_string()),
                fg_color: Some("#FFFFFF".to_string()),
                corner_radius: Some(10),
                default_mode: Some("client".to_string()),
                border_width: Some(0),
                border_color: Some("#303030".to_string()),
                border_color_focused: Some("#303030".to_string()),
                opacity: Some(1.0),
                opacity_focused: Some(1.0),
                blur: Some(false),
                shadow: Some(true),
                title_bar_height: Some(25),
                font: Some("Adwaita Sans".to_string()),
                font_size: Some(11),
                font_weight: Some("medium".to_string()),
                title_align: Some("center".to_string()),
            },
            effects: EffectsConfig {
                blur_radius: Some(2),
                blur_strength: Some(1.1),
                animate_blur_fps: Some(20),
                animation_speed: Some(0.5),
                animation_scale: Some(0.95),
            },
            background: BackgroundConfig {
                kind: Some("default".to_string()),
                path: None,
                texture: None,
                mirror_tile: Some(false),
                cache_shader: Some(false),
                transparent_shader: Some(false),
                cache_budget_mb: Some(128),
                animate_fps: Some(0),
            },
            output: OutputConfig::default(),
            backend: BackendConfig::default(),
            autostart: Some(vec![]),
            env: Some(HashMap::new()),
            window_rules: Some(vec![]),
        }
    }

    /// Create default template DocumentMut with helpful section comments
    pub fn create_default_document() -> DocumentMut {
        let template = r##"# driftwm configuration
# Managed by driftwm-settings

mod_key = "super"
focus_follows_mouse = false
window_placement = "center"
focus_placement = "center"

[session]
suspend_on_close = false
restore_windows = false
restore_camera = false
restore_bookmarks = false

[snap]
enabled = true
gap = 12.0
outer_gap = 0.0
distance = 24.0
break_force = 32.0
corners = false
centers = false

[decorations]
default_mode = "client"
bg_color = "#303030"
fg_color = "#FFFFFF"
corner_radius = 10
border_width = 0
border_color = "#303030"
border_color_focused = "#303030"
opacity = 1.0
opacity_focused = 1.0
blur = false
shadow = true
title_bar_height = 25
font = "Adwaita Sans"
font_size = 11
font_weight = "medium"
title_align = "center"

[effects]
blur_radius = 2
blur_strength = 1.1
animate_blur_fps = 20
animation_speed = 0.5
animation_scale = 0.95

[background]
type = "default"

[navigation]
camera_speed = 0.3
auto_navigate_on_close = true
auto_navigate_on_click = false
nudge_step = 20
resize_step = 20
pan_step = 100.0
trackpad_speed = 1.5
mouse_speed = 1.0
drift = 0.5

[navigation.bookmarks]
"1" = [-1750.0, 1750.0]
"2" = [1750.0, 1750.0]
"3" = [1750.0, -1750.0]
"4" = [-1750.0, -1750.0]

[navigation.edge_pan]
zone = 100.0
speed_min = 4.0
speed_max = 10.0
cursor_pan = false
cursor_zone = 20.0
latency_ms = 120

[zoom]
step = 1.1
fit_padding = 80.0
reset_on_new_window = true
reset_on_activation = true
interact_min = 0.0

[input.keyboard]
layout = "us"
repeat_rate = 25
repeat_delay = 200
layout_independent = true
num_lock = true
caps_lock = false

[input.trackpad]
tap_to_click = true
natural_scroll = true
tap_and_drag = true
accel_speed = 0.0
accel_profile = "adaptive"
click_method = "none"
disable_while_typing = true
enable = true
disable_on_external_mouse = false

[input.mouse]
accel_speed = 0.0
accel_profile = "flat"
natural_scroll = false
left_handed = false
"##;
        template.parse().unwrap_or_else(|_| DocumentMut::new())
    }

    /// Sync the current struct fields into DocumentMut, preserving non-managed sections & comments
    pub fn apply_to_document(&self, doc: &mut DocumentMut) {
        // Root fields
        set_opt_str(doc.as_table_mut(), "mod_key", &self.mod_key);
        set_opt_bool(doc.as_table_mut(), "focus_follows_mouse", self.focus_follows_mouse);
        set_opt_str(doc.as_table_mut(), "window_placement", &self.window_placement);
        set_opt_str(doc.as_table_mut(), "focus_placement", &self.focus_placement);

        // Session
        let session = ensure_table(doc, "session");
        set_opt_bool(session, "suspend_on_close", self.session.suspend_on_close);
        set_opt_bool(session, "restore_windows", self.session.restore_windows);
        set_opt_bool(session, "restore_camera", self.session.restore_camera);
        set_opt_bool(session, "restore_bookmarks", self.session.restore_bookmarks);

        // Snap
        let snap = ensure_table(doc, "snap");
        set_opt_bool(snap, "enabled", self.snap.enabled);
        set_opt_f64(snap, "gap", self.snap.gap);
        set_opt_f64(snap, "outer_gap", self.snap.outer_gap);
        set_opt_f64(snap, "distance", self.snap.distance);
        set_opt_f64(snap, "break_force", self.snap.break_force);
        set_opt_bool(snap, "corners", self.snap.corners);
        set_opt_bool(snap, "centers", self.snap.centers);

        // Decorations
        let deco = ensure_table(doc, "decorations");
        set_opt_str(deco, "default_mode", &self.decorations.default_mode);
        set_opt_str(deco, "bg_color", &self.decorations.bg_color);
        set_opt_str(deco, "fg_color", &self.decorations.fg_color);
        set_opt_i64(deco, "corner_radius", self.decorations.corner_radius.map(|v| v as i64));
        set_opt_i64(deco, "border_width", self.decorations.border_width.map(|v| v as i64));
        set_opt_str(deco, "border_color", &self.decorations.border_color);
        set_opt_str(deco, "border_color_focused", &self.decorations.border_color_focused);
        set_opt_f64(deco, "opacity", self.decorations.opacity);
        set_opt_f64(deco, "opacity_focused", self.decorations.opacity_focused);
        set_opt_bool(deco, "blur", self.decorations.blur);
        set_opt_bool(deco, "shadow", self.decorations.shadow);
        set_opt_i64(deco, "title_bar_height", self.decorations.title_bar_height.map(|v| v as i64));
        set_opt_str(deco, "font", &self.decorations.font);
        set_opt_i64(deco, "font_size", self.decorations.font_size.map(|v| v as i64));
        set_opt_str(deco, "font_weight", &self.decorations.font_weight);
        set_opt_str(deco, "title_align", &self.decorations.title_align);

        // Effects
        let eff = ensure_table(doc, "effects");
        set_opt_i64(eff, "blur_radius", self.effects.blur_radius.map(|v| v as i64));
        set_opt_f64(eff, "blur_strength", self.effects.blur_strength);
        set_opt_i64(eff, "animate_blur_fps", self.effects.animate_blur_fps.map(|v| v as i64));
        set_opt_f64(eff, "animation_speed", self.effects.animation_speed);
        set_opt_f64(eff, "animation_scale", self.effects.animation_scale);

        // Background
        let bg = ensure_table(doc, "background");
        set_opt_str(bg, "type", &self.background.kind);
        set_opt_str(bg, "path", &self.background.path);
        set_opt_str(bg, "texture", &self.background.texture);
        set_opt_bool(bg, "mirror_tile", self.background.mirror_tile);
        set_opt_bool(bg, "cache_shader", self.background.cache_shader);
        set_opt_bool(bg, "transparent_shader", self.background.transparent_shader);
        set_opt_i64(bg, "cache_budget_mb", self.background.cache_budget_mb.map(|v| v as i64));
        set_opt_i64(bg, "animate_fps", self.background.animate_fps.map(|v| v as i64));

        // Navigation
        let nav = ensure_table(doc, "navigation");
        set_opt_f64(nav, "camera_speed", self.navigation.camera_speed);
        set_opt_bool(nav, "auto_navigate_on_close", self.navigation.auto_navigate_on_close);
        set_opt_bool(nav, "auto_navigate_on_click", self.navigation.auto_navigate_on_click);
        set_opt_i64(nav, "nudge_step", self.navigation.nudge_step.map(|v| v as i64));
        set_opt_i64(nav, "resize_step", self.navigation.resize_step.map(|v| v as i64));
        set_opt_f64(nav, "pan_step", self.navigation.pan_step);
        set_opt_f64(nav, "trackpad_speed", self.navigation.trackpad_speed);
        set_opt_f64(nav, "mouse_speed", self.navigation.mouse_speed);
        set_opt_f64(nav, "touch_speed", self.navigation.touch_speed);
        set_opt_f64(nav, "drift", self.navigation.drift);

        // Anchors
        if let Some(anchors) = &self.navigation.anchors {
            let mut arr = Array::new();
            for pt in anchors {
                let mut pt_arr = Array::new();
                pt_arr.push(pt[0]);
                pt_arr.push(pt[1]);
                arr.push(pt_arr);
            }
            nav["anchors"] = Item::Value(Value::Array(arr));
        }

        // Bookmarks
        if let Some(bookmarks) = &self.navigation.bookmarks {
            let bm_table = ensure_nested_table(doc, "navigation", "bookmarks");
            bm_table.clear();
            for (key, coord) in bookmarks {
                let mut pt_arr = Array::new();
                pt_arr.push(coord[0]);
                pt_arr.push(coord[1]);
                bm_table[key] = Item::Value(Value::Array(pt_arr));
            }
        }

        // Edge pan
        let edge = ensure_nested_table(doc, "navigation", "edge_pan");
        set_opt_f64(edge, "zone", self.navigation.edge_pan.zone);
        set_opt_f64(edge, "speed_min", self.navigation.edge_pan.speed_min);
        set_opt_f64(edge, "speed_max", self.navigation.edge_pan.speed_max);
        set_opt_bool(edge, "cursor_pan", self.navigation.edge_pan.cursor_pan);
        set_opt_f64(edge, "cursor_zone", self.navigation.edge_pan.cursor_zone);
        set_opt_i64(edge, "latency_ms", self.navigation.edge_pan.latency_ms.map(|v| v as i64));

        // Zoom
        let zoom = ensure_table(doc, "zoom");
        set_opt_f64(zoom, "step", self.zoom.step);
        set_opt_f64(zoom, "fit_padding", self.zoom.fit_padding);
        set_opt_bool(zoom, "reset_on_new_window", self.zoom.reset_on_new_window);
        set_opt_bool(zoom, "reset_on_activation", self.zoom.reset_on_activation);
        set_opt_f64(zoom, "interact_min", self.zoom.interact_min);
        set_opt_f64(zoom, "trackpad_speed", self.zoom.trackpad_speed);
        set_opt_f64(zoom, "mouse_speed", self.zoom.mouse_speed);
        set_opt_f64(zoom, "touch_speed", self.zoom.touch_speed);

        // Input Keyboard
        let kbd = ensure_nested_table(doc, "input", "keyboard");
        set_opt_str(kbd, "layout", &self.input.keyboard.layout);
        set_opt_str(kbd, "variant", &self.input.keyboard.variant);
        set_opt_str(kbd, "options", &self.input.keyboard.options);
        set_opt_str(kbd, "model", &self.input.keyboard.model);
        set_opt_i64(kbd, "repeat_rate", self.input.keyboard.repeat_rate.map(|v| v as i64));
        set_opt_i64(kbd, "repeat_delay", self.input.keyboard.repeat_delay.map(|v| v as i64));
        set_opt_bool(kbd, "layout_independent", self.input.keyboard.layout_independent);
        set_opt_bool(kbd, "num_lock", self.input.keyboard.num_lock);
        set_opt_bool(kbd, "caps_lock", self.input.keyboard.caps_lock);
        set_opt_bool(kbd, "remember_layout_per_window", self.input.keyboard.remember_layout_per_window);

        // Input Trackpad
        let pad = ensure_nested_table(doc, "input", "trackpad");
        set_opt_bool(pad, "tap_to_click", self.input.trackpad.tap_to_click);
        set_opt_bool(pad, "natural_scroll", self.input.trackpad.natural_scroll);
        set_opt_bool(pad, "tap_and_drag", self.input.trackpad.tap_and_drag);
        set_opt_f64(pad, "accel_speed", self.input.trackpad.accel_speed);
        set_opt_str(pad, "accel_profile", &self.input.trackpad.accel_profile);
        set_opt_str(pad, "click_method", &self.input.trackpad.click_method);
        set_opt_bool(pad, "disable_while_typing", self.input.trackpad.disable_while_typing);
        set_opt_bool(pad, "enable", self.input.trackpad.enable);
        set_opt_bool(pad, "disable_on_external_mouse", self.input.trackpad.disable_on_external_mouse);

        // Input Mouse
        let mouse = ensure_nested_table(doc, "input", "mouse");
        set_opt_f64(mouse, "accel_speed", self.input.mouse.accel_speed);
        set_opt_str(mouse, "accel_profile", &self.input.mouse.accel_profile);
        set_opt_bool(mouse, "natural_scroll", self.input.mouse.natural_scroll);
        set_opt_bool(mouse, "left_handed", self.input.mouse.left_handed);

        // Cursor
        let cur = ensure_table(doc, "cursor");
        set_opt_str(cur, "theme", &self.cursor.theme);
        set_opt_i64(cur, "size", self.cursor.size.map(|v| v as i64));
        set_opt_f64(cur, "inactive_opacity", self.cursor.inactive_opacity);
    }
}

// Helpers for toml_edit Table updates
fn ensure_table<'a>(doc: &'a mut DocumentMut, name: &str) -> &'a mut Table {
    if !doc.contains_table(name) {
        doc[name] = Item::Table(Table::new());
    }
    doc[name].as_table_mut().unwrap()
}

fn ensure_nested_table<'a>(doc: &'a mut DocumentMut, parent: &str, child: &str) -> &'a mut Table {
    let p = ensure_table(doc, parent);
    if !p.contains_table(child) {
        p[child] = Item::Table(Table::new());
    }
    p[child].as_table_mut().unwrap()
}

fn set_opt_str(table: &mut Table, key: &str, val: &Option<String>) {
    match val {
        Some(s) if !s.is_empty() => {
            table[key] = Item::Value(Value::from(s.clone()));
        }
        Some(_) | None => {
            table.remove(key);
        }
    }
}

fn set_opt_bool(table: &mut Table, key: &str, val: Option<bool>) {
    match val {
        Some(b) => {
            table[key] = Item::Value(Value::from(b));
        }
        None => {
            table.remove(key);
        }
    }
}

fn set_opt_i64(table: &mut Table, key: &str, val: Option<i64>) {
    match val {
        Some(n) => {
            table[key] = Item::Value(Value::from(n));
        }
        None => {
            table.remove(key);
        }
    }
}

fn set_opt_f64(table: &mut Table, key: &str, val: Option<f64>) {
    match val {
        Some(n) => {
            table[key] = Item::Value(Value::from(n));
        }
        None => {
            table.remove(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presets_have_driftwm_defaults() {
        let cfg = DriftwmConfig::default_with_presets();
        assert_eq!(cfg.mod_key.as_deref(), Some("super"));
        assert_eq!(cfg.window_placement.as_deref(), Some("center"));
        assert_eq!(cfg.focus_placement.as_deref(), Some("center"));
        assert_eq!(cfg.snap.gap, Some(12.0));
        assert_eq!(cfg.snap.outer_gap, Some(0.0));
        assert_eq!(cfg.decorations.default_mode.as_deref(), Some("client"));
        assert_eq!(cfg.decorations.corner_radius, Some(10));
        assert_eq!(cfg.navigation.camera_speed, Some(0.3));
        assert_eq!(cfg.navigation.drift, Some(0.5));
    }

    #[test]
    fn test_apply_to_document_preserves_comments_and_custom_tables() {
        let initial_toml = r#"# User header comment
mod_key = "alt" # inline comment

# My custom keybindings table that settings shouldn't erase
[keybindings]
"mod+t" = "exec-terminal"
"mod+q" = "close-window"
"#;

        let mut doc: DocumentMut = initial_toml.parse().expect("valid toml");
        let mut cfg = DriftwmConfig::default_with_presets();
        cfg.mod_key = Some("super".to_string());
        cfg.window_placement = Some("auto".to_string());
        cfg.snap.gap = Some(16.0);

        cfg.apply_to_document(&mut doc);
        let output = doc.to_string();

        // 1. Comments preserved
        assert!(output.contains("# User header comment"));
        assert!(output.contains("# My custom keybindings table"));

        // 2. Custom unmanaged table preserved
        assert!(output.contains("[keybindings]"));
        assert!(output.contains("\"mod+t\" = \"exec-terminal\""));

        // 3. Updated values reflected
        assert!(output.contains("mod_key = \"super\""));
        assert!(output.contains("window_placement = \"auto\""));
        assert!(output.contains("gap = 16.0"));
    }

    #[test]
    fn test_bookmarks_sync() {
        let mut doc = DocumentMut::new();
        let mut cfg = DriftwmConfig::default_with_presets();
        cfg.navigation.bookmarks = Some(HashMap::from([
            ("code".to_string(), [100.0, 200.0]),
            ("chat".to_string(), [-500.0, 500.0]),
        ]));

        cfg.apply_to_document(&mut doc);
        let output = doc.to_string();

        assert!(output.contains("[navigation.bookmarks]"));
        assert!(output.contains("code = [100.0, 200.0]") || output.contains("\"code\" = [100.0, 200.0]"));
    }
}
