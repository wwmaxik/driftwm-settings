use crate::config::{DriftwmConfig, WindowRule};
use crate::i18n::Language;
use crate::icons;
use crate::theme::{card_style, input_style, mocha, pick_list_style, primary_button_style, secondary_button_style, slider_style};
use iced::widget::{button, checkbox, column, container, pick_list, row, slider, text, text_input};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub enum RulesMessage {
    AddRule,
    AddTemplatePip,
    AddTemplateGame,
    AddTemplateTerminal,
    RemoveRule(usize),
    AppIdChanged(usize, String),
    TitleChanged(usize, String),
    PinnedToggled(usize, bool),
    WidgetToggled(usize, bool),
    FullscreenToggled(usize, bool),
    FocusOnOpenToggled(usize, bool),
    BlurToggled(usize, bool),
    ShadowToggled(usize, bool),
    SuspendOnCloseToggled(usize, bool),
    RestoreWindowsToggled(usize, bool),
    PreserveAspectToggled(usize, bool),
    DecorationChanged(usize, String),
    OpacityChanged(usize, f64),
    CornerRadiusChanged(usize, i32),
    BorderWidthChanged(usize, i32),
    BorderColorChanged(usize, String),
    BorderColorFocusedChanged(usize, String),
    OutputChanged(usize, String),
}

pub fn update(config: &mut DriftwmConfig, msg: RulesMessage) {
    if config.window_rules.is_none() {
        config.window_rules = Some(Vec::new());
    }
    let rules = config.window_rules.as_mut().unwrap();

    match msg {
        RulesMessage::AddRule => {
            rules.push(WindowRule {
                app_id: Some("".to_string()),
                title: None,
                pinned_to_screen: false,
                widget: false,
                fullscreen: None,
                focus_on_open: None,
                suspend_on_close: None,
                restore_windows: None,
                preserve_aspect_ratio: false,
                decoration: Some("default".to_string()),
                blur: None,
                opacity: None,
                border_width: None,
                border_color: None,
                border_color_focused: None,
                corner_radius: None,
                shadow: None,
                output: None,
                position: None,
                size: None,
                layer_order: None,
            });
        }
        RulesMessage::AddTemplatePip => {
            rules.push(WindowRule {
                app_id: None,
                title: Some("Picture-in-Picture".to_string()),
                pinned_to_screen: true,
                widget: false,
                fullscreen: None,
                focus_on_open: Some(false),
                suspend_on_close: None,
                restore_windows: None,
                preserve_aspect_ratio: true,
                decoration: Some("client".to_string()),
                blur: None,
                opacity: None,
                border_width: None,
                border_color: None,
                border_color_focused: None,
                corner_radius: Some(8),
                shadow: Some(true),
                output: None,
                position: Some([0, -300]),
                size: Some([480, 270]),
                layer_order: None,
            });
        }
        RulesMessage::AddTemplateGame => {
            rules.push(WindowRule {
                app_id: Some("steam_app_*".to_string()),
                title: None,
                pinned_to_screen: false,
                widget: false,
                fullscreen: Some(true),
                focus_on_open: Some(true),
                suspend_on_close: None,
                restore_windows: None,
                preserve_aspect_ratio: false,
                decoration: Some("none".to_string()),
                blur: Some(false),
                opacity: Some(1.0),
                border_width: Some(0),
                border_color: None,
                border_color_focused: None,
                corner_radius: Some(0),
                shadow: Some(false),
                output: None,
                position: None,
                size: None,
                layer_order: None,
            });
        }
        RulesMessage::AddTemplateTerminal => {
            rules.push(WindowRule {
                app_id: Some("kitty".to_string()),
                title: None,
                pinned_to_screen: false,
                widget: false,
                fullscreen: None,
                focus_on_open: None,
                suspend_on_close: None,
                restore_windows: None,
                preserve_aspect_ratio: false,
                decoration: Some("client".to_string()),
                blur: Some(true),
                opacity: Some(0.9),
                border_width: Some(2),
                border_color: Some("#303030".to_string()),
                border_color_focused: Some("#cba6f7".to_string()),
                corner_radius: Some(10),
                shadow: Some(true),
                output: None,
                position: None,
                size: None,
                layer_order: None,
            });
        }
        RulesMessage::RemoveRule(idx) => {
            if idx < rules.len() {
                rules.remove(idx);
            }
        }
        RulesMessage::AppIdChanged(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.app_id = if val.is_empty() { None } else { Some(val) };
            }
        }
        RulesMessage::TitleChanged(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.title = if val.is_empty() { None } else { Some(val) };
            }
        }
        RulesMessage::PinnedToggled(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.pinned_to_screen = val;
            }
        }
        RulesMessage::WidgetToggled(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.widget = val;
            }
        }
        RulesMessage::FullscreenToggled(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.fullscreen = Some(val);
            }
        }
        RulesMessage::FocusOnOpenToggled(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.focus_on_open = Some(val);
            }
        }
        RulesMessage::BlurToggled(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.blur = Some(val);
            }
        }
        RulesMessage::ShadowToggled(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.shadow = Some(val);
            }
        }
        RulesMessage::SuspendOnCloseToggled(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.suspend_on_close = Some(val);
            }
        }
        RulesMessage::RestoreWindowsToggled(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.restore_windows = Some(val);
            }
        }
        RulesMessage::PreserveAspectToggled(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.preserve_aspect_ratio = val;
            }
        }
        RulesMessage::DecorationChanged(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.decoration = Some(val);
            }
        }
        RulesMessage::OpacityChanged(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.opacity = Some(val);
            }
        }
        RulesMessage::CornerRadiusChanged(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.corner_radius = Some(val);
            }
        }
        RulesMessage::BorderWidthChanged(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.border_width = Some(val);
            }
        }
        RulesMessage::BorderColorChanged(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.border_color = if val.is_empty() { None } else { Some(val) };
            }
        }
        RulesMessage::BorderColorFocusedChanged(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.border_color_focused = if val.is_empty() { None } else { Some(val) };
            }
        }
        RulesMessage::OutputChanged(idx, val) => {
            if let Some(r) = rules.get_mut(idx) {
                r.output = if val.is_empty() { None } else { Some(val) };
            }
        }
    }
}

