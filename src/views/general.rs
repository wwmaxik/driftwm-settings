use crate::config::DriftwmConfig;
use crate::i18n::Language;
use crate::icons;
use crate::theme::{card_style, mocha, pick_list_style};
use iced::widget::{checkbox, column, container, pick_list, row, text};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub enum GeneralMessage {
    ModKeyChanged(String),
    FocusFollowsMouseToggled(bool),
    WindowPlacementChanged(String),
    FocusPlacementChanged(String),
    AutoNavigateOnCloseToggled(bool),
    AutoNavigateOnClickToggled(bool),
    SuspendOnCloseToggled(bool),
    RestoreWindowsToggled(bool),
    RestoreCameraToggled(bool),
    RestoreBookmarksToggled(bool),
}

pub fn update(config: &mut DriftwmConfig, msg: GeneralMessage) {
    match msg {
        GeneralMessage::ModKeyChanged(val) => {
            config.mod_key = Some(val);
        }
        GeneralMessage::FocusFollowsMouseToggled(val) => {
            config.focus_follows_mouse = Some(val);
        }
        GeneralMessage::WindowPlacementChanged(val) => {
            config.window_placement = Some(val);
        }
        GeneralMessage::FocusPlacementChanged(val) => {
            config.focus_placement = Some(val);
        }
        GeneralMessage::AutoNavigateOnCloseToggled(val) => {
            config.navigation.auto_navigate_on_close = Some(val);
        }
        GeneralMessage::AutoNavigateOnClickToggled(val) => {
            config.navigation.auto_navigate_on_click = Some(val);
        }
        GeneralMessage::SuspendOnCloseToggled(val) => {
            config.session.suspend_on_close = Some(val);
        }
        GeneralMessage::RestoreWindowsToggled(val) => {
            config.session.restore_windows = Some(val);
        }
        GeneralMessage::RestoreCameraToggled(val) => {
            config.session.restore_camera = Some(val);
        }
        GeneralMessage::RestoreBookmarksToggled(val) => {
            config.session.restore_bookmarks = Some(val);
        }
    }
}

