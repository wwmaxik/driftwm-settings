use crate::config::DriftwmConfig;
use crate::theme::{card_style, input_style, mocha, pick_list_style, slider_style};
use iced::widget::{checkbox, column, container, pick_list, row, slider, text, text_input};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub enum AppearanceMessage {
    DefaultModeChanged(String),
    TitleBarHeightChanged(f32),
    FontChanged(String),
    FontSizeChanged(f32),
    FontWeightChanged(String),
    TitleAlignChanged(String),
    BgColorChanged(String),
    FgColorChanged(String),
    BorderWidthChanged(f32),
    BorderColorChanged(String),
    BorderColorFocusedChanged(String),
    CornerRadiusChanged(f32),
    ShadowToggled(bool),
    SnapEnabledToggled(bool),
    GapChanged(f32),
    OuterGapChanged(f32),
    SnapDistanceChanged(f32),
    SnapBreakForceChanged(f32),
    SnapCornersToggled(bool),
    SnapCentersToggled(bool),
    OpacityChanged(f32),
    OpacityFocusedChanged(f32),
    BlurToggled(bool),
    BlurRadiusChanged(f32),
    BlurStrengthChanged(f32),
    AnimateBlurFpsChanged(f32),
    AnimationSpeedChanged(f32),
    AnimationScaleChanged(f32),
}

pub fn update(config: &mut DriftwmConfig, msg: AppearanceMessage) {
    match msg {
        AppearanceMessage::DefaultModeChanged(val) => {
            config.decorations.default_mode = Some(val);
        }
        AppearanceMessage::TitleBarHeightChanged(val) => {
            config.decorations.title_bar_height = Some(val.round() as i32);
        }
        AppearanceMessage::FontChanged(val) => {
            config.decorations.font = Some(val);
        }
        AppearanceMessage::FontSizeChanged(val) => {
            config.decorations.font_size = Some(val.round() as u32);
        }
        AppearanceMessage::FontWeightChanged(val) => {
            config.decorations.font_weight = Some(val);
        }
        AppearanceMessage::TitleAlignChanged(val) => {
            config.decorations.title_align = Some(val);
        }
        AppearanceMessage::BgColorChanged(val) => {
            config.decorations.bg_color = Some(val);
        }
        AppearanceMessage::FgColorChanged(val) => {
            config.decorations.fg_color = Some(val);
        }
        AppearanceMessage::BorderWidthChanged(val) => {
            config.decorations.border_width = Some(val.round() as i32);
        }
        AppearanceMessage::BorderColorChanged(val) => {
            config.decorations.border_color = Some(val);
        }
        AppearanceMessage::BorderColorFocusedChanged(val) => {
            config.decorations.border_color_focused = Some(val);
        }
        AppearanceMessage::CornerRadiusChanged(val) => {
            config.decorations.corner_radius = Some(val.round() as i32);
        }
        AppearanceMessage::ShadowToggled(val) => {
            config.decorations.shadow = Some(val);
        }
        AppearanceMessage::SnapEnabledToggled(val) => {
            config.snap.enabled = Some(val);
        }
        AppearanceMessage::GapChanged(val) => {
            config.snap.gap = Some(val as f64);
        }
        AppearanceMessage::OuterGapChanged(val) => {
            config.snap.outer_gap = Some(val as f64);
        }
        AppearanceMessage::SnapDistanceChanged(val) => {
            config.snap.distance = Some(val as f64);
        }
        AppearanceMessage::SnapBreakForceChanged(val) => {
            config.snap.break_force = Some(val as f64);
        }
        AppearanceMessage::SnapCornersToggled(val) => {
            config.snap.corners = Some(val);
        }
        AppearanceMessage::SnapCentersToggled(val) => {
            config.snap.centers = Some(val);
        }
        AppearanceMessage::OpacityChanged(val) => {
            config.decorations.opacity = Some(val as f64);
        }
        AppearanceMessage::OpacityFocusedChanged(val) => {
            config.decorations.opacity_focused = Some(val as f64);
        }
        AppearanceMessage::BlurToggled(val) => {
            config.decorations.blur = Some(val);
        }
        AppearanceMessage::BlurRadiusChanged(val) => {
            config.effects.blur_radius = Some(val.round() as u32);
        }
        AppearanceMessage::BlurStrengthChanged(val) => {
            config.effects.blur_strength = Some(val as f64);
        }
        AppearanceMessage::AnimateBlurFpsChanged(val) => {
            config.effects.animate_blur_fps = Some(val.round() as u32);
        }
        AppearanceMessage::AnimationSpeedChanged(val) => {
            config.effects.animation_speed = Some(val as f64);
        }
        AppearanceMessage::AnimationScaleChanged(val) => {
            config.effects.animation_scale = Some(val as f64);
        }
    }
}

