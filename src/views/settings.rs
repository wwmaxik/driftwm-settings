use iced::widget::{button, column, container, pick_list, row, text, text_input, toggler};
use iced::{Alignment, Element, Length};

use crate::app_settings::AppSettings;
use crate::config::DriftwmConfig;
use crate::i18n::Language;
use crate::icons;
use crate::theme::{self, card_style, input_style, mocha, secondary_button_style};

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    LanguageChanged(Language),
    ConfigPathChanged(String),
    ResetConfigPath,
    AutoValidateToggled(bool),
}

pub fn update(settings: &mut AppSettings, message: SettingsMessage) -> bool {
    let mut changed = false;
    match message {
        SettingsMessage::LanguageChanged(lang) => {
            if settings.language != lang {
                settings.language = lang;
                changed = true;
            }
        }
        SettingsMessage::ConfigPathChanged(path) => {
            settings.custom_config_path = if path.trim().is_empty() {
                None
            } else {
                Some(path)
            };
            changed = true;
        }
        SettingsMessage::ResetConfigPath => {
            settings.custom_config_path = None;
            changed = true;
        }
        SettingsMessage::AutoValidateToggled(val) => {
            settings.auto_validate = val;
            changed = true;
        }
    }
    if changed {
        let _ = settings.save();
    }
    changed
}

pub fn view<'a>(settings: &'a AppSettings) -> Element<'a, SettingsMessage> {
    let lang = settings.language;

    // Header
    let header = column![
        row![
            icons::icon_settings(mocha::MAUVE, 22.0),
            text(lang.settings_heading())
                .size(24)
                .color(mocha::TEXT),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        text(lang.settings_subheading())
            .size(13)
            .color(mocha::SUBTEXT0),
    ]
    .spacing(4);

    // Language Card
    let lang_card = container(
        column![
            row![
                icons::icon_globe(mocha::BLUE, 18.0),
                text(lang.language_section())
                    .size(16)
                    .color(mocha::TEXT),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(lang.language_desc())
                .size(12)
                .color(mocha::SUBTEXT0),
            row![
                text("Language / Язык:")
                    .size(13)
                    .color(mocha::SUBTEXT0)
                    .width(Length::Fixed(150.0)),
                pick_list(
                    &Language::ALL[..],
                    Some(settings.language),
                    SettingsMessage::LanguageChanged,
                )
                .style(theme::pick_list_style)
                .width(Length::Fixed(200.0)),
            ]
            .spacing(16)
            .align_y(Alignment::Center),
        ]
        .spacing(12),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    // Config Path Card
    let current_path = settings
        .custom_config_path
        .clone()
        .unwrap_or_else(|| DriftwmConfig::default_path().display().to_string());

    let config_card = container(
        column![
            row![
                icons::icon_general(mocha::TEAL, 18.0),
                text(lang.config_path_section())
                    .size(16)
                    .color(mocha::TEXT),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(lang.config_path_desc())
                .size(12)
                .color(mocha::SUBTEXT0),
            row![
                text_input(&current_path, &current_path)
                    .on_input(SettingsMessage::ConfigPathChanged)
                    .style(input_style)
                    .width(Length::Fill),
                button(
                    row![
                        icons::icon_reload(mocha::TEXT, 14.0),
                        text(lang.reset_default()).size(13),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center)
                )
                .on_press(SettingsMessage::ResetConfigPath)
                .style(secondary_button_style)
                .padding(8),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        ]
        .spacing(12),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    // Validation Card
    let validation_card = container(
        column![
            row![
                icons::icon_check(mocha::GREEN, 18.0),
                text(lang.live_validation_section())
                    .size(16)
                    .color(mocha::TEXT),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(lang.live_validation_desc())
                .size(12)
                .color(mocha::SUBTEXT0),
            row![
                toggler(settings.auto_validate)
                    .label(match lang {
                        Language::English => "Validate config syntax automatically",
                        Language::Russian => "Автоматически проверять синтаксис конфига",
                    })
                    .on_toggle(SettingsMessage::AutoValidateToggled)
                    .style(theme::toggler_style),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
        ]
        .spacing(12),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    // About Card
    let about_card = container(
        column![
            row![
                icons::icon_info(mocha::LAVENDER, 18.0),
                text(lang.about_heading())
                    .size(16)
                    .color(mocha::TEXT),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(lang.about_desc())
                .size(13)
                .color(mocha::SUBTEXT0),
            column![
                row![
                    text(match lang {
                        Language::English => "Version:",
                        Language::Russian => "Версия:",
                    })
                    .size(13)
                    .color(mocha::SUBTEXT0)
                    .width(Length::Fixed(120.0)),
                    text(lang.about_version())
                        .size(13)
                        .color(mocha::MAUVE),
                ]
                .spacing(10),
                row![
                    text(match lang {
                        Language::English => "Engine:",
                        Language::Russian => "Движок:",
                    })
                    .size(13)
                    .color(mocha::SUBTEXT0)
                    .width(Length::Fixed(120.0)),
                    text(lang.about_tech())
                        .size(13)
                        .color(mocha::LAVENDER),
                ]
                .spacing(10),
                row![
                    text(match lang {
                        Language::English => "Source code:",
                        Language::Russian => "Исходный код:",
                    })
                    .size(13)
                    .color(mocha::SUBTEXT0)
                    .width(Length::Fixed(120.0)),
                    text(lang.about_github())
                        .size(13)
                        .color(mocha::BLUE),
                ]
                .spacing(10),
            ]
            .spacing(6),
        ]
        .spacing(12),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    column![header, lang_card, config_card, validation_card, about_card]
        .spacing(18)
        .into()
}