pub fn view(config: &DriftwmConfig, lang: Language) -> Element<'static, GeneralMessage> {
    // Options
    let mod_keys = vec!["super".to_string(), "alt".to_string(), "mod3".to_string()];
    let current_mod_key = config.mod_key.clone().unwrap_or_else(|| "super".to_string());

    let placement_options = vec![
        "center".to_string(),
        "cursor".to_string(),
        "auto".to_string(),
    ];
    let current_placement = config
        .window_placement
        .clone()
        .unwrap_or_else(|| "center".to_string());

    let focus_options = vec![
        "center".to_string(),
        "top".to_string(),
        "bottom".to_string(),
        "left".to_string(),
        "right".to_string(),
        "top-left".to_string(),
        "top-right".to_string(),
        "bottom-left".to_string(),
        "bottom-right".to_string(),
    ];
    let current_focus = config
        .focus_placement
        .clone()
        .unwrap_or_else(|| "center".to_string());

    // Card 1: Placement & Mod Key
    let placement_desc = match (current_placement.as_str(), lang) {
        ("auto", Language::Russian) => "Авто: привязка к существующему кластеру окон в поле зрения (или по центру).",
        ("auto", Language::English) => "Auto: Snap-place adjacent to an existing window cluster in view (falls back to center).",
        ("cursor", Language::Russian) => "Курсор: создание окна прямо под указателем мыши; камера остаётся неподвижной.",
        ("cursor", Language::English) => "Cursor: Spawn centered on the mouse pointer; camera stays stationary.",
        (_, Language::Russian) => "По центру: размещение в точке фокуса; камера плавно центрируется на новом окне.",
        (_, Language::English) => "Center: Spawn at the focus placement point in the viewport; camera animates to new window.",
    };

    let card_placement = container(
        column![
            row![
                icons::icon_general(mocha::MAUVE, 18.0),
                text(match lang {
                    Language::English => "Window Placement & Modifiers",
                    Language::Russian => "Размещение окон и клавиша-модификатор",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Controls how new windows spawn and which modifier key is used for shortcuts.",
                Language::Russian => "Управляет появлением новых окон и клавишей для горячих клавиш оконного менеджера.",
            })
            .size(13)
            .color(mocha::SUBTEXT0),
            
            row![
                text(match lang {
                    Language::English => "Modifier Key (mod_key):",
                    Language::Russian => "Клавиша-модификатор (mod_key):",
                }).size(14).color(mocha::TEXT).width(Length::Fixed(240.0)),
                pick_list(mod_keys, Some(current_mod_key), GeneralMessage::ModKeyChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(match lang {
                    Language::English => "Window Placement:",
                    Language::Russian => "Размещение окон:",
                }).size(14).color(mocha::TEXT).width(Length::Fixed(240.0)),
                pick_list(placement_options, Some(current_placement), GeneralMessage::WindowPlacementChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            container(
                text(placement_desc)
                    .size(12)
                    .color(mocha::LAVENDER)
            )
            .padding(8),

            row![
                text(match lang {
                    Language::English => "Focus Placement:",
                    Language::Russian => "Точка фокусировки:",
                }).size(14).color(mocha::TEXT).width(Length::Fixed(240.0)),
                pick_list(focus_options, Some(current_focus), GeneralMessage::FocusPlacementChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            checkbox(
                match lang {
                    Language::English => "Focus follows mouse (Sloppy focus: pointer hover focuses window)",
                    Language::Russian => "Фокус следует за мышью (наведение курсора активирует окно)",
                },
                config.focus_follows_mouse.unwrap_or(false)
            )
            .on_toggle(GeneralMessage::FocusFollowsMouseToggled)
            .size(16),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    // Card 2: Camera & Navigation
    let card_navigation = container(
        column![
            row![
                icons::icon_bookmarks(mocha::LAVENDER, 18.0),
                text(match lang {
                    Language::English => "Navigation & Camera Reactions",
                    Language::Russian => "Навигация и реакция камеры",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Configure camera tracking when closing or activating windows.",
                Language::Russian => "Настройка слежения камеры при закрытии или активации окон.",
            })
            .size(13)
            .color(mocha::SUBTEXT0),

            checkbox(
                match lang {
                    Language::English => "Auto-navigate on close: Pan to newly focused window when active window closes",
                    Language::Russian => "Автонавигация при закрытии: перемещать камеру к новому фокусному окну",
                },
                config.navigation.auto_navigate_on_close.unwrap_or(true)
            )
            .on_toggle(GeneralMessage::AutoNavigateOnCloseToggled)
            .size(16),

            checkbox(
                match lang {
                    Language::English => "Auto-navigate on click: Pan partially off-screen window into view when clicked",
                    Language::Russian => "Автонавигация по клику: центрировать окно при клике, если оно за границами экрана",
                },
                config.navigation.auto_navigate_on_click.unwrap_or(false)
            )
            .on_toggle(GeneralMessage::AutoNavigateOnClickToggled)
            .size(16),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    // Card 3: Session Persistence
    let card_session = container(
        column![
            row![
                icons::icon_save(mocha::TEAL, 18.0),
                text(match lang {
                    Language::English => "Session State Persistence",
                    Language::Russian => "Сохранение состояния сессии",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Control what driftwm remembers between restarts and exits.",
                Language::Russian => "Управление тем, что сохраняет driftwm между перезапусками.",
            })
            .size(13)
            .color(mocha::SUBTEXT0),

            checkbox(
                match lang {
                    Language::English => "Suspend on close: Keep windows running when closed (driftwm process management)",
                    Language::Russian => "Приостанавливать при закрытии: сохранять процессы окон в фоне",
                },
                config.session.suspend_on_close.unwrap_or(false)
            )
            .on_toggle(GeneralMessage::SuspendOnCloseToggled)
            .size(16),

            checkbox(
                match lang {
                    Language::English => "Restore windows: Reopen applications and position them where you left them",
                    Language::Russian => "Восстанавливать окна: открывать приложения на прежних координатах",
                },
                config.session.restore_windows.unwrap_or(false)
            )
            .on_toggle(GeneralMessage::RestoreWindowsToggled)
            .size(16),

            checkbox(
                match lang {
                    Language::English => "Restore camera: Return to the last canvas coordinates on startup",
                    Language::Russian => "Восстанавливать камеру: возвращаться к последней позиции холста при запуске",
                },
                config.session.restore_camera.unwrap_or(false)
            )
            .on_toggle(GeneralMessage::RestoreCameraToggled)
            .size(16),

            checkbox(
                match lang {
                    Language::English => "Restore bookmarks: Preserve runtime bookmark coordinates across sessions",
                    Language::Russian => "Восстанавливать закладки: сохранять динамические закладки между сессиями",
                },
                config.session.restore_bookmarks.unwrap_or(false)
            )
            .on_toggle(GeneralMessage::RestoreBookmarksToggled)
            .size(16),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    column![card_placement, card_navigation, card_session]
        .spacing(20)
        .into()
}
