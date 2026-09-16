use crate::config::DriftwmConfig;
use crate::i18n::Language;
use crate::icons;
use crate::theme::{card_style, input_style, mocha, pick_list_style, primary_button_style, secondary_button_style, success_button_style};
use iced::widget::{button, checkbox, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element, Length};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct KeybindingsState {
    pub search_query: String,
    pub new_key: String,
    pub new_action: String,
    pub selected_preset: Option<String>,
}

#[derive(Debug, Clone)]
pub enum KeybindingsMessage {
    SearchChanged(String),
    NewKeyChanged(String),
    NewActionChanged(String),
    PresetSelected(String),
    AddKeybinding,
    RemoveKeybinding(String),
    ActionChanged(String, String),
    UnbindKey(String),
    PopulateDefaults,
    ToggleDisableDefault(String, bool),
}

pub fn update(config: &mut DriftwmConfig, state: &mut KeybindingsState, msg: KeybindingsMessage) {
    match msg {
        KeybindingsMessage::SearchChanged(q) => {
            state.search_query = q;
        }
        KeybindingsMessage::NewKeyChanged(k) => {
            state.new_key = k;
        }
        KeybindingsMessage::NewActionChanged(a) => {
            state.new_action = a;
        }
        KeybindingsMessage::PresetSelected(p) => {
            state.selected_preset = Some(p.clone());
            if p == "custom (spawn)" {
                state.new_action = "spawn ".to_string();
            } else if p == "custom (exec)" {
                state.new_action = "exec ".to_string();
            } else {
                state.new_action = p;
            }
        }
        KeybindingsMessage::AddKeybinding => {
            let key = state.new_key.trim().to_lowercase();
            let action = state.new_action.trim().to_string();
            if !key.is_empty() && !action.is_empty() {
                if config.keybindings.is_none() {
                    config.keybindings = Some(HashMap::new());
                }
                if let Some(map) = &mut config.keybindings {
                    map.insert(key, action);
                }
                state.new_key.clear();
                state.new_action.clear();
                state.selected_preset = None;
            }
        }
        KeybindingsMessage::RemoveKeybinding(key) => {
            if let Some(map) = &mut config.keybindings {
                map.remove(&key);
            }
        }
        KeybindingsMessage::ActionChanged(key, new_act) => {
            if let Some(map) = &mut config.keybindings {
                map.insert(key, new_act);
            }
        }
        KeybindingsMessage::UnbindKey(key) => {
            if let Some(map) = &mut config.keybindings {
                map.insert(key, "none".to_string());
            }
        }
        KeybindingsMessage::PopulateDefaults => {
            config.keybindings = Some(DriftwmConfig::default_keybindings());
        }
        KeybindingsMessage::ToggleDisableDefault(cat, disabled) => {
            let list = config.bindings.disable_defaults.get_or_insert_with(Vec::new);
            if disabled {
                if !list.contains(&cat) {
                    list.push(cat);
                }
            } else {
                list.retain(|item| item != &cat);
            }
        }
    }
}

