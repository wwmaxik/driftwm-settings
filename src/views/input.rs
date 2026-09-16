use crate::config::DriftwmConfig;
use crate::theme::{card_style, input_style, mocha, pick_list_style, slider_style};
use iced::widget::{checkbox, column, container, pick_list, row, slider, text, text_input};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub enum InputMessage {
    // Keyboard
    LayoutChanged(String),
    VariantChanged(String),
    OptionsChanged(String),
    ModelChanged(String),
    RepeatRateChanged(f32),
    RepeatDelayChanged(f32),
    LayoutIndependentToggled(bool),
    NumLockToggled(bool),
    CapsLockToggled(bool),
    RememberLayoutToggled(bool),

    // Trackpad
    TapToClickToggled(bool),
    NaturalScrollTrackpadToggled(bool),
    TapAndDragToggled(bool),
    TrackpadAccelSpeedChanged(f32),
    TrackpadAccelProfileChanged(String),
    TrackpadClickMethodChanged(String),
    DisableWhileTypingToggled(bool),
    TrackpadEnableToggled(bool),
    DisableOnExternalMouseToggled(bool),

    // Mouse
    MouseAccelSpeedChanged(f32),
    MouseAccelProfileChanged(String),
    NaturalScrollMouseToggled(bool),
    LeftHandedToggled(bool),
}

pub fn update(config: &mut DriftwmConfig, msg: InputMessage) {
    match msg {
        InputMessage::LayoutChanged(val) => {
            config.input.keyboard.layout = Some(val);
        }
        InputMessage::VariantChanged(val) => {
            config.input.keyboard.variant = Some(val);
        }
        InputMessage::OptionsChanged(val) => {
            config.input.keyboard.options = Some(val);
        }
        InputMessage::ModelChanged(val) => {
            config.input.keyboard.model = Some(val);
        }
        InputMessage::RepeatRateChanged(val) => {
            config.input.keyboard.repeat_rate = Some(val.round() as i32);
        }
        InputMessage::RepeatDelayChanged(val) => {
            config.input.keyboard.repeat_delay = Some(val.round() as i32);
        }
        InputMessage::LayoutIndependentToggled(val) => {
            config.input.keyboard.layout_independent = Some(val);
        }
        InputMessage::NumLockToggled(val) => {
            config.input.keyboard.num_lock = Some(val);
        }
        InputMessage::CapsLockToggled(val) => {
            config.input.keyboard.caps_lock = Some(val);
        }
        InputMessage::RememberLayoutToggled(val) => {
            config.input.keyboard.remember_layout_per_window = Some(val);
        }

        InputMessage::TapToClickToggled(val) => {
            config.input.trackpad.tap_to_click = Some(val);
        }
        InputMessage::NaturalScrollTrackpadToggled(val) => {
            config.input.trackpad.natural_scroll = Some(val);
        }
        InputMessage::TapAndDragToggled(val) => {
            config.input.trackpad.tap_and_drag = Some(val);
        }
        InputMessage::TrackpadAccelSpeedChanged(val) => {
            config.input.trackpad.accel_speed = Some(val as f64);
        }
        InputMessage::TrackpadAccelProfileChanged(val) => {
            config.input.trackpad.accel_profile = Some(val);
        }
        InputMessage::TrackpadClickMethodChanged(val) => {
            config.input.trackpad.click_method = Some(val);
        }
        InputMessage::DisableWhileTypingToggled(val) => {
            config.input.trackpad.disable_while_typing = Some(val);
        }
        InputMessage::TrackpadEnableToggled(val) => {
            config.input.trackpad.enable = Some(val);
        }
        InputMessage::DisableOnExternalMouseToggled(val) => {
            config.input.trackpad.disable_on_external_mouse = Some(val);
        }

        InputMessage::MouseAccelSpeedChanged(val) => {
            config.input.mouse.accel_speed = Some(val as f64);
        }
        InputMessage::MouseAccelProfileChanged(val) => {
            config.input.mouse.accel_profile = Some(val);
        }
        InputMessage::NaturalScrollMouseToggled(val) => {
            config.input.mouse.natural_scroll = Some(val);
        }
        InputMessage::LeftHandedToggled(val) => {
            config.input.mouse.left_handed = Some(val);
        }
    }
}

