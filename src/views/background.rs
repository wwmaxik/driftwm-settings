use crate::config::DriftwmConfig;
use crate::theme::{
    card_style, input_style, mocha, pick_list_style, secondary_button_style, slider_style,
};
use iced::widget::{button, checkbox, column, container, pick_list, row, slider, text, text_input};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub enum BackgroundMessage {
    KindChanged(String),
    PathChanged(String),
    TextureChanged(String),
    MirrorTileToggled(bool),
    CacheShaderToggled(bool),
    TransparentShaderToggled(bool),
    CacheBudgetChanged(f32),
    AnimateFpsChanged(f32),
    ApplyPreset {
        kind: String,
        path: String,
        texture: Option<String>,
    },
    OpenShaderStudio,
}

pub fn update(config: &mut DriftwmConfig, msg: BackgroundMessage) {
    match msg {
        BackgroundMessage::KindChanged(val) => {
            config.background.kind = Some(val);
        }
        BackgroundMessage::PathChanged(val) => {
            config.background.path = if val.trim().is_empty() {
                None
            } else {
                Some(val)
            };
        }
        BackgroundMessage::TextureChanged(val) => {
            config.background.texture = if val.trim().is_empty() {
                None
            } else {
                Some(val)
            };
        }
        BackgroundMessage::MirrorTileToggled(val) => {
            config.background.mirror_tile = Some(val);
        }
        BackgroundMessage::CacheShaderToggled(val) => {
            config.background.cache_shader = Some(val);
        }
        BackgroundMessage::TransparentShaderToggled(val) => {
            config.background.transparent_shader = Some(val);
        }
        BackgroundMessage::CacheBudgetChanged(val) => {
            config.background.cache_budget_mb = Some(val.round() as u32);
        }
        BackgroundMessage::AnimateFpsChanged(val) => {
            config.background.animate_fps = Some(val.round() as u32);
        }
        BackgroundMessage::ApplyPreset {
            kind,
            path,
            texture,
        } => {
            config.background.kind = Some(kind);
            config.background.path = Some(path);
            config.background.texture = texture;
        }
        BackgroundMessage::OpenShaderStudio => {}
    }
}

use crate::i18n::Language;
use crate::icons;