pub fn view<'a>(
    config: &'a DriftwmConfig,
    state: &'a KeybindingsState,
    lang: Language,
) -> Element<'a, KeybindingsMessage> {
    // 1. Header Card with Search, Populate Defaults, and Quick Add
    let search_bar = row![
        icons::icon_search(mocha::SUBTEXT0, 16.0),
        text_input(lang.search_keybindings(), &state.search_query)
            .on_input(KeybindingsMessage::SearchChanged)
            .style(input_style)
            .padding(8)
            .width(Length::Fill),
        button(
            row![
                icons::icon_reload(mocha::TEXT, 14.0),
                text(lang.populate_defaults()).size(13),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .on_press(KeybindingsMessage::PopulateDefaults)
        .style(secondary_button_style)
        .padding(8),
    ]
    .spacing(12)
    .align_y(Alignment::Center);

    // Presets for quick actions
    let preset_actions = vec![
        "exec-terminal".to_string(),
        "exec-launcher".to_string(),
        "close-window".to_string(),
        "toggle-fullscreen".to_string(),
        "fit-window".to_string(),
        "fit-window-snapped".to_string(),
        "toggle-pin-to-screen".to_string(),
        "center-window".to_string(),
        "focus-center".to_string(),
        "home-toggle".to_string(),
        "cycle-windows forward".to_string(),
        "cycle-windows backward".to_string(),
        "zoom-in".to_string(),
        "zoom-out".to_string(),
        "zoom-reset".to_string(),
        "zoom-to-fit".to_string(),
        "zoom-to-fit-snapped".to_string(),
        "toggle-cursor-pan".to_string(),
        "quit".to_string(),
        "custom (spawn)".to_string(),
        "custom (exec)".to_string(),
        "none".to_string(),
    ];

    // Add New Keybinding Row
    let add_row = row![
        text_input("e.g. mod+t, ctrl+alt+t", &state.new_key)
            .on_input(KeybindingsMessage::NewKeyChanged)
            .style(input_style)
            .padding(8)
            .width(Length::Fixed(220.0)),
        pick_list(
            preset_actions,
            state.selected_preset.clone(),
            KeybindingsMessage::PresetSelected,
        )
        .placeholder(match lang {
            Language::English => "Choose action preset...",
            Language::Russian => "Выбрать действие...",
        })
        .style(pick_list_style)
        .width(Length::Fixed(220.0)),
        text_input(
            "Action or command",
            &state.new_action,
        )
        .on_input(KeybindingsMessage::NewActionChanged)
        .style(input_style)
        .padding(8)
        .width(Length::Fill),
        button(
            row![
                icons::icon_plus(mocha::BASE, 14.0),
                text(lang.add_keybinding()).size(13),
            ]
            .spacing(6)
            .align_y(Alignment::Center),
        )
        .on_press(KeybindingsMessage::AddKeybinding)
        .style(primary_button_style)
        .padding(8),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    let top_card = container(
        column![
            row![
                icons::icon_hotkeys(mocha::MAUVE, 20.0),
                text(lang.keybindings_heading()).size(18).color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(lang.keybindings_desc())
                .size(13)
                .color(mocha::SUBTEXT0),
            search_bar,
            add_row,
        ]
        .spacing(14),
    )
    .padding(16)
    .style(card_style)
    .width(Length::Fill);

    // 2. Disable Defaults (Categories: keys, mouse, gestures, touch)
    let disabled_list = config.bindings.disable_defaults.as_deref().unwrap_or(&[]);
    let is_keys_disabled = disabled_list.contains(&"keys".to_string());
    let is_mouse_disabled = disabled_list.contains(&"mouse".to_string());
    let is_gestures_disabled = disabled_list.contains(&"gestures".to_string());
    let is_touch_disabled = disabled_list.contains(&"touch".to_string());

    let defaults_card = container(
        column![
            text(lang.disable_defaults_heading())
                .size(15)
                .color(mocha::YELLOW),
            row![
                checkbox(lang.disable_keys_label(), is_keys_disabled)
                    .on_toggle(|val| KeybindingsMessage::ToggleDisableDefault("keys".to_string(), val)),
                checkbox(lang.disable_mouse_label(), is_mouse_disabled)
                    .on_toggle(|val| KeybindingsMessage::ToggleDisableDefault("mouse".to_string(), val)),
            ]
            .spacing(20),
            row![
                checkbox(lang.disable_gestures_label(), is_gestures_disabled)
                    .on_toggle(|val| KeybindingsMessage::ToggleDisableDefault("gestures".to_string(), val)),
                checkbox(lang.disable_touch_label(), is_touch_disabled)
                    .on_toggle(|val| KeybindingsMessage::ToggleDisableDefault("touch".to_string(), val)),
            ]
            .spacing(20),
        ]
        .spacing(10),
    )
    .padding(14)
    .style(card_style)
    .width(Length::Fill);

    // 3. Keybindings List
    let mut list_col = column![].spacing(8);

    let mut keys: Vec<&'a str> = if let Some(map) = &config.keybindings {
        map.keys().map(|s| s.as_str()).collect()
    } else {
        Vec::new()
    };
    keys.sort();

    let q = state.search_query.trim().to_lowercase();

    let mut displayed_count = 0;
    for key in keys {
        let action_str: &'a str = if let Some(map) = &config.keybindings {
            map.get(key).map(|s| s.as_str()).unwrap_or("")
        } else {
            ""
        };

        if !q.is_empty() && !key.to_lowercase().contains(&q) && !action_str.to_lowercase().contains(&q) {
            continue;
        }

        displayed_count += 1;
        let is_unbound = action_str == "none";

        let key_str = key.to_string();
        let key_for_delete = key.to_string();
        let key_for_unbind = key.to_string();

        let row_item = container(
            row![
                // Shortcut badge
                container(
                    text(key)
                        .size(13)
                        .color(mocha::MAUVE)
                )
                .padding(6)
                .style(|_theme| iced::widget::container::Style {
                    background: Some(iced::Background::Color(mocha::MANTLE)),
                    border: iced::Border {
                        color: mocha::SURFACE2,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                })
                .width(Length::Fixed(190.0)),

                // Action text input for live inline editing
                text_input("", action_str)
                    .on_input(move |new_val| KeybindingsMessage::ActionChanged(key_str.clone(), new_val))
                    .style(input_style)
                    .padding(6)
                    .width(Length::Fill),

                // Unbind button (sets to "none")
                button(text(lang.unbind()).size(11))
                    .on_press(KeybindingsMessage::UnbindKey(key_for_unbind))
                    .style(if is_unbound { success_button_style } else { secondary_button_style })
                    .padding(6),

                // Delete button
                button(icons::icon_trash(mocha::RED, 13.0))
                    .on_press(KeybindingsMessage::RemoveKeybinding(key_for_delete))
                    .style(secondary_button_style)
                    .padding(6),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        )
        .padding(8)
        .style(|_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(mocha::SURFACE0)),
            border: iced::Border {
                color: mocha::SURFACE1,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        })
        .width(Length::Fill);

        list_col = list_col.push(row_item);
    }

    if displayed_count == 0 {
        let empty_msg = if config.keybindings.is_none() || config.keybindings.as_ref().map_or(true, |m| m.is_empty()) {
            match lang {
                Language::English => "No keybindings configured. Click \"Load Standard Defaults\" above to populate built-in driftwm shortcuts!",
                Language::Russian => "Нет настроенных горячих клавиш. Нажмите «Загрузить стандартные бинды» выше, чтобы отобразить и настроить встроенные сочетания driftwm!",
            }
        } else {
            match lang {
                Language::English => "No keybindings match your search query.",
                Language::Russian => "По вашему запросу ничего не найдено.",
            }
        };

        list_col = list_col.push(
            container(
                text(empty_msg)
                    .size(13)
                    .color(mocha::SUBTEXT0),
            )
            .padding(16)
            .align_x(Alignment::Center)
            .width(Length::Fill),
        );
    }

    let list_card = container(
        column![
            row![
                text(format!(
                    "{}: {}",
                    lang.shortcut_action_header(),
                    displayed_count
                ))
                .size(14)
                .color(mocha::TEXT),
            ]
            .width(Length::Fill),
            list_col,
        ]
        .spacing(10),
    )
    .padding(16)
    .style(card_style)
    .width(Length::Fill);

    column![top_card, defaults_card, list_card]
        .spacing(16)
        .into()
}