pub fn view<'a>(
    config: &'a DriftwmConfig,
    lang: Language,
) -> Element<'a, RulesMessage> {
    let top_card = container(
        column![
            row![
                icons::icon_rules(mocha::MAUVE, 20.0),
                text(lang.rules_heading()).size(18).color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(lang.rules_desc()).size(13).color(mocha::SUBTEXT0),
            row![
                button(
                    row![
                        icons::icon_plus(mocha::BASE, 14.0),
                        text(lang.add_rule()).size(13),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center),
                )
                .on_press(RulesMessage::AddRule)
                .style(primary_button_style)
                .padding(8),

                button(text("+ PiP Preset").size(12))
                    .on_press(RulesMessage::AddTemplatePip)
                    .style(secondary_button_style)
                    .padding(8),

                button(text("+ Game Preset").size(12))
                    .on_press(RulesMessage::AddTemplateGame)
                    .style(secondary_button_style)
                    .padding(8),

                button(text("+ Terminal Preset").size(12))
                    .on_press(RulesMessage::AddTemplateTerminal)
                    .style(secondary_button_style)
                    .padding(8),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        ]
        .spacing(12),
    )
    .padding(16)
    .style(card_style)
    .width(Length::Fill);

    let mut rules_col = column![].spacing(16);

    let rules = config.window_rules.as_deref().unwrap_or(&[]);

    let deco_options = vec![
        "default".to_string(),
        "client".to_string(),
        "server".to_string(),
        "minimal".to_string(),
        "none".to_string(),
    ];

    for (idx, rule) in rules.iter().enumerate() {
        let rule_title_label = if let Some(app_id) = &rule.app_id {
            format!("Rule #{}: {}", idx + 1, app_id)
        } else if let Some(title) = &rule.title {
            format!("Rule #{}: [Title] {}", idx + 1, title)
        } else {
            format!("Rule #{}", idx + 1)
        };

        let opacity_val = rule.opacity.unwrap_or(1.0);
        let corner_radius_val = rule.corner_radius.unwrap_or(10);
        let border_width_val = rule.border_width.unwrap_or(0);

        let card = container(
            column![
                // Rule header
                row![
                    icons::icon_rules(mocha::LAVENDER, 16.0),
                    text(rule_title_label).size(15).color(mocha::LAVENDER),
                    row![].width(Length::Fill),
                    button(
                        row![
                            icons::icon_trash(mocha::RED, 13.0),
                            text(lang.delete()).size(12).color(mocha::RED),
                        ]
                        .spacing(4)
                        .align_y(Alignment::Center)
                    )
                    .on_press(RulesMessage::RemoveRule(idx))
                    .style(secondary_button_style)
                    .padding(6),
                ]
                .spacing(8)
                .align_y(Alignment::Center),

                // Match fields
                row![
                    column![
                        text(lang.rule_app_id()).size(12).color(mocha::TEXT),
                        text_input("e.g. kitty, firefox, steam_app_*", rule.app_id.as_deref().unwrap_or(""))
                            .on_input(move |v| RulesMessage::AppIdChanged(idx, v))
                            .style(input_style)
                            .padding(6),
                    ]
                    .spacing(4)
                    .width(Length::FillPortion(1)),

                    column![
                        text(lang.rule_title()).size(12).color(mocha::TEXT),
                        text_input("e.g. Picture-in-Picture", rule.title.as_deref().unwrap_or(""))
                            .on_input(move |v| RulesMessage::TitleChanged(idx, v))
                            .style(input_style)
                            .padding(6),
                    ]
                    .spacing(4)
                    .width(Length::FillPortion(1)),
                ]
                .spacing(12),

                // Flags
                row![
                    checkbox(lang.rule_pinned(), rule.pinned_to_screen)
                        .on_toggle(move |v| RulesMessage::PinnedToggled(idx, v)),
                    checkbox(lang.rule_widget(), rule.widget)
                        .on_toggle(move |v| RulesMessage::WidgetToggled(idx, v)),
                    checkbox(lang.rule_fullscreen(), rule.fullscreen.unwrap_or(false))
                        .on_toggle(move |v| RulesMessage::FullscreenToggled(idx, v)),
                ]
                .spacing(16),

                row![
                    checkbox(lang.rule_blur(), rule.blur.unwrap_or(false))
                        .on_toggle(move |v| RulesMessage::BlurToggled(idx, v)),
                    checkbox("Shadow", rule.shadow.unwrap_or(true))
                        .on_toggle(move |v| RulesMessage::ShadowToggled(idx, v)),
                    checkbox(lang.rule_focus_on_open(), rule.focus_on_open.unwrap_or(true))
                        .on_toggle(move |v| RulesMessage::FocusOnOpenToggled(idx, v)),
                ]
                .spacing(16),

                row![
                    checkbox("Preserve aspect ratio", rule.preserve_aspect_ratio)
                        .on_toggle(move |v| RulesMessage::PreserveAspectToggled(idx, v)),
                    checkbox("Suspend on close", rule.suspend_on_close.unwrap_or(false))
                        .on_toggle(move |v| RulesMessage::SuspendOnCloseToggled(idx, v)),
                    checkbox("Restore in session", rule.restore_windows.unwrap_or(false))
                        .on_toggle(move |v| RulesMessage::RestoreWindowsToggled(idx, v)),
                ]
                .spacing(16),

                // Decoration mode
                row![
                    text(lang.rule_decoration()).size(12).color(mocha::TEXT).width(Length::Fixed(180.0)),
                    pick_list(
                        deco_options.clone(),
                        rule.decoration.clone(),
                        move |v| RulesMessage::DecorationChanged(idx, v)
                    )
                    .style(pick_list_style)
                    .width(Length::Fixed(160.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),

                // Opacity slider
                row![
                    text(format!("{}: {:.2}", lang.rule_opacity(), opacity_val))
                        .size(12)
                        .color(mocha::TEXT)
                        .width(Length::Fixed(180.0)),
                    slider(0.1..=1.0, opacity_val as f32, move |val| RulesMessage::OpacityChanged(idx, (val as f64 * 100.0).round() / 100.0))
                        .style(slider_style)
                        .width(Length::Fixed(220.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),

                // Corner radius slider
                row![
                    text(format!("Corner radius: {} px", corner_radius_val))
                        .size(12)
                        .color(mocha::TEXT)
                        .width(Length::Fixed(180.0)),
                    slider(0.0..=30.0, corner_radius_val as f32, move |val| RulesMessage::CornerRadiusChanged(idx, val.round() as i32))
                        .style(slider_style)
                        .width(Length::Fixed(220.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),

                // Border width slider
                row![
                    text(format!("Border width: {} px", border_width_val))
                        .size(12)
                        .color(mocha::TEXT)
                        .width(Length::Fixed(180.0)),
                    slider(0.0..=12.0, border_width_val as f32, move |val| RulesMessage::BorderWidthChanged(idx, val.round() as i32))
                        .style(slider_style)
                        .width(Length::Fixed(220.0)),
                ]
                .spacing(12)
                .align_y(Alignment::Center),

                // Colors & Output
                row![
                    column![
                        text("Border Color:").size(12).color(mocha::TEXT),
                        text_input("#303030", rule.border_color.as_deref().unwrap_or(""))
                            .on_input(move |v| RulesMessage::BorderColorChanged(idx, v))
                            .style(input_style)
                            .padding(6)
                            .width(Length::Fixed(140.0)),
                    ]
                    .spacing(4),

                    column![
                        text("Focused Border:").size(12).color(mocha::TEXT),
                        text_input("#cba6f7", rule.border_color_focused.as_deref().unwrap_or(""))
                            .on_input(move |v| RulesMessage::BorderColorFocusedChanged(idx, v))
                            .style(input_style)
                            .padding(6)
                            .width(Length::Fixed(140.0)),
                    ]
                    .spacing(4),

                    column![
                        text("Target Output:").size(12).color(mocha::TEXT),
                        text_input("e.g. DP-1", rule.output.as_deref().unwrap_or(""))
                            .on_input(move |v| RulesMessage::OutputChanged(idx, v))
                            .style(input_style)
                            .padding(6)
                            .width(Length::Fixed(140.0)),
                    ]
                    .spacing(4),
                ]
                .spacing(16)
                .align_y(Alignment::Center),
            ]
            .spacing(12),
        )
        .padding(14)
        .style(card_style)
        .width(Length::Fill);

        rules_col = rules_col.push(card);
    }

    if rules.is_empty() {
        rules_col = rules_col.push(
            container(
                text(match lang {
                    Language::English => "No window rules configured. Click \"+ Add Window Rule\" or select a preset above!",
                    Language::Russian => "Нет настроенных правил окон. Нажмите «+ Добавить правило» или выберите пресет выше!",
                })
                .size(13)
                .color(mocha::SUBTEXT0),
            )
            .padding(20)
            .align_x(Alignment::Center)
            .width(Length::Fill),
        );
    }

    column![top_card, rules_col]
        .spacing(16)
        .into()
}