pub fn view(config: &DriftwmConfig, lang: Language) -> Element<'static, BackgroundMessage> {
    let kind_options = vec![
        "default".to_string(),
        "shader".to_string(),
        "tile".to_string(),
        "wallpaper".to_string(),
        "none".to_string(),
    ];
    let current_kind = config
        .background
        .kind
        .clone()
        .unwrap_or_else(|| "default".to_string());

    let path_val = config.background.path.clone().unwrap_or_default();
    let texture_val = config.background.texture.clone().unwrap_or_default();
    let cache_budget = config.background.cache_budget_mb.unwrap_or(128) as f32;
    let anim_fps = config.background.animate_fps.unwrap_or(0) as f32;

    let desc = match (current_kind.as_str(), lang) {
        ("default", Language::Russian) => "По умолчанию: встроенная точечная сетка Catppuccin с динамическим масштабированием.",
        ("default", Language::English) => "Default: Built-in dot-grid canvas with dynamic scale.",
        ("shader", Language::Russian) => "Шейдер: процедурный GLSL-шейдер, плавно панорамируемый вместе с холстом.",
        ("shader", Language::English) => "Shader: Procedural GLSL wallpaper shader that pans and zooms with the canvas.",
        ("tile", Language::Russian) => "Тайлинг: бесшовная текстура, замостившая весь бесконечный холст.",
        ("tile", Language::English) => "Tile: Image tiled seamlessly across the infinite canvas.",
        ("wallpaper", Language::Russian) => "Обои: фиксированная картинка, привязанная к экрану (не скроллится).",
        ("wallpaper", Language::English) => "Wallpaper: Fixed single image pinned to viewport (does not scroll or zoom).",
        ("none", Language::Russian) => "Отключен: без фона (для внешних демонов swaybg, swww, mpvpaper).",
        ("none", Language::English) => "None: No built-in wallpaper. Use with external Wayland daemons (swaybg, swww, mpvpaper).",
        _ => "",
    };

    let card_kind = container(
        column![
            row![
                icons::icon_background(mocha::MAUVE, 18.0),
                text(match lang {
                    Language::English => "Background Mode",
                    Language::Russian => "Режим фона рабочего стола",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Select between built-in dot grid, custom GLSL shader, image tile, fixed wallpaper or external daemon.",
                Language::Russian => "Выбор между сеткой, процедурным шейдером, тайлингом текстуры, обоями или внешним демоном.",
            })
            .size(13)
            .color(mocha::SUBTEXT0),

            row![
                text("Background Type:").size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                pick_list(kind_options, Some(current_kind.clone()), BackgroundMessage::KindChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(200.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            container(
                text(desc)
                    .size(13)
                    .color(mocha::LAVENDER)
            )
            .padding(8),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    let card_source = match current_kind.as_str() {
        "shader" => container(
            column![
                row![
                    column![
                        text("Shader Configuration")
                            .size(18)
                            .color(mocha::MAUVE),
                        text("Specify the GLSL shader path and optional texture sampler.")
                            .size(13)
                            .color(mocha::SUBTEXT0),
                    ]
                    .width(Length::Fill),
                    button(
                        row![
                            icons::icon_sparkles(mocha::BASE, 14.0),
                            text(match lang {
                                Language::English => "✨ Open Shader Studio",
                                Language::Russian => "✨ Студия шейдеров",
                            })
                            .size(13),
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center)
                    )
                    .on_press(BackgroundMessage::OpenShaderStudio)
                    .style(crate::theme::primary_button_style)
                    .padding(8),
                ]
                .align_y(Alignment::Center),

                row![
                    text("Shader File (.glsl):").size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                    text_input("/usr/local/share/driftwm/wallpapers/animated/fast_smoke.glsl", &path_val)
                        .on_input(BackgroundMessage::PathChanged)
                        .style(input_style)
                        .width(Length::Fill),
                ]
                .spacing(12)
                .align_y(Alignment::Center),

                row![
                    text("Optional Texture:").size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                    text_input("~/Pictures/Wallpapers/photo.jpg", &texture_val)
                        .on_input(BackgroundMessage::TextureChanged)
                        .style(input_style)
                        .width(Length::Fill),
                ]
                .spacing(12)
                .align_y(Alignment::Center),

                row![
                    text("Presets:").size(13).color(mocha::SUBTEXT0).width(Length::Fixed(80.0)),
                    button(text("Smoke").size(12))
                        .on_press(BackgroundMessage::ApplyPreset {
                            kind: "shader".to_string(),
                            path: "/usr/local/share/driftwm/wallpapers/animated/fast_smoke.glsl".to_string(),
                            texture: None,
                        })
                        .style(secondary_button_style),
                    button(text("Grid Waves").size(12))
                        .on_press(BackgroundMessage::ApplyPreset {
                            kind: "shader".to_string(),
                            path: "/usr/local/share/driftwm/wallpapers/animated/grid_waves.glsl".to_string(),
                            texture: None,
                        })
                        .style(secondary_button_style),
                    button(text("Ripple Texture").size(12))
                        .on_press(BackgroundMessage::ApplyPreset {
                            kind: "shader".to_string(),
                            path: "/usr/local/share/driftwm/wallpapers/textured/ripple.glsl".to_string(),
                            texture: Some("~/Pictures/Wallpapers/photo.jpg".to_string()),
                        })
                        .style(secondary_button_style),
                ]
                .spacing(10)
                .align_y(Alignment::Center),

                row![
                    text(format!("FPS Cap: {:.0}", anim_fps)).size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                    slider(0.0..=144.0, anim_fps, BackgroundMessage::AnimateFpsChanged)
                        .style(slider_style)
                        .width(Length::Fixed(220.0)),
                    text("0 = full display refresh rate").size(12).color(mocha::SUBTEXT0),
                ]
                .spacing(12)
                .align_y(Alignment::Center),

                row![
                    text(format!("Cache Budget: {:.0} MB", cache_budget)).size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                    slider(32.0..=512.0, cache_budget, BackgroundMessage::CacheBudgetChanged)
                        .step(16.0_f32)
                        .style(slider_style)
                        .width(Length::Fixed(220.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),

                checkbox("Cache static shader (bake to texture once and pan that)", config.background.cache_shader.unwrap_or(false))
                    .on_toggle(BackgroundMessage::CacheShaderToggled)
                    .size(16),

                checkbox("Transparent shader (honor shader alpha channel for backdrop passthrough)", config.background.transparent_shader.unwrap_or(false))
                    .on_toggle(BackgroundMessage::TransparentShaderToggled)
                    .size(16),
            ]
            .spacing(14),
        )
        .padding(20)
        .style(card_style)
        .width(Length::Fill),

        "tile" => container(
            column![
                text("Tiled Background")
                    .size(18)
                    .color(mocha::MAUVE),
                text("Specify an image to repeat across the canvas.")
                    .size(13)
                    .color(mocha::SUBTEXT0),

                row![
                    text("Tile Image Path:").size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                    text_input("~/Pictures/Wallpapers/tile.png", &path_val)
                        .on_input(BackgroundMessage::PathChanged)
                        .style(input_style)
                        .width(Length::Fill),
                ]
                .spacing(12)
                .align_y(Alignment::Center),

                checkbox("Mirror-fold tile (2x2 reflected block to eliminate seam lines)", config.background.mirror_tile.unwrap_or(false))
                    .on_toggle(BackgroundMessage::MirrorTileToggled)
                    .size(16),
            ]
            .spacing(14),
        )
        .padding(20)
        .style(card_style)
        .width(Length::Fill),

        "wallpaper" => container(
            column![
                text("Static Wallpaper")
                    .size(18)
                    .color(mocha::MAUVE),
                text("Specify a single image to fix to the viewport.")
                    .size(13)
                    .color(mocha::SUBTEXT0),

                row![
                    text("Wallpaper Image Path:").size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                    text_input("~/Pictures/Wallpapers/wallpaper.jpg", &path_val)
                        .on_input(BackgroundMessage::PathChanged)
                        .style(input_style)
                        .width(Length::Fill),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            ]
            .spacing(14),
        )
        .padding(20)
        .style(card_style)
        .width(Length::Fill),

        _ => container(
            column![
                text("Active Background Mode: Default / External")
                    .size(16)
                    .color(mocha::LAVENDER),
                text("No extra path settings needed for this mode.")
                    .size(13)
                    .color(mocha::SUBTEXT0),
            ]
            .spacing(8),
        )
        .padding(20)
        .style(card_style)
        .width(Length::Fill),
    };

    column![card_kind, card_source]
        .spacing(18)
        .width(Length::Fill)
        .into()
}
