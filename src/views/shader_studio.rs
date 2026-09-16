use crate::config::DriftwmConfig;
use crate::i18n::Language;
use crate::icons;
use crate::theme::{
    card_style, mocha, pick_list_style, primary_button_style, secondary_button_style, slider_style,
    success_button_style, text_input_style,
};
use iced::widget::svg::Handle;
use iced::widget::{
    button, checkbox, column, container, pick_list, row, slider, text, text_editor, text_input,
};
use iced::{Alignment, Element, Length};
use std::path::{Path, PathBuf};
use toml_edit::DocumentMut;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderArchetype {
    PlasmaWaves,
    CyberGrid,
    Starfield,
    DigitalRain,
    AuroraBorealis,
    CatppuccinFlow,
    DotPulseGrid,
    AcidLava,
    BlankCustom,
}

impl ShaderArchetype {
    pub const ALL: [ShaderArchetype; 9] = [
        ShaderArchetype::PlasmaWaves,
        ShaderArchetype::CyberGrid,
        ShaderArchetype::Starfield,
        ShaderArchetype::DigitalRain,
        ShaderArchetype::AuroraBorealis,
        ShaderArchetype::CatppuccinFlow,
        ShaderArchetype::DotPulseGrid,
        ShaderArchetype::AcidLava,
        ShaderArchetype::BlankCustom,
    ];