use crate::i18n::Language;
use crate::icons;

pub fn view(config: &DriftwmConfig, lang: Language) -> Element<'static, AppearanceMessage> {
    let mode_options = vec!["client".to_string(), "minimal".to_string(), "none".to_string()];
    let current_mode = config
        .decorations
        .default_mode
        .clone()
        .unwrap_or_else(|| "client".to_string());

    let weight_options = vec![
        "regular".to_string(),
        "medium".to_string(),
        "semibold".to_string(),
        "bold".to_string(),
    ];
    let current_weight = config
        .decorations
        .font_weight
        .clone()
        .unwrap_or_else(|| "medium".to_string());

    let align_options = vec!["center".to_string(), "left".to_string()];
    let current_align = config
        .decorations
        .title_align
        .clone()
        .unwrap_or_else(|| "center".to_string());

    // Card 1: Window Chrome & SSD Titlebar
    let title_bar_h = config.decorations.title_bar_height.unwrap_or(25) as f32;
    let font_sz = config.decorations.font_size.unwrap_or(11) as f32;
    let font_str = config.decorations.font.clone().unwrap_or_else(|| "Adwaita Sans".to_string());
    let bg_color = config.decorations.bg_color.clone().unwrap_or_else(|| "#303030".to_string());
    let fg_color = config.decorations.fg_color.clone().unwrap_or_else(|| "#FFFFFF".to_string());

    let card_chrome = container(
        column![
            row![
                icons::icon_appearance(mocha::MAUVE, 18.0),
                text(match lang {
                    Language::English => "Window Chrome & Titlebar",
                    Language::Russian => "Заголовок и оформление окон",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Global window decoration mode and server-side decoration (SSD) typography.",
                Language::Russian => "Режим рамок окон и параметры серверных заголовков (SSD).",
            })
            .size(13)
            .color(mocha::SUBTEXT0),

            row![
                text(match lang {
                    Language::English => "Default Mode:",
                    Language::Russian => "Режим по умолчанию:",
                }).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                pick_list(mode_options, Some(current_mode), AppearanceMessage::DefaultModeChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Titlebar Height: {} px", title_bar_h as i32)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(16.0..=50.0, title_bar_h, AppearanceMessage::TitleBarHeightChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text("Font Family:").size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                text_input("Adwaita Sans", &font_str)
                    .on_input(AppearanceMessage::FontChanged)
                    .style(input_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Font Size: {} pt", font_sz as i32)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(8.0..=24.0, font_sz, AppearanceMessage::FontSizeChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text("Font Weight:").size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                pick_list(weight_options, Some(current_weight), AppearanceMessage::FontWeightChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text("Title Alignment:").size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                pick_list(align_options, Some(current_align), AppearanceMessage::TitleAlignChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text("Background Color:").size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                text_input("#303030", &bg_color)
                    .on_input(AppearanceMessage::BgColorChanged)
                    .style(input_style)
                    .width(Length::Fixed(120.0)),
                text("Foreground Color:").size(14).color(mocha::TEXT),
                text_input("#FFFFFF", &fg_color)
                    .on_input(AppearanceMessage::FgColorChanged)
                    .style(input_style)
                    .width(Length::Fixed(120.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    // Card 2: Borders & Corners
    let border_w = config.decorations.border_width.unwrap_or(0) as f32;
    let corner_r = config.decorations.corner_radius.unwrap_or(10) as f32;
    let border_col = config.decorations.border_color.clone().unwrap_or_else(|| "#303030".to_string());
    let border_col_foc = config.decorations.border_color_focused.clone().unwrap_or_else(|| "#303030".to_string());

    let card_borders = container(
        column![
            row![
                icons::icon_general(mocha::TEAL, 18.0),
                text(match lang {
                    Language::English => "Borders, Corners & Shadows",
                    Language::Russian => "Границы, скругление и тени",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Appearance of window borders, rounded corners and drop shadows.",
                Language::Russian => "Внешний вид рамок окон, скругление углов и отбрасывание теней.",
            })
            .size(13)
            .color(mocha::SUBTEXT0),

            row![
                text(match lang {
                    Language::English => format!("Border Width: {} px", border_w as i32),
                    Language::Russian => format!("Толщина границы: {} px", border_w as i32),
                }).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.0..=16.0, border_w, AppearanceMessage::BorderWidthChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(match lang {
                    Language::English => "Unfocused Border Color:",
                    Language::Russian => "Цвет неактивной границы:",
                }).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                text_input("#303030", &border_col)
                    .on_input(AppearanceMessage::BorderColorChanged)
                    .style(input_style)
                    .width(Length::Fixed(140.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(match lang {
                    Language::English => "Focused Border Color:",
                    Language::Russian => "Цвет активной границы:",
                }).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                text_input("#303030", &border_col_foc)
                    .on_input(AppearanceMessage::BorderColorFocusedChanged)
                    .style(input_style)
                    .width(Length::Fixed(140.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(match lang {
                    Language::English => format!("Corner Radius: {} px", corner_r as i32),
                    Language::Russian => format!("Радиус скругления: {} px", corner_r as i32),
                }).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.0..=32.0, corner_r, AppearanceMessage::CornerRadiusChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            checkbox(
                match lang {
                    Language::English => "Drop shadows (render soft shadows around floating/tiled windows)",
                    Language::Russian => "Отбрасывание теней (мягкие тени вокруг окон)",
                },
                config.decorations.shadow.unwrap_or(true)
            )
            .on_toggle(AppearanceMessage::ShadowToggled)
            .size(16),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    // Card 3: Spacing & Gaps
    let gap = config.snap.gap.unwrap_or(12.0) as f32;
    let outer_gap = config.snap.outer_gap.unwrap_or(0.0) as f32;
    let snap_dist = config.snap.distance.unwrap_or(24.0) as f32;
    let break_f = config.snap.break_force.unwrap_or(32.0) as f32;

    let card_spacing = container(
        column![
            row![
                icons::icon_settings(mocha::BLUE, 18.0),
                text(match lang {
                    Language::English => "Window Spacing & Magnetic Snapping",
                    Language::Russian => "Отступы окон и магнитное прилипание",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Configure window-to-window spacing (gap), screen edge inset (outer_gap), and snap thresholds.",
                Language::Russian => "Настройка расстояния между окнами (gap), отступа от краёв экрана (outer_gap) и порогов прилипания.",
            })
            .size(13)
            .color(mocha::SUBTEXT0),

            checkbox(
                match lang {
                    Language::English => "Enable magnetic snapping during window drag",
                    Language::Russian => "Включить магнитное прилипание при перетаскивании окон",
                },
                config.snap.enabled.unwrap_or(true)
            )
            .on_toggle(AppearanceMessage::SnapEnabledToggled)
            .size(16),

            row![
                text(match lang {
                    Language::English => format!("Window Gap: {:.1} px", gap),
                    Language::Russian => format!("Отступ окон: {:.1} px", gap),
                }).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.0..=48.0, gap, AppearanceMessage::GapChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Spacing between adjacent windows").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Outer Gap: {:.1} px", outer_gap)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.0..=48.0, outer_gap, AppearanceMessage::OuterGapChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Inset from screen edges to window edges").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Snap Distance: {:.0} px", snap_dist)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(4.0..=64.0, snap_dist, AppearanceMessage::SnapDistanceChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Snap Break Force: {:.0} px", break_f)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(8.0..=96.0, break_f, AppearanceMessage::SnapBreakForceChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                checkbox("Align corners (parallel edges line up)", config.snap.corners.unwrap_or(false))
                    .on_toggle(AppearanceMessage::SnapCornersToggled)
                    .size(16),
                checkbox("Align centers (midpoints line up)", config.snap.centers.unwrap_or(false))
                    .on_toggle(AppearanceMessage::SnapCentersToggled)
                    .size(16),
            ]
            .spacing(20),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    // Card 4: Opacity & Effects
    let op = config.decorations.opacity.unwrap_or(1.0) as f32;
    let op_foc = config.decorations.opacity_focused.unwrap_or(1.0) as f32;
    let blur_r = config.effects.blur_radius.unwrap_or(2) as f32;
    let blur_s = config.effects.blur_strength.unwrap_or(1.1) as f32;
    let blur_fps = config.effects.animate_blur_fps.unwrap_or(20) as f32;
    let anim_spd = config.effects.animation_speed.unwrap_or(0.5) as f32;
    let anim_scl = config.effects.animation_scale.unwrap_or(0.95) as f32;

    let card_effects = container(
        column![
            row![
                icons::icon_appearance(mocha::LAVENDER, 18.0),
                text(match lang {
                    Language::English => "Opacity & Backdrop Blur Effects",
                    Language::Russian => "Прозрачность и эффекты размытия",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Transparency, backdrop Kawase blur and window motion animation.",
                Language::Russian => "Прозрачность окон, размытие фона (Kawase blur) и плавные анимации.",
            })
            .size(13)
            .color(mocha::SUBTEXT0),

            row![
                text(format!("Unfocused Opacity: {:.2}", op)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.1..=1.0, op, AppearanceMessage::OpacityChanged)
                    .step(0.05_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Focused Opacity: {:.2}", op_foc)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.1..=1.0, op_foc, AppearanceMessage::OpacityFocusedChanged)
                    .step(0.05_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            checkbox("Global backdrop blur (frost backdrop of windows without rules)", config.decorations.blur.unwrap_or(false))
                .on_toggle(AppearanceMessage::BlurToggled)
                .size(16),

            row![
                text(format!("Blur Radius (Passes): {:.0}", blur_r)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(1.0..=8.0, blur_r, AppearanceMessage::BlurRadiusChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Blur Strength: {:.2}", blur_s)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.1..=3.0, blur_s, AppearanceMessage::BlurStrengthChanged)
                    .step(0.1_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Animate Blur FPS Cap: {:.0}", blur_fps)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.0..=120.0, blur_fps, AppearanceMessage::AnimateBlurFpsChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("0 = never re-sample animated background").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Animation Speed: {:.2}", anim_spd)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.1..=1.0, anim_spd, AppearanceMessage::AnimationSpeedChanged)
                    .step(0.05_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Window motion lerp factor (1 = instant)").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Animation Scale: {:.2}", anim_scl)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.5..=1.0, anim_scl, AppearanceMessage::AnimationScaleChanged)
                    .step(0.05_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Open/close amplitude (1 = fade only)").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    column![card_chrome, card_borders, card_spacing, card_effects]
        .spacing(18)
        .width(Length::Fill)
        .into()
}