use crate::i18n::Language;
use crate::icons;

pub fn view(config: &DriftwmConfig, lang: Language) -> Element<'static, InputMessage> {
    // Keyboard card
    let layout_val = config.input.keyboard.layout.clone().unwrap_or_else(|| "us".to_string());
    let variant_val = config.input.keyboard.variant.clone().unwrap_or_default();
    let options_val = config.input.keyboard.options.clone().unwrap_or_default();
    let model_val = config.input.keyboard.model.clone().unwrap_or_default();
    let rep_rate = config.input.keyboard.repeat_rate.unwrap_or(25) as f32;
    let rep_del = config.input.keyboard.repeat_delay.unwrap_or(600) as f32;

    let card_keyboard = container(
        column![
            row![
                icons::icon_input(mocha::MAUVE, 18.0),
                text(match lang {
                    Language::English => "Keyboard Configuration",
                    Language::Russian => "Настройки клавиатуры",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "XKB layout, key repeat timings and modifier preferences.",
                Language::Russian => "Раскладка XKB, задержка автоповтора клавиш и параметры модификаторов.",
            })
            .size(13)
            .color(mocha::SUBTEXT0),

            row![
                text("Layout (e.g. us, ru):").size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                text_input("us", &layout_val)
                    .on_input(InputMessage::LayoutChanged)
                    .style(input_style)
                    .width(Length::Fixed(200.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text("Variant:").size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                text_input("dvorak", &variant_val)
                    .on_input(InputMessage::VariantChanged)
                    .style(input_style)
                    .width(Length::Fixed(200.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text("XKB Options:").size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                text_input("grp:win_space_toggle", &options_val)
                    .on_input(InputMessage::OptionsChanged)
                    .style(input_style)
                    .width(Length::Fixed(280.0)),
                text("e.g. grp:win_space_toggle").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text("Hardware Model:").size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                text_input("pc105", &model_val)
                    .on_input(InputMessage::ModelChanged)
                    .style(input_style)
                    .width(Length::Fixed(200.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Repeat Rate: {:.0} keys/s", rep_rate)).size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                slider(5.0..=100.0, rep_rate, InputMessage::RepeatRateChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Repeat Delay: {:.0} ms", rep_del)).size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                slider(100.0..=1000.0, rep_del, InputMessage::RepeatDelayChanged)
                    .step(10.0_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            checkbox("Layout independent bindings (match by physical key position across layouts)", config.input.keyboard.layout_independent.unwrap_or(true))
                .on_toggle(InputMessage::LayoutIndependentToggled)
                .size(16),

            checkbox("Num Lock on startup", config.input.keyboard.num_lock.unwrap_or(true))
                .on_toggle(InputMessage::NumLockToggled)
                .size(16),

            checkbox("Caps Lock on startup", config.input.keyboard.caps_lock.unwrap_or(false))
                .on_toggle(InputMessage::CapsLockToggled)
                .size(16),

            checkbox("Remember keyboard layout per window", config.input.keyboard.remember_layout_per_window.unwrap_or(false))
                .on_toggle(InputMessage::RememberLayoutToggled)
                .size(16),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    // Trackpad card
    let pad_accel = config.input.trackpad.accel_speed.unwrap_or(0.0) as f32;
    let pad_prof = config.input.trackpad.accel_profile.clone().unwrap_or_else(|| "adaptive".to_string());
    let pad_click = config.input.trackpad.click_method.clone().unwrap_or_else(|| "none".to_string());

    let prof_options = vec!["adaptive".to_string(), "flat".to_string()];
    let click_options = vec!["none".to_string(), "clickfinger".to_string(), "button_areas".to_string()];

    let card_trackpad = container(
        column![
            row![
                icons::icon_general(mocha::TEAL, 18.0),
                text(match lang {
                    Language::English => "Trackpad Configuration",
                    Language::Russian => "Настройки тачпада",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Pointer acceleration, tap behaviors and click method for trackpads.",
                Language::Russian => "Ускорение курсора, жесты касания и метод клика для тачпадов.",
            })
            .size(13)
            .color(mocha::SUBTEXT0),

            checkbox(
                match lang {
                    Language::English => "Enable trackpad hardware",
                    Language::Russian => "Включить тачпад",
                },
                config.input.trackpad.enable.unwrap_or(true)
            )
            .on_toggle(InputMessage::TrackpadEnableToggled)
            .size(16),

            checkbox(
                match lang {
                    Language::English => "Tap to click",
                    Language::Russian => "Клик касанием (Tap to click)",
                },
                config.input.trackpad.tap_to_click.unwrap_or(true)
            )
            .on_toggle(InputMessage::TapToClickToggled)
            .size(16),

            checkbox(
                match lang {
                    Language::English => "Natural scrolling (reverse scroll direction)",
                    Language::Russian => "Естественная прокрутка (реверс направления)",
                },
                config.input.trackpad.natural_scroll.unwrap_or(true)
            )
            .on_toggle(InputMessage::NaturalScrollTrackpadToggled)
            .size(16),

            checkbox(
                match lang {
                    Language::English => "Tap and drag (double-tap-hold to drag)",
                    Language::Russian => "Перетаскивание касанием (двойное касание с удержанием)",
                },
                config.input.trackpad.tap_and_drag.unwrap_or(true)
            )
            .on_toggle(InputMessage::TapAndDragToggled)
            .size(16),

            row![
                text(format!("Pointer Accel Speed: {:.2}", pad_accel)).size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                slider(-1.0..=1.0, pad_accel, InputMessage::TrackpadAccelSpeedChanged)
                    .step(0.05_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text("Acceleration Profile:").size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                pick_list(prof_options.clone(), Some(pad_prof), InputMessage::TrackpadAccelProfileChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text("Click Method:").size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                pick_list(click_options, Some(pad_click), InputMessage::TrackpadClickMethodChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            checkbox(
                match lang {
                    Language::English => "Disable while typing (palm rejection)",
                    Language::Russian => "Отключать при наборе текста (защита от ладони)",
                },
                config.input.trackpad.disable_while_typing.unwrap_or(true)
            )
            .on_toggle(InputMessage::DisableWhileTypingToggled)
            .size(16),

            checkbox(
                match lang {
                    Language::English => "Disable trackpad when external mouse is connected",
                    Language::Russian => "Отключать тачпад при подключении внешней мыши",
                },
                config.input.trackpad.disable_on_external_mouse.unwrap_or(false)
            )
            .on_toggle(InputMessage::DisableOnExternalMouseToggled)
            .size(16),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    // Mouse card
    let mouse_accel = config.input.mouse.accel_speed.unwrap_or(0.0) as f32;
    let mouse_prof = config.input.mouse.accel_profile.clone().unwrap_or_else(|| "flat".to_string());

    let card_mouse = container(
        column![
            row![
                icons::icon_settings(mocha::BLUE, 18.0),
                text(match lang {
                    Language::English => "Mouse Configuration",
                    Language::Russian => "Настройки мыши",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Pointer settings for external mice.",
                Language::Russian => "Параметры курсора и кнопок для внешних мышей.",
            })
            .size(13)
            .color(mocha::SUBTEXT0),

            row![
                text(format!("Pointer Accel Speed: {:.2}", mouse_accel)).size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                slider(-1.0..=1.0, mouse_accel, InputMessage::MouseAccelSpeedChanged)
                    .step(0.05_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text("Acceleration Profile:").size(14).color(mocha::TEXT).width(Length::Fixed(180.0)),
                pick_list(prof_options, Some(mouse_prof), InputMessage::MouseAccelProfileChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            checkbox(
                match lang {
                    Language::English => "Natural scrolling (reverse scroll direction)",
                    Language::Russian => "Естественная прокрутка (реверс направления)",
                },
                config.input.mouse.natural_scroll.unwrap_or(false)
            )
            .on_toggle(InputMessage::NaturalScrollMouseToggled)
            .size(16),

            checkbox(
                match lang {
                    Language::English => "Left-handed mode (swap left and right buttons)",
                    Language::Russian => "Режим для левши (поменять местами левую и правую кнопки)",
                },
                config.input.mouse.left_handed.unwrap_or(false)
            )
            .on_toggle(InputMessage::LeftHandedToggled)
            .size(16),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    column![card_keyboard, card_trackpad, card_mouse]
        .spacing(18)
        .width(Length::Fill)
        .into()
}