    pub fn to_string(&self) -> String {
        match self {
            ShaderArchetype::PlasmaWaves => "🌊 Plasma Waves".to_string(),
            ShaderArchetype::CyberGrid => "🌆 Cyberpunk Horizon Grid".to_string(),
            ShaderArchetype::Starfield => "✨ Cosmic Starfield & Nebula".to_string(),
            ShaderArchetype::DigitalRain => "🟢 Matrix Digital Rain".to_string(),
            ShaderArchetype::AuroraBorealis => "🌌 Aurora Borealis".to_string(),
            ShaderArchetype::CatppuccinFlow => "🎨 Catppuccin Aesthetic Mesh".to_string(),
            ShaderArchetype::DotPulseGrid => "⚪ Pulsing Dot Grid".to_string(),
            ShaderArchetype::AcidLava => "🔥 Acid Molten Lava".to_string(),
            ShaderArchetype::BlankCustom => "📝 Custom Scratchpad".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        for arch in Self::ALL {
            if arch.to_string() == s {
                return arch;
            }
        }
        ShaderArchetype::PlasmaWaves
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PalettePreset {
    CatppuccinMocha,
    CyberpunkNeon,
    NordicFrost,
    SunsetHorizon,
    EmeraldMatrix,
}

#[derive(Debug)]
pub struct ShaderStudioState {
    pub archetype: ShaderArchetype,
    pub c1_r: u8,
    pub c1_g: u8,
    pub c1_b: u8,
    pub c2_r: u8,
    pub c2_g: u8,
    pub c2_b: u8,
    pub c3_r: u8,
    pub c3_g: u8,
    pub c3_b: u8,
    pub bg_r: u8,
    pub bg_g: u8,
    pub bg_b: u8,
    pub speed: f32,
    pub scale: f32,
    pub parallax: f32,
    pub glow: f32,
    pub zoom_reactive: bool,
    pub transparent: bool,
    pub filename: String,
    pub code_content: text_editor::Content,
    pub manual_mode: bool,
    pub validation_msg: Option<(bool, String)>,
    pub status_msg: Option<(bool, String)>,
}

impl Default for ShaderStudioState {
    fn default() -> Self {
        let mut state = Self {
            archetype: ShaderArchetype::CyberGrid,
            // Catppuccin Mocha / Cyberpunk defaults
            c1_r: 203,
            c1_g: 166,
            c1_b: 247, // Mauve #cba6f7
            c2_r: 137,
            c2_g: 180,
            c2_b: 250, // Blue #89b4fa
            c3_r: 243,
            c3_g: 139,
            c3_b: 168, // Red/Pink #f38ba8
            bg_r: 30,
            bg_g: 30,
            bg_b: 46, // Base #1e1e2e
            speed: 1.0,
            scale: 1.0,
            parallax: 0.5,
            glow: 1.2,
            zoom_reactive: true,
            transparent: false,
            filename: "my_custom_shader.glsl".to_string(),
            code_content: text_editor::Content::new(),
            manual_mode: false,
            validation_msg: None,
            status_msg: None,
        };
        let code = generate_glsl(&state);
        state.code_content = text_editor::Content::with_text(&code);
        state
    }
}

impl ShaderStudioState {
    pub fn apply_palette(&mut self, palette: PalettePreset) {
        match palette {
            PalettePreset::CatppuccinMocha => {
                self.c1_r = 203;
                self.c1_g = 166;
                self.c1_b = 247; // Mauve
                self.c2_r = 137;
                self.c2_g = 180;
                self.c2_b = 250; // Blue
                self.c3_r = 250;
                self.c3_g = 179;
                self.c3_b = 135; // Peach
                self.bg_r = 30;
                self.bg_g = 30;
                self.bg_b = 46; // Base
            }
            PalettePreset::CyberpunkNeon => {
                self.c1_r = 0;
                self.c1_g = 255;
                self.c1_b = 255; // Cyan
                self.c2_r = 255;
                self.c2_g = 0;
                self.c2_b = 128; // Hot Pink
                self.c3_r = 255;
                self.c3_g = 230;
                self.c3_b = 0; // Neon Yellow
                self.bg_r = 8;
                self.bg_g = 8;
                self.bg_b = 20; // Deep Dark
            }
            PalettePreset::NordicFrost => {
                self.c1_r = 136;
                self.c1_g = 192;
                self.c1_b = 208; // Frost Cyan
                self.c2_r = 129;
                self.c2_g = 161;
                self.c2_b = 193; // Glacier Blue
                self.c3_r = 163;
                self.c3_g = 190;
                self.c3_b = 140; // Aurora Green
                self.bg_r = 46;
                self.bg_g = 52;
                self.bg_b = 64; // Polar Night
            }
            PalettePreset::SunsetHorizon => {
                self.c1_r = 255;
                self.c1_g = 123;
                self.c1_b = 0; // Solar Orange
                self.c2_r = 247;
                self.c2_g = 37;
                self.c2_b = 133; // Vibrant Rose
                self.c3_r = 114;
                self.c3_g = 9;
                self.c3_b = 183; // Violet Dusk
                self.bg_r = 16;
                self.bg_g = 0;
                self.bg_b = 43; // Midnight Abyss
            }
            PalettePreset::EmeraldMatrix => {
                self.c1_r = 0;
                self.c1_g = 255;
                self.c1_b = 102; // Terminal Neon
                self.c2_r = 0;
                self.c2_g = 200;
                self.c2_b = 80; // Matrix Green
                self.c3_r = 180;
                self.c3_g = 255;
                self.c3_b = 200; // Phosphor Glow
                self.bg_r = 2;
                self.bg_g = 14;
                self.bg_b = 6; // Dark Room
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum ShaderStudioMessage {
    ArchetypeSelected(String),
    PaletteSelected(PalettePreset),
    Color1R(f32),
    Color1G(f32),
    Color1B(f32),
    Color2R(f32),
    Color2G(f32),
    Color2B(f32),
    Color3R(f32),
    Color3G(f32),
    Color3B(f32),
    ColorBgR(f32),
    ColorBgG(f32),
    ColorBgB(f32),
    SpeedChanged(f32),
    ScaleChanged(f32),
    ParallaxChanged(f32),
    GlowChanged(f32),
    ZoomReactiveToggled(bool),
    TransparentToggled(bool),
    FilenameChanged(String),
    CodeAction(text_editor::Action),
    ManualModeToggled(bool),
    ResetCodeToGenerated,
    ValidateCode,
    SaveShader,
    SaveAndApply,
}

pub fn update(
    state: &mut ShaderStudioState,
    config: &mut DriftwmConfig,
    doc: &mut DocumentMut,
    config_path: &Path,
    msg: ShaderStudioMessage,
) {
    match msg {
        ShaderStudioMessage::ArchetypeSelected(s) => {
            state.archetype = ShaderArchetype::from_string(&s);
            if !state.manual_mode {
                let code = generate_glsl(state);
                state.code_content = text_editor::Content::with_text(&code);
            }
        }
        ShaderStudioMessage::PaletteSelected(pal) => {
            state.apply_palette(pal);
            if !state.manual_mode {
                let code = generate_glsl(state);
                state.code_content = text_editor::Content::with_text(&code);
            }
        }
        ShaderStudioMessage::Color1R(v) => {
            state.c1_r = v.round() as u8;
            sync_code(state);
        }
        ShaderStudioMessage::Color1G(v) => {
            state.c1_g = v.round() as u8;
            sync_code(state);
        }
        ShaderStudioMessage::Color1B(v) => {
            state.c1_b = v.round() as u8;
            sync_code(state);
        }
        ShaderStudioMessage::Color2R(v) => {
            state.c2_r = v.round() as u8;
            sync_code(state);
        }
        ShaderStudioMessage::Color2G(v) => {
            state.c2_g = v.round() as u8;
            sync_code(state);
        }
        ShaderStudioMessage::Color2B(v) => {
            state.c2_b = v.round() as u8;
            sync_code(state);
        }
        ShaderStudioMessage::Color3R(v) => {
            state.c3_r = v.round() as u8;
            sync_code(state);
        }
        ShaderStudioMessage::Color3G(v) => {
            state.c3_g = v.round() as u8;
            sync_code(state);
        }
        ShaderStudioMessage::Color3B(v) => {
            state.c3_b = v.round() as u8;
            sync_code(state);
        }
        ShaderStudioMessage::ColorBgR(v) => {
            state.bg_r = v.round() as u8;
            sync_code(state);
        }
        ShaderStudioMessage::ColorBgG(v) => {
            state.bg_g = v.round() as u8;
            sync_code(state);
        }
        ShaderStudioMessage::ColorBgB(v) => {
            state.bg_b = v.round() as u8;
            sync_code(state);
        }
        ShaderStudioMessage::SpeedChanged(v) => {
            state.speed = v;
            sync_code(state);
        }
        ShaderStudioMessage::ScaleChanged(v) => {
            state.scale = v;
            sync_code(state);
        }
        ShaderStudioMessage::ParallaxChanged(v) => {
            state.parallax = v;
            sync_code(state);
        }
        ShaderStudioMessage::GlowChanged(v) => {
            state.glow = v;
            sync_code(state);
        }
        ShaderStudioMessage::ZoomReactiveToggled(v) => {
            state.zoom_reactive = v;
            sync_code(state);
        }
        ShaderStudioMessage::TransparentToggled(v) => {
            state.transparent = v;
            sync_code(state);
        }
        ShaderStudioMessage::FilenameChanged(name) => {
            state.filename = name;
        }
        ShaderStudioMessage::CodeAction(action) => {
            state.code_content.perform(action);
            state.manual_mode = true;
        }
        ShaderStudioMessage::ManualModeToggled(v) => {
            state.manual_mode = v;
            if !v {
                let code = generate_glsl(state);
                state.code_content = text_editor::Content::with_text(&code);
            }
        }
        ShaderStudioMessage::ResetCodeToGenerated => {
            state.manual_mode = false;
            let code = generate_glsl(state);
            state.code_content = text_editor::Content::with_text(&code);
            state.validation_msg = None;
        }
        ShaderStudioMessage::ValidateCode => {
            let code = state.code_content.text();
            let mut errors = Vec::new();
            if !code.contains("gl_FragColor") {
                errors.push("Missing `gl_FragColor = ...;` output");
            }
            if !code.contains("void main") {
                errors.push("Missing `void main() { ... }` entrypoint");
            }
            if !code.contains("precision") {
                errors.push("Missing `precision highp float;` specifier");
            }
            if errors.is_empty() {
                state.validation_msg = Some((
                    true,
                    "✓ Valid GLSL ES 1.0 shader structure for smithay/driftwm.".to_string(),
                ));
            } else {
                state.validation_msg = Some((false, format!("⚠ Validation: {}", errors.join(", "))));
            }
        }
        ShaderStudioMessage::SaveShader => {
            let code = state.code_content.text();
            match save_shader_to_disk(&state.filename, &code) {
                Ok(path) => {
                    state.status_msg = Some((
                        true,
                        format!("✓ Shader successfully saved to {}", path.display()),
                    ));
                }
                Err(e) => {
                    state.status_msg = Some((false, format!("Error saving shader: {}", e)));
                }
            }
        }
        ShaderStudioMessage::SaveAndApply => {
            let code = state.code_content.text();
            match save_shader_to_disk(&state.filename, &code) {
                Ok(path) => {
                    let path_str = path.to_string_lossy().to_string();
                    config.background.kind = Some("shader".to_string());
                    config.background.path = Some(path_str);
                    config.background.transparent_shader = Some(state.transparent);

                    if let Err(e) = config.save(doc, config_path) {
                        state.status_msg = Some((false, format!("Saved shader but failed to update config: {}", e)));
                    } else {
                        // Touch config file to trigger driftwm inotify reload
                        let _ = std::process::Command::new("touch")
                            .arg(config_path)
                            .status();
                        state.status_msg = Some((
                            true,
                            "🚀 Applied! driftwm reloaded with your custom shader background.".to_string(),
                        ));
                    }
                }
                Err(e) => {
                    state.status_msg = Some((false, format!("Failed to save shader file: {}", e)));
                }
            }
        }
    }
}

fn sync_code(state: &mut ShaderStudioState) {
    if !state.manual_mode {
        let code = generate_glsl(state);
        state.code_content = text_editor::Content::with_text(&code);
    }
}

pub fn save_shader_to_disk(filename: &str, code: &str) -> std::io::Result<PathBuf> {
    let clean_name = if filename.trim().is_empty() {
        "custom_wallpaper.glsl"
    } else {
        filename.trim()
    };
    let clean_name = if clean_name.ends_with(".glsl") {
        clean_name.to_string()
    } else {
        format!("{}.glsl", clean_name)
    };

    let dir = dirs::config_dir()
        .map(|d| d.join("driftwm").join("shaders"))
        .unwrap_or_else(|| PathBuf::from("/root/.config/driftwm/shaders"));

    std::fs::create_dir_all(&dir)?;
    let target = dir.join(clean_name);
    std::fs::write(&target, code)?;
    Ok(target)
}

pub fn generate_glsl(state: &ShaderStudioState) -> String {
    let r1 = state.c1_r as f32 / 255.0;
    let g1 = state.c1_g as f32 / 255.0;
    let b1 = state.c1_b as f32 / 255.0;

    let r2 = state.c2_r as f32 / 255.0;
    let g2 = state.c2_g as f32 / 255.0;
    let b2 = state.c2_b as f32 / 255.0;

    let r3 = state.c3_r as f32 / 255.0;
    let g3 = state.c3_g as f32 / 255.0;
    let b3 = state.c3_b as f32 / 255.0;

    let rbg = state.bg_r as f32 / 255.0;
    let gbg = state.bg_g as f32 / 255.0;
    let bbg = state.bg_b as f32 / 255.0;

    let alpha_expr = if state.transparent {
        "0.7"
    } else {
        "1.0"
    };

    let zoom_factor = if state.zoom_reactive {
        " / u_zoom"
    } else {
        ""
    };

    match state.archetype {
        ShaderArchetype::CyberGrid => format!(
            r#"// driftwm Shader Studio — Cyberpunk Horizon Grid
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform vec2 u_camera;
uniform float u_time;
uniform float u_zoom;

const vec3 COLOR_PRIMARY   = vec3({r1:.3}, {g1:.3}, {b1:.3});
const vec3 COLOR_SECONDARY = vec3({r2:.3}, {g2:.3}, {b2:.3});
const vec3 COLOR_ACCENT    = vec3({r3:.3}, {g3:.3}, {b3:.3});
const vec3 COLOR_BG        = vec3({rbg:.3}, {gbg:.3}, {bbg:.3});
const float SPEED          = {speed:.2};
const float SCALE          = {scale:.2};
const float PARALLAX       = {parallax:.2};
const float GLOW           = {glow:.2};

void main() {{
    vec2 p = v_coords * 2.0 - 1.0;
    p.x *= size.x / size.y;
    vec2 cam = u_camera * PARALLAX * 0.001{zoom_factor};
    p += cam;

    float horizon = -0.15;
    vec3 col = COLOR_BG;

    if (p.y < horizon) {{
        float depth = 1.0 / (horizon - p.y);
        vec2 grid_uv = vec2(p.x * depth * SCALE, depth * SCALE + u_time * SPEED * 1.5);
        vec2 grid = abs(fract(grid_uv - 0.5) - 0.5) / fwidth(grid_uv);
        float line = min(grid.x, grid.y);
        float grid_val = 1.0 - min(line, 1.0);
        float fog = exp(-depth * 0.12);
        vec3 grid_col = mix(COLOR_PRIMARY, COLOR_SECONDARY, sin(grid_uv.x * 2.0) * 0.5 + 0.5);
        col = mix(COLOR_BG, grid_col * GLOW, grid_val * fog);
    }} else {{
        float dist_horizon = p.y - horizon;
        float glow_h = exp(-dist_horizon * 5.0) * GLOW;
        col += COLOR_ACCENT * glow_h * 0.6;
    }}

    gl_FragColor = vec4(col, {alpha_expr});
}}
"#,
            r1 = r1, g1 = g1, b1 = b1,
            r2 = r2, g2 = g2, b2 = b2,
            r3 = r3, g3 = g3, b3 = b3,
            rbg = rbg, gbg = gbg, bbg = bbg,
            speed = state.speed,
            scale = state.scale,
            parallax = state.parallax,
            glow = state.glow,
            zoom_factor = zoom_factor,
            alpha_expr = alpha_expr
        ),

        ShaderArchetype::PlasmaWaves => format!(
            r#"// driftwm Shader Studio — Fluid Plasma Waves
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform vec2 u_camera;
uniform float u_time;
uniform float u_zoom;

const vec3 COLOR_PRIMARY   = vec3({r1:.3}, {g1:.3}, {b1:.3});
const vec3 COLOR_SECONDARY = vec3({r2:.3}, {g2:.3}, {b2:.3});
const vec3 COLOR_ACCENT    = vec3({r3:.3}, {g3:.3}, {b3:.3});
const vec3 COLOR_BG        = vec3({rbg:.3}, {gbg:.3}, {bbg:.3});
const float SPEED          = {speed:.2};
const float SCALE          = {scale:.2};
const float PARALLAX       = {parallax:.2};
const float GLOW           = {glow:.2};

void main() {{
    vec2 uv = (v_coords * size + u_camera * PARALLAX{zoom_factor}) / (400.0 / SCALE);
    float t = u_time * SPEED;

    float v1 = sin(uv.x + t);
    float v2 = sin(uv.y * 1.2 - t * 0.8);
    float v3 = sin(uv.x + uv.y + t * 1.4);
    float cx = uv.x + 0.5 * sin(t * 0.33);
    float cy = uv.y + 0.5 * cos(t * 0.5);
    float v4 = sin(sqrt(cx * cx + cy * cy + 1.0) * 2.0 - t * 1.6);
    float plasma = (v1 + v2 + v3 + v4) * 0.25;

    vec3 col = COLOR_BG;
    col = mix(col, COLOR_PRIMARY, smoothstep(-0.6, 0.2, plasma));
    col = mix(col, COLOR_SECONDARY, smoothstep(0.0, 0.7, plasma));
    col += COLOR_ACCENT * pow(max(0.0, plasma), 3.0) * GLOW;

    gl_FragColor = vec4(col, {alpha_expr});
}}
"#,
            r1 = r1, g1 = g1, b1 = b1,
            r2 = r2, g2 = g2, b2 = b2,
            r3 = r3, g3 = g3, b3 = b3,
            rbg = rbg, gbg = gbg, bbg = bbg,
            speed = state.speed,
            scale = state.scale,
            parallax = state.parallax,
            glow = state.glow,
            zoom_factor = zoom_factor,
            alpha_expr = alpha_expr
        ),

        ShaderArchetype::Starfield => format!(
            r#"// driftwm Shader Studio — Cosmic Starfield & Nebula
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform vec2 u_camera;
uniform float u_time;
uniform float u_zoom;

const vec3 COLOR_PRIMARY   = vec3({r1:.3}, {g1:.3}, {b1:.3});
const vec3 COLOR_SECONDARY = vec3({r2:.3}, {g2:.3}, {b2:.3});
const vec3 COLOR_ACCENT    = vec3({r3:.3}, {g3:.3}, {b3:.3});
const vec3 COLOR_BG        = vec3({rbg:.3}, {gbg:.3}, {bbg:.3});
const float SPEED          = {speed:.2};
const float SCALE          = {scale:.2};
const float PARALLAX       = {parallax:.2};
const float GLOW           = {glow:.2};

float hash21(vec2 p) {{
    p = fract(p * vec2(123.34, 456.21));
    p += dot(p, p + 45.32);
    return fract(p.x * p.y);
}}

void main() {{
    vec2 uv = (v_coords * size + u_camera * PARALLAX{zoom_factor}) / (250.0 / SCALE);
    float t = u_time * SPEED * 0.5;

    vec3 col = COLOR_BG;

    // Nebula dust
    float n1 = sin(uv.x * 0.4 + t * 0.3) * cos(uv.y * 0.5 - t * 0.2);
    float n2 = sin((uv.x + uv.y) * 0.3 - t * 0.4);
    col += COLOR_SECONDARY * max(0.0, n1) * 0.3 * GLOW;
    col += COLOR_ACCENT * max(0.0, n2) * 0.25 * GLOW;

    // Star layers with parallax
    for (int i = 1; i <= 3; i++) {{
        float layer = float(i);
        vec2 st = uv * (1.0 + layer * 0.8) + vec2(t * 0.1 * layer, 0.0);
        vec2 id = floor(st);
        vec2 gv = fract(st) - 0.5;
        float h = hash21(id);
        float star_size = (h * 0.05 + 0.02) * (1.0 / layer);
        float d = length(gv);
        float twinkle = sin(t * 3.0 + h * 6.28) * 0.5 + 0.5;
        if (h > 0.65) {{
            float star = smoothstep(star_size, 0.0, d) * (0.5 + 0.5 * twinkle);
            col += mix(COLOR_PRIMARY, vec3(1.0), 0.6) * star * GLOW;
        }}
    }}

    gl_FragColor = vec4(col, {alpha_expr});
}}
"#,
            r1 = r1, g1 = g1, b1 = b1,
            r2 = r2, g2 = g2, b2 = b2,
            r3 = r3, g3 = g3, b3 = b3,
            rbg = rbg, gbg = gbg, bbg = bbg,
            speed = state.speed,
            scale = state.scale,
            parallax = state.parallax,
            glow = state.glow,
            zoom_factor = zoom_factor,
            alpha_expr = alpha_expr
        ),

        ShaderArchetype::DigitalRain => format!(
            r#"// driftwm Shader Studio — Matrix Digital Rain
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform vec2 u_camera;
uniform float u_time;
uniform float u_zoom;

const vec3 COLOR_PRIMARY   = vec3({r1:.3}, {g1:.3}, {b1:.3});
const vec3 COLOR_SECONDARY = vec3({r2:.3}, {g2:.3}, {b2:.3});
const vec3 COLOR_ACCENT    = vec3({r3:.3}, {g3:.3}, {b3:.3});
const vec3 COLOR_BG        = vec3({rbg:.3}, {gbg:.3}, {bbg:.3});
const float SPEED          = {speed:.2};
const float SCALE          = {scale:.2};
const float PARALLAX       = {parallax:.2};
const float GLOW           = {glow:.2};

float hash(vec2 p) {{
    return fract(sin(dot(p, vec2(12.9898, 78.233))) * 43758.5453);
}}

void main() {{
    vec2 uv = (v_coords * size + u_camera * PARALLAX{zoom_factor}) / (32.0 / SCALE);
    float col_id = floor(uv.x);
    float row_id = floor(uv.y);
    float h = hash(vec2(col_id, 0.0));
    float speed = (0.8 + h * 0.8) * SPEED;
    float drop_y = fract(uv.y * 0.05 + u_time * speed * 0.2 + h);
    float glyph = step(0.3, hash(vec2(col_id, row_id)));
    float tail = pow(1.0 - drop_y, 4.0);

    vec3 col = COLOR_BG;
    col += COLOR_PRIMARY * tail * glyph * GLOW;
    if (drop_y < 0.05) {{
        col += COLOR_ACCENT * (1.0 - drop_y / 0.05) * GLOW;
    }}

    gl_FragColor = vec4(col, {alpha_expr});
}}
"#,
            r1 = r1, g1 = g1, b1 = b1,
            r2 = r2, g2 = g2, b2 = b2,
            r3 = r3, g3 = g3, b3 = b3,
            rbg = rbg, gbg = gbg, bbg = bbg,
            speed = state.speed,
            scale = state.scale,
            parallax = state.parallax,
            glow = state.glow,
            zoom_factor = zoom_factor,
            alpha_expr = alpha_expr
        ),

        ShaderArchetype::AuroraBorealis => format!(
            r#"// driftwm Shader Studio — Aurora Borealis
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform vec2 u_camera;
uniform float u_time;
uniform float u_zoom;

const vec3 COLOR_PRIMARY   = vec3({r1:.3}, {g1:.3}, {b1:.3});
const vec3 COLOR_SECONDARY = vec3({r2:.3}, {g2:.3}, {b2:.3});
const vec3 COLOR_ACCENT    = vec3({r3:.3}, {g3:.3}, {b3:.3});
const vec3 COLOR_BG        = vec3({rbg:.3}, {gbg:.3}, {bbg:.3});
const float SPEED          = {speed:.2};
const float SCALE          = {scale:.2};
const float PARALLAX       = {parallax:.2};
const float GLOW           = {glow:.2};

void main() {{
    vec2 p = (v_coords * size + u_camera * PARALLAX{zoom_factor}) / size.y;
    float t = u_time * SPEED * 0.4;

    vec3 col = COLOR_BG;
    for (float i = 1.0; i <= 3.0; i += 1.0) {{
        float wave = sin(p.x * 3.0 * SCALE + t * i + i * 1.5) * 0.15;
        float dist = abs(p.y - (0.45 + wave + i * 0.08));
        float intensity = exp(-dist * 18.0) * GLOW;
        vec3 band_col = mix(COLOR_PRIMARY, COLOR_SECONDARY, sin(p.x * 2.0 + t) * 0.5 + 0.5);
        col += band_col * intensity;
    }}

    gl_FragColor = vec4(col, {alpha_expr});
}}
"#,
            r1 = r1, g1 = g1, b1 = b1,
            r2 = r2, g2 = g2, b2 = b2,
            r3 = r3, g3 = g3, b3 = b3,
            rbg = rbg, gbg = gbg, bbg = bbg,
            speed = state.speed,
            scale = state.scale,
            parallax = state.parallax,
            glow = state.glow,
            zoom_factor = zoom_factor,
            alpha_expr = alpha_expr
        ),

        ShaderArchetype::CatppuccinFlow => format!(
            r#"// driftwm Shader Studio — Catppuccin Aesthetic Mesh
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform vec2 u_camera;
uniform float u_time;
uniform float u_zoom;

const vec3 COLOR_PRIMARY   = vec3({r1:.3}, {g1:.3}, {b1:.3});
const vec3 COLOR_SECONDARY = vec3({r2:.3}, {g2:.3}, {b2:.3});
const vec3 COLOR_ACCENT    = vec3({r3:.3}, {g3:.3}, {b3:.3});
const vec3 COLOR_BG        = vec3({rbg:.3}, {gbg:.3}, {bbg:.3});
const float SPEED          = {speed:.2};
const float SCALE          = {scale:.2};
const float PARALLAX       = {parallax:.2};
const float GLOW           = {glow:.2};

void main() {{
    vec2 uv = (v_coords * size + u_camera * PARALLAX{zoom_factor}) / (500.0 / SCALE);
    float t = u_time * SPEED * 0.5;

    vec2 p1 = vec2(sin(t * 0.7), cos(t * 0.6)) * 0.8;
    vec2 p2 = vec2(cos(t * 0.5), sin(t * 0.8)) * 0.9;
    vec2 p3 = vec2(sin(t * 0.9 + 2.0), cos(t * 0.4 + 1.0)) * 0.7;

    float d1 = length(uv - p1);
    float d2 = length(uv - p2);
    float d3 = length(uv - p3);

    float w1 = 1.0 / (d1 * d1 + 0.6);
    float w2 = 1.0 / (d2 * d2 + 0.6);
    float w3 = 1.0 / (d3 * d3 + 0.6);
    float sum = w1 + w2 + w3 + 0.4;

    vec3 col = (COLOR_PRIMARY * w1 + COLOR_SECONDARY * w2 + COLOR_ACCENT * w3 + COLOR_BG * 0.4) / sum;
    col *= GLOW;

    gl_FragColor = vec4(col, {alpha_expr});
}}
"#,
            r1 = r1, g1 = g1, b1 = b1,
            r2 = r2, g2 = g2, b2 = b2,
            r3 = r3, g3 = g3, b3 = b3,
            rbg = rbg, gbg = gbg, bbg = bbg,
            speed = state.speed,
            scale = state.scale,
            parallax = state.parallax,
            glow = state.glow,
            zoom_factor = zoom_factor,
            alpha_expr = alpha_expr
        ),

        ShaderArchetype::DotPulseGrid => format!(
            r#"// driftwm Shader Studio — Pulsing Dot Grid
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform vec2 u_camera;
uniform float u_time;
uniform float u_zoom;

const vec3 COLOR_PRIMARY   = vec3({r1:.3}, {g1:.3}, {b1:.3});
const vec3 COLOR_SECONDARY = vec3({r2:.3}, {g2:.3}, {b2:.3});
const vec3 COLOR_ACCENT    = vec3({r3:.3}, {g3:.3}, {b3:.3});
const vec3 COLOR_BG        = vec3({rbg:.3}, {gbg:.3}, {bbg:.3});
const float SPEED          = {speed:.2};
const float SCALE          = {scale:.2};
const float PARALLAX       = {parallax:.2};
const float GLOW           = {glow:.2};

void main() {{
    float spacing = 64.0 / SCALE;
    vec2 canvas_pos = v_coords * size + mod(u_camera * PARALLAX{zoom_factor}, spacing);
    vec2 grid = mod(canvas_pos, spacing);
    vec2 dist = min(grid, spacing - grid);
    float d = length(dist);

    float t = u_time * SPEED * 2.0;
    vec2 center = size * 0.5;
    float dist_center = length(v_coords * size - center);
    float pulse = sin(dist_center * 0.02 - t) * 0.5 + 0.5;

    float dot_radius = 1.5 + pulse * 1.5;
    float dot_alpha = 1.0 - smoothstep(dot_radius - 0.5, dot_radius + 0.5, d);

    vec3 dot_col = mix(COLOR_PRIMARY, COLOR_ACCENT, pulse) * GLOW;
    vec3 col = mix(COLOR_BG, dot_col, dot_alpha);

    gl_FragColor = vec4(col, {alpha_expr});
}}
"#,
            r1 = r1, g1 = g1, b1 = b1,
            r2 = r2, g2 = g2, b2 = b2,
            r3 = r3, g3 = g3, b3 = b3,
            rbg = rbg, gbg = gbg, bbg = bbg,
            speed = state.speed,
            scale = state.scale,
            parallax = state.parallax,
            glow = state.glow,
            zoom_factor = zoom_factor,
            alpha_expr = alpha_expr
        ),

        ShaderArchetype::AcidLava => format!(
            r#"// driftwm Shader Studio — Acid Molten Lava
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform vec2 u_camera;
uniform float u_time;
uniform float u_zoom;

const vec3 COLOR_PRIMARY   = vec3({r1:.3}, {g1:.3}, {b1:.3});
const vec3 COLOR_SECONDARY = vec3({r2:.3}, {g2:.3}, {b2:.3});
const vec3 COLOR_ACCENT    = vec3({r3:.3}, {g3:.3}, {b3:.3});
const vec3 COLOR_BG        = vec3({rbg:.3}, {gbg:.3}, {bbg:.3});
const float SPEED          = {speed:.2};
const float SCALE          = {scale:.2};
const float PARALLAX       = {parallax:.2};
const float GLOW           = {glow:.2};

void main() {{
    vec2 uv = (v_coords * size + u_camera * PARALLAX{zoom_factor}) / (300.0 / SCALE);
    float t = u_time * SPEED * 0.4;

    float n = sin(uv.x * 2.0 + sin(uv.y * 3.0 + t)) + cos(uv.y * 2.0 + sin(uv.x * 3.0 - t));
    float vein = abs(sin(n * 2.5 + t));
    float heat = 1.0 - smoothstep(0.0, 0.35, vein);

    vec3 col = COLOR_BG;
    col = mix(col, COLOR_SECONDARY, smoothstep(0.1, 0.6, heat));
    col = mix(col, COLOR_PRIMARY, smoothstep(0.5, 0.9, heat));
    col += COLOR_ACCENT * pow(heat, 3.0) * GLOW;

    gl_FragColor = vec4(col, {alpha_expr});
}}
"#,
            r1 = r1, g1 = g1, b1 = b1,
            r2 = r2, g2 = g2, b2 = b2,
            r3 = r3, g3 = g3, b3 = b3,
            rbg = rbg, gbg = gbg, bbg = bbg,
            speed = state.speed,
            scale = state.scale,
            parallax = state.parallax,
            glow = state.glow,
            zoom_factor = zoom_factor,
            alpha_expr = alpha_expr
        ),

        ShaderArchetype::BlankCustom => format!(
            r#"// driftwm Shader Studio — Custom Scratchpad
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform vec2 u_camera;
uniform float u_time;
uniform float u_zoom;

const vec3 COLOR_PRIMARY   = vec3({r1:.3}, {g1:.3}, {b1:.3});
const vec3 COLOR_BG        = vec3({rbg:.3}, {gbg:.3}, {bbg:.3});

void main() {{
    vec2 uv = (v_coords * size + u_camera{zoom_factor}) / 200.0;
    vec3 col = mix(COLOR_BG, COLOR_PRIMARY, sin(uv.x) * 0.5 + 0.5);
    gl_FragColor = vec4(col, {alpha_expr});
}}
"#,
            r1 = r1, g1 = g1, b1 = b1,
            rbg = rbg, gbg = gbg, bbg = bbg,
            zoom_factor = zoom_factor,
            alpha_expr = alpha_expr
        ),
    }
}

pub fn generate_preview_svg(state: &ShaderStudioState) -> String {
    let hex_c1 = format!("#{:02x}{:02x}{:02x}", state.c1_r, state.c1_g, state.c1_b);
    let hex_c2 = format!("#{:02x}{:02x}{:02x}", state.c2_r, state.c2_g, state.c2_b);
    let hex_c3 = format!("#{:02x}{:02x}{:02x}", state.c3_r, state.c3_g, state.c3_b);
    let hex_bg = format!("#{:02x}{:02x}{:02x}", state.bg_r, state.bg_g, state.bg_b);

    let mut svg = String::with_capacity(4096);
    svg.push_str(r#"<svg viewBox="0 0 640 260" xmlns="http://www.w3.org/2000/svg">"#);
    svg.push_str("<defs>");
    svg.push_str(&format!(
        r#"<linearGradient id="pGrad" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" stop-color="{hex_c1}"/>
            <stop offset="50%" stop-color="{hex_c2}"/>
            <stop offset="100%" stop-color="{hex_c3}"/>
        </linearGradient>"#
    ));
    svg.push_str(&format!(
        r#"<radialGradient id="sunGrad" cx="50%" cy="50%" r="50%">
            <stop offset="0%" stop-color="{hex_c3}" stop-opacity="0.9"/>
            <stop offset="100%" stop-color="{hex_c1}" stop-opacity="0"/>
        </radialGradient>"#
    ));
    svg.push_str(&format!(
        r#"<filter id="blurGlow" x="-20%" y="-20%" width="140%" height="140%">
            <feGaussianBlur stdDeviation="8"/>
        </filter>"#
    ));
    svg.push_str("</defs>");

    // Background base
    svg.push_str(&format!(
        r#"<rect width="640" height="260" rx="8" fill="{hex_bg}"/>"#
    ));

    match state.archetype {
        ShaderArchetype::CyberGrid => {
            // Sun glow
            svg.push_str(&format!(
                r#"<circle cx="320" cy="120" r="50" fill="url(#sunGrad)"/>"#
            ));
            svg.push_str(&format!(
                r#"<circle cx="320" cy="120" r="32" fill="{hex_c3}" opacity="0.8"/>"#
            ));
            // Horizon line
            svg.push_str(&format!(
                r#"<line x1="0" y1="130" x2="640" y2="130" stroke="{hex_c3}" stroke-width="2" opacity="0.85"/>"#
            ));
            // Perspective grid rays
            let vanish_x = 320.0;
            let vanish_y = 130.0;
            for i in -8..=8 {
                let bottom_x = vanish_x + (i as f32) * 55.0;
                svg.push_str(&format!(
                    r#"<line x1="{vanish_x}" y1="{vanish_y}" x2="{bottom_x}" y2="260" stroke="{hex_c1}" stroke-width="1.3" opacity="0.65"/>"#
                ));
            }
            // Hyperbolic horizontal lines
            for k in 1..=8 {
                let t = (k as f32) / 8.0;
                let y = 130.0 + 130.0 * (t * t);
                let op = 0.2 + 0.7 * t;
                svg.push_str(&format!(
                    r#"<line x1="0" y1="{y:.1}" x2="640" y2="{y:.1}" stroke="{hex_c2}" stroke-width="1.2" opacity="{op:.2}"/>"#
                ));
            }
        }
        ShaderArchetype::PlasmaWaves => {
            // Layered waves
            svg.push_str(&format!(
                r#"<path d="M 0,90 Q 160,40 320,110 T 640,80 L 640,260 L 0,260 Z" fill="{hex_c1}" opacity="0.35"/>"#
            ));
            svg.push_str(&format!(
                r#"<path d="M 0,130 Q 180,180 340,120 T 640,160 L 640,260 L 0,260 Z" fill="{hex_c2}" opacity="0.45"/>"#
            ));
            svg.push_str(&format!(
                r#"<path d="M 0,170 Q 200,110 380,190 T 640,150 L 640,260 L 0,260 Z" fill="{hex_c3}" opacity="0.55"/>"#
            ));
            svg.push_str(&format!(
                r#"<circle cx="200" cy="100" r="40" fill="{hex_c1}" filter="url(#blurGlow)" opacity="0.4"/>"#
            ));
            svg.push_str(&format!(
                r#"<circle cx="450" cy="150" r="55" fill="{hex_c2}" filter="url(#blurGlow)" opacity="0.35"/>"#
            ));
        }
        ShaderArchetype::Starfield => {
            // Nebula glow
            svg.push_str(&format!(
                r#"<ellipse cx="240" cy="110" rx="140" ry="70" fill="{hex_c2}" opacity="0.35" filter="url(#blurGlow)"/>"#
            ));
            svg.push_str(&format!(
                r#"<ellipse cx="440" cy="150" rx="120" ry="60" fill="{hex_c3}" opacity="0.3" filter="url(#blurGlow)"/>"#
            ));
            // Stars
            let star_coords = [
                (50, 40, 1.5), (90, 120, 2.0), (140, 70, 1.2), (180, 180, 2.2),
                (230, 60, 3.0), (280, 140, 1.5), (320, 50, 2.0), (370, 190, 1.8),
                (410, 80, 2.5), (460, 40, 1.3), (510, 160, 2.2), (560, 90, 3.2),
                (600, 190, 1.5), (70, 210, 2.0), (340, 220, 1.6), (530, 230, 2.4)
            ];
            for (sx, sy, sr) in star_coords {
                svg.push_str(&format!(
                    r#"<circle cx="{sx}" cy="{sy}" r="{sr}" fill="{hex_c1}" opacity="0.85"/>"#
                ));
            }
            // Cross spikes on big stars
            for (bx, by) in [(230, 60), (560, 90)] {
                svg.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1" opacity="0.9"/>"#,
                    bx - 10, by, bx + 10, by, hex_c1
                ));
                svg.push_str(&format!(
                    r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="1" opacity="0.9"/>"#,
                    bx, by - 10, bx, by + 10, hex_c1
                ));
            }
        }
        ShaderArchetype::DigitalRain => {
            // Matrix columns
            for col in 0..24 {
                let x = 20 + col * 26;
                let y1 = (col * 17) % 80;
                let y2 = y1 + 100 + ((col * 31) % 60);
                svg.push_str(&format!(
                    r##"<line x1="{x}" y1="{y1}" x2="{x}" y2="{y2}" stroke="{hex_c1}" stroke-width="2.2" stroke-dasharray="3,5" opacity="0.8"/>"##
                ));
                // Glowing drop head
                svg.push_str(&format!(
                    r##"<circle cx="{x}" cy="{y2}" r="3" fill="#ffffff" opacity="0.95"/>"##
                ));
            }
        }
        ShaderArchetype::AuroraBorealis => {
            // Mountains silhouette
            svg.push_str(
                r##"<polygon points="0,260 90,200 190,230 300,180 420,220 540,190 640,240 640,260 0,260" fill="#0c0e17"/>"##
            );
            // Aurora vertical curtains
            for i in 0..5 {
                let x = 70 + i * 110;
                let q1_x = x - 25;
                let q1_target = x + 25;
                let q2_x = x + 15;
                let q2_ctrl = x + 35;
                let q2_target = x - 10;
                let q2_final = x + 15;
                svg.push_str(&format!(
                    r##"<path d="M {x},30 Q {q1_x},80 {q1_target},140 T {x},210" stroke="{hex_c1}" stroke-width="38" opacity="0.38" fill="none" filter="url(#blurGlow)"/>"##
                ));
                svg.push_str(&format!(
                    r##"<path d="M {q2_x},40 Q {q2_ctrl},90 {q2_target},150 T {q2_final},200" stroke="{hex_c2}" stroke-width="24" opacity="0.45" fill="none"/>"##
                ));
            }
        }
        ShaderArchetype::CatppuccinFlow => {
            svg.push_str(&format!(
                r##"<circle cx="180" cy="110" r="95" fill="{hex_c1}" opacity="0.55" filter="url(#blurGlow)"/>"##
            ));
            svg.push_str(&format!(
                r##"<circle cx="340" cy="140" r="105" fill="{hex_c2}" opacity="0.55" filter="url(#blurGlow)"/>"##
            ));
            svg.push_str(&format!(
                r##"<circle cx="480" cy="90" r="85" fill="{hex_c3}" opacity="0.5" filter="url(#blurGlow)"/>"##
            ));
        }
        ShaderArchetype::DotPulseGrid => {
            let spacing = (32.0f32 / state.scale).clamp(16.0f32, 64.0f32);
            let mut gx = 16.0f32;
            while gx < 640.0f32 {
                let mut gy = 16.0f32;
                while gy < 260.0f32 {
                    let dx = gx - 320.0f32;
                    let dy = gy - 130.0f32;
                    let d = (dx * dx + dy * dy).sqrt();
                    let wave = (d * 0.05f32).sin();
                    let r = 1.0f32 + wave.max(0.0f32) * 1.8f32;
                    let op = 0.3f32 + wave.max(0.0f32) * 0.6f32;
                    svg.push_str(&format!(
                        r##"<circle cx="{gx:.1}" cy="{gy:.1}" r="{r:.1}" fill="{hex_c1}" opacity="{op:.2}"/>"##
                    ));
                    gy += spacing;
                }
                gx += spacing;
            }
            // Concentric rings
            svg.push_str(&format!(
                r##"<circle cx="320" cy="130" r="60" stroke="{hex_c3}" stroke-width="1.5" fill="none" opacity="0.6" stroke-dasharray="6,6"/>"##
            ));
            svg.push_str(&format!(
                r##"<circle cx="320" cy="130" r="110" stroke="{hex_c2}" stroke-width="1.2" fill="none" opacity="0.4" stroke-dasharray="8,8"/>"##
            ));
        }
        ShaderArchetype::AcidLava => {
            svg.push_str(&format!(
                r##"<path d="M 40,20 Q 140,80 220,50 T 400,90 T 580,40" stroke="{hex_c3}" stroke-width="8" fill="none" opacity="0.75" filter="url(#blurGlow)"/>"##
            ));
            svg.push_str(&format!(
                r##"<path d="M 60,180 Q 200,120 320,190 T 520,160 T 600,220" stroke="{hex_c1}" stroke-width="12" fill="none" opacity="0.8" filter="url(#blurGlow)"/>"##
            ));
            svg.push_str(&format!(
                r##"<path d="M 60,180 Q 200,120 320,190 T 520,160 T 600,220" stroke="{hex_c2}" stroke-width="4" fill="none" opacity="0.95"/>"##
            ));
        }
        ShaderArchetype::BlankCustom => {
            svg.push_str(&format!(
                r##"<text x="320" y="110" fill="{hex_c1}" font-size="28" font-family="monospace" text-anchor="middle" font-weight="bold">{{ GLSL ES 1.0 }}</text>"##
            ));
            svg.push_str(&format!(
                r##"<text x="320" y="150" fill="{hex_c2}" font-size="14" font-family="monospace" text-anchor="middle">v_coords • size • u_camera • u_time • u_zoom</text>"##
            ));
        }
    }

    // Top watermark pill
    svg.push_str(
        r##"<rect x="14" y="12" width="168" height="24" rx="12" fill="#11111b" opacity="0.8"/>
        <text x="24" y="28" fill="#cdd6f4" font-size="11" font-family="sans-serif" font-weight="600">LIVE PREVIEW SIMULATION</text>"##
    );

    svg.push_str("</svg>");
    svg
}

pub fn view<'a>(state: &'a ShaderStudioState, lang: Language) -> Element<'a, ShaderStudioMessage> {
    let archetype_options: Vec<String> =
        ShaderArchetype::ALL.iter().map(|a| a.to_string()).collect();

    let hex1 = format!("#{:02X}{:02X}{:02X}", state.c1_r, state.c1_g, state.c1_b);
    let hex2 = format!("#{:02X}{:02X}{:02X}", state.c2_r, state.c2_g, state.c2_b);
    let hex3 = format!("#{:02X}{:02X}{:02X}", state.c3_r, state.c3_g, state.c3_b);
    let hex_bg = format!("#{:02X}{:02X}{:02X}", state.bg_r, state.bg_g, state.bg_b);

    // 1. Header Card
    let card_header = container(
        column![
            row![
                icons::icon_sparkles(mocha::MAUVE, 20.0),
                text(match lang {
                    Language::English => "Interactive Shader Studio",
                    Language::Russian => "Интерактивная студия шейдеров",
                })
                .size(20)
                .color(mocha::MAUVE),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            text(lang.shader_studio_desc())
                .size(13)
                .color(mocha::SUBTEXT0),
        ]
        .spacing(6),
    )
    .padding(18)
    .style(card_style)
    .width(Length::Fill);

    // 2. SVG Live Simulation Box & Palettes (Left Column)
    let svg_code = generate_preview_svg(state);
    let svg_handle = Handle::from_memory(svg_code.into_bytes());
    let svg_preview = iced::widget::svg(svg_handle)
        .width(Length::Fill)
        .height(Length::Fixed(240.0));

    let palette_buttons = row![
        text(lang.shader_palette_presets())
            .size(13)
            .color(mocha::SUBTEXT0)
            .width(Length::Fixed(120.0)),
        button(text("Mocha").size(11))
            .on_press(ShaderStudioMessage::PaletteSelected(
                PalettePreset::CatppuccinMocha
            ))
            .style(secondary_button_style),
        button(text("Cyberpunk").size(11))
            .on_press(ShaderStudioMessage::PaletteSelected(
                PalettePreset::CyberpunkNeon
            ))
            .style(secondary_button_style),
        button(text("Nordic").size(11))
            .on_press(ShaderStudioMessage::PaletteSelected(
                PalettePreset::NordicFrost
            ))
            .style(secondary_button_style),
        button(text("Sunset").size(11))
            .on_press(ShaderStudioMessage::PaletteSelected(
                PalettePreset::SunsetHorizon
            ))
            .style(secondary_button_style),
        button(text("Matrix").size(11))
            .on_press(ShaderStudioMessage::PaletteSelected(
                PalettePreset::EmeraldMatrix
            ))
            .style(secondary_button_style),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let left_column = container(
        column![
            text(lang.shader_preview_badge())
                .size(12)
                .color(mocha::LAVENDER),
            container(svg_preview)
                .padding(2)
                .style(card_style)
                .width(Length::Fill),
            palette_buttons,
        ]
        .spacing(12),
    )
    .padding(16)
    .style(card_style)
    .width(Length::FillPortion(1));

    // 3. Archetype & Math Parameters Controls (Right Column)
    let right_column = container(
        column![
            row![
                text(lang.shader_archetype())
                    .size(13)
                    .color(mocha::TEXT)
                    .width(Length::Fixed(160.0)),
                pick_list(
                    archetype_options,
                    Some(state.archetype.to_string()),
                    ShaderStudioMessage::ArchetypeSelected,
                )
                .style(pick_list_style)
                .width(Length::Fill),
            ]
            .spacing(10)
            .align_y(Alignment::Center),

            row![
                text(format!("{}: {:.1}x", lang.shader_speed(), state.speed))
                    .size(13)
                    .color(mocha::TEXT)
                    .width(Length::Fixed(190.0)),
                slider(0.0..=3.0, state.speed, ShaderStudioMessage::SpeedChanged)
                    .step(0.05_f32)
                    .style(slider_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_y(Alignment::Center),

            row![
                text(format!("{}: {:.1}x", lang.shader_scale(), state.scale))
                    .size(13)
                    .color(mocha::TEXT)
                    .width(Length::Fixed(190.0)),
                slider(0.2..=4.0, state.scale, ShaderStudioMessage::ScaleChanged)
                    .step(0.1_f32)
                    .style(slider_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_y(Alignment::Center),

            row![
                text(format!("{}: {:.2}", lang.shader_parallax(), state.parallax))
                    .size(13)
                    .color(mocha::TEXT)
                    .width(Length::Fixed(190.0)),
                slider(0.0..=1.0, state.parallax, ShaderStudioMessage::ParallaxChanged)
                    .step(0.05_f32)
                    .style(slider_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_y(Alignment::Center),

            row![
                text(format!("{}: {:.1}", lang.shader_glow(), state.glow))
                    .size(13)
                    .color(mocha::TEXT)
                    .width(Length::Fixed(190.0)),
                slider(0.4..=2.5, state.glow, ShaderStudioMessage::GlowChanged)
                    .step(0.1_f32)
                    .style(slider_style)
                    .width(Length::Fill),
            ]
            .spacing(10)
            .align_y(Alignment::Center),

            checkbox(lang.shader_zoom_reactive(), state.zoom_reactive)
                .on_toggle(ShaderStudioMessage::ZoomReactiveToggled)
                .size(15),

            checkbox(lang.shader_transparent(), state.transparent)
                .on_toggle(ShaderStudioMessage::TransparentToggled)
                .size(15),
        ]
        .spacing(12),
    )
    .padding(16)
    .style(card_style)
    .width(Length::FillPortion(1));

    let top_split = row![left_column, right_column].spacing(16);

    // 4. Color Channels Tuner
    let make_color_card = |label: &'static str,
                           hex: String,
                           r: u8,
                           g: u8,
                           b: u8,
                           on_r: fn(f32) -> ShaderStudioMessage,
                           on_g: fn(f32) -> ShaderStudioMessage,
                           on_b: fn(f32) -> ShaderStudioMessage| {
        container(
            column![
                row![
                    container(column![])
                        .width(Length::Fixed(22.0))
                        .height(Length::Fixed(22.0))
                        .style(move |_theme| {
                            container::Style {
                                background: Some(iced::Color::from_rgb8(r, g, b).into()),
                                border: iced::Border {
                                    color: mocha::SURFACE2,
                                    width: 1.0,
                                    radius: iced::border::Radius::from(4.0),
                                },
                                ..Default::default()
                            }
                        }),
                    column![
                        text(label).size(13).color(mocha::TEXT),
                        text(hex).size(11).color(mocha::SUBTEXT0),
                    ]
                    .spacing(1),
                ]
                .spacing(8)
                .align_y(Alignment::Center),

                row![
                    text("R:").size(11).color(mocha::RED).width(Length::Fixed(16.0)),
                    slider(0.0..=255.0, r as f32, on_r).step(1.0_f32).style(slider_style),
                ]
                .spacing(4)
                .align_y(Alignment::Center),

                row![
                    text("G:").size(11).color(mocha::GREEN).width(Length::Fixed(16.0)),
                    slider(0.0..=255.0, g as f32, on_g).step(1.0_f32).style(slider_style),
                ]
                .spacing(4)
                .align_y(Alignment::Center),

                row![
                    text("B:").size(11).color(mocha::BLUE).width(Length::Fixed(16.0)),
                    slider(0.0..=255.0, b as f32, on_b).step(1.0_f32).style(slider_style),
                ]
                .spacing(4)
                .align_y(Alignment::Center),
            ]
            .spacing(6),
        )
        .padding(12)
        .style(card_style)
        .width(Length::FillPortion(1))
    };

    let color_cards = row![
        make_color_card(
            lang.shader_color_primary(),
            hex1,
            state.c1_r,
            state.c1_g,
            state.c1_b,
            ShaderStudioMessage::Color1R,
            ShaderStudioMessage::Color1G,
            ShaderStudioMessage::Color1B
        ),
        make_color_card(
            lang.shader_color_secondary(),
            hex2,
            state.c2_r,
            state.c2_g,
            state.c2_b,
            ShaderStudioMessage::Color2R,
            ShaderStudioMessage::Color2G,
            ShaderStudioMessage::Color2B
        ),
        make_color_card(
            lang.shader_color_accent(),
            hex3,
            state.c3_r,
            state.c3_g,
            state.c3_b,
            ShaderStudioMessage::Color3R,
            ShaderStudioMessage::Color3G,
            ShaderStudioMessage::Color3B
        ),
        make_color_card(
            lang.shader_color_bg(),
            hex_bg,
            state.bg_r,
            state.bg_g,
            state.bg_b,
            ShaderStudioMessage::ColorBgR,
            ShaderStudioMessage::ColorBgG,
            ShaderStudioMessage::ColorBgB
        ),
    ]
    .spacing(12);

    // 5. Code Editor & Actions
    let editor_toolbar = row![
        text(lang.shader_code_title())
            .size(14)
            .color(mocha::MAUVE)
            .width(Length::Fill),
        checkbox(lang.shader_manual_mode(), state.manual_mode)
            .on_toggle(ShaderStudioMessage::ManualModeToggled)
            .size(15),
        button(text(lang.shader_reset_code()).size(12))
            .on_press(ShaderStudioMessage::ResetCodeToGenerated)
            .style(secondary_button_style),
        button(text(lang.shader_check_syntax()).size(12))
            .on_press(ShaderStudioMessage::ValidateCode)
            .style(secondary_button_style),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    let editor_widget = text_editor(&state.code_content)
        .on_action(ShaderStudioMessage::CodeAction)
        .height(Length::Fixed(220.0));

    let validation_banner: Element<'static, ShaderStudioMessage> =
        if let Some((valid, msg)) = &state.validation_msg {
            let color = if *valid { mocha::GREEN } else { mocha::YELLOW };
            container(text(msg.clone()).size(12).color(color))
                .padding(6)
                .into()
        } else {
            column![].into()
        };

    let editor_card = container(
        column![
            editor_toolbar,
            validation_banner,
            editor_widget,
        ]
        .spacing(10),
    )
    .padding(16)
    .style(card_style)
    .width(Length::Fill);

    // 6. Footer Action Bar (File Name, Save, Save & Apply)
    let footer_actions = container(
        column![
            row![
                text(lang.shader_filename_label())
                    .size(13)
                    .color(mocha::TEXT)
                    .width(Length::Fixed(140.0)),
                text_input("my_shader.glsl", &state.filename)
                    .on_input(ShaderStudioMessage::FilenameChanged)
                    .style(text_input_style)
                    .width(Length::Fixed(220.0)),
                column![].width(Length::Fill),
                button(
                    row![
                        icons::icon_save(mocha::BASE, 14.0),
                        text(lang.shader_save_button()).size(13),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center)
                )
                .on_press(ShaderStudioMessage::SaveShader)
                .style(primary_button_style)
                .padding(10),
                button(
                    row![
                        icons::icon_sparkles(mocha::BASE, 14.0),
                        text(lang.shader_apply_button()).size(13),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center)
                )
                .on_press(ShaderStudioMessage::SaveAndApply)
                .style(success_button_style)
                .padding(10),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            if let Some((success, msg)) = &state.status_msg {
                let color = if *success { mocha::GREEN } else { mocha::RED };
                container(text(msg.clone()).size(13).color(color)).padding(4)
            } else {
                container(column![])
            }
        ]
        .spacing(8),
    )
    .padding(16)
    .style(card_style)
    .width(Length::Fill);

    column![
        card_header,
        top_split,
        color_cards,
        editor_card,
        footer_actions,
    ]
    .spacing(18)
    .width(Length::Fill)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_archetypes_generate_valid_glsl() {
        for arch in ShaderArchetype::ALL {
            let mut state = ShaderStudioState::default();
            state.archetype = arch;
            let code = generate_glsl(&state);
            assert!(code.contains("precision highp float;"), "missing precision for {:?}", arch);
            assert!(code.contains("void main()"), "missing main for {:?}", arch);
            assert!(code.contains("gl_FragColor"), "missing output for {:?}", arch);
            assert!(code.contains("v_coords"), "missing v_coords for {:?}", arch);

            let svg = generate_preview_svg(&state);
            assert!(svg.starts_with("<svg"), "invalid svg start for {:?}", arch);
            assert!(svg.ends_with("</svg>"), "invalid svg end for {:?}", arch);
        }
    }

    #[test]
    fn test_palette_presets() {
        let mut state = ShaderStudioState::default();
        state.apply_palette(PalettePreset::CyberpunkNeon);
        assert_eq!(state.c1_r, 0);
        assert_eq!(state.c1_g, 255);
        assert_eq!(state.c1_b, 255);

        state.apply_palette(PalettePreset::NordicFrost);
        assert_eq!(state.c1_r, 136);
        assert_eq!(state.c1_g, 192);
        assert_eq!(state.c1_b, 208);
    }
}
