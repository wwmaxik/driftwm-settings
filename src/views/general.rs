use crate::config::DriftwmConfig;
use crate::i18n::Language;
use crate::icons;
use crate::theme::{card_style, mocha, pick_list_style, primary_button_style, secondary_button_style, text_input_style};
use iced::widget::{button, checkbox, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone, Default)]
pub struct GeneralState {
    pub new_autostart_cmd: String,
}

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
    NewAutostartChanged(String),
    AddAutostart,
    RemoveAutostart(usize),
    XwaylandToggled(bool),
    XwaylandPathChanged(String),
    WaitForFrameToggled(bool),
    DisableDirectScanoutToggled(bool),
    DisableHardwareCursorToggled(bool),
    MaxCaptureFpsChanged(String),
}

pub fn update(config: &mut DriftwmConfig, state: &mut GeneralState, msg: GeneralMessage) {
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
        GeneralMessage::NewAutostartChanged(cmd) => {
            state.new_autostart_cmd = cmd;
        }
        GeneralMessage::AddAutostart => {
            let cmd = state.new_autostart_cmd.trim().to_string();
            if !cmd.is_empty() {
                let list = config.autostart.get_or_insert_with(Vec::new);
                list.push(cmd);
                state.new_autostart_cmd.clear();
            }
        }
        GeneralMessage::RemoveAutostart(idx) => {
            if let Some(list) = &mut config.autostart {
                if idx < list.len() {
                    list.remove(idx);
                }
            }
        }
        GeneralMessage::XwaylandToggled(val) => {
            config.xwayland.enabled = Some(val);
        }
        GeneralMessage::XwaylandPathChanged(val) => {
            config.xwayland.path = if val.is_empty() { None } else { Some(val) };
        }
        GeneralMessage::WaitForFrameToggled(val) => {
            config.backend.wait_for_frame_completion = Some(val);
        }
        GeneralMessage::DisableDirectScanoutToggled(val) => {
            config.backend.disable_direct_scanout = Some(val);
        }
        GeneralMessage::DisableHardwareCursorToggled(val) => {
            config.backend.disable_hardware_cursor = Some(val);
        }
        GeneralMessage::MaxCaptureFpsChanged(val) => {
            config.backend.max_capture_fps = val.parse::<u32>().ok();
        }
    }
}

pub fn view<'a>(
    config: &'a DriftwmConfig,
    state: &'a GeneralState,
    lang: Language,
) -> Element<'a, GeneralMessage> {
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
                    Language::English => "Focus Centering Point:",
                    Language::Russian => "Точка парковки окна:",
                }).size(14).color(mocha::TEXT).width(Length::Fixed(240.0)),
                pick_list(focus_options, Some(current_focus), GeneralMessage::FocusPlacementChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            checkbox(
                match lang {
                    Language::English => "Focus follows mouse (sloppy focus: hover focuses window, canvas click unfocuses)",
                    Language::Russian => "Фокус за курсором мыши (наведение фокусирует окно, клик по холсту снимает)",
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

    // Card 2: Navigation & Camera Behavior
    let card_navigation = container(
        column![
            row![
                icons::icon_bookmarks(mocha::BLUE, 18.0),
                text(match lang {
                    Language::English => "Camera & Navigation Behavior",
                    Language::Russian => "Поведение камеры и навигации",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Automated camera tracking when closing or interacting with windows.",
                Language::Russian => "Автоматическое слежение камеры при закрытии или кликах по окнам.",
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

    // Card 3: Session State Persistence
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

    // Card 4: Autostart Applications
    let autostart_list = config.autostart.as_deref().unwrap_or(&[]);

    let mut autostart_items = column![].spacing(6);
    for (idx, cmd) in autostart_list.iter().enumerate() {
        let cmd_owned = cmd.clone();
        autostart_items = autostart_items.push(
            row![
                text(format!("{}.", idx + 1)).size(13).color(mocha::SUBTEXT0).width(Length::Fixed(24.0)),
                text(cmd_owned).size(13).color(mocha::TEXT).width(Length::Fill),
                button(icons::icon_trash(mocha::RED, 12.0))
                    .on_press(GeneralMessage::RemoveAutostart(idx))
                    .style(secondary_button_style)
                    .padding(4),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        );
    }

    let card_autostart = container(
        column![
            row![
                icons::icon_reload(mocha::GREEN, 18.0),
                text(match lang {
                    Language::English => "Autostart Applications",
                    Language::Russian => "Автозапуск приложений",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Commands executed on compositor startup via sh -c (e.g., waybar, swaync, mako).",
                Language::Russian => "Команды, запускаемые при старте driftwm (например, waybar, swaync, mako).",
            })
            .size(13)
            .color(mocha::SUBTEXT0),

            row![
                text_input("e.g. waybar, swaync", &state.new_autostart_cmd)
                    .on_input(GeneralMessage::NewAutostartChanged)
                    .style(text_input_style)
                    .padding(8)
                    .width(Length::Fill),
                button(
                    row![
                        icons::icon_plus(mocha::BASE, 14.0),
                        text(match lang {
                            Language::English => "Add Command",
                            Language::Russian => "Добавить",
                        })
                        .size(13),
                    ]
                    .spacing(6)
                    .align_y(Alignment::Center)
                )
                .on_press(GeneralMessage::AddAutostart)
                .style(primary_button_style)
                .padding(8),
            ]
            .spacing(10)
            .align_y(Alignment::Center),

            autostart_items,
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    // Card 5: Xwayland & Backend Stability
    let card_xwayland_backend = container(
        column![
            row![
                icons::icon_general(mocha::PEACH, 18.0),
                text(match lang {
                    Language::English => "Xwayland & Backend Stability",
                    Language::Russian => "Xwayland и стабильность видеодрайвера",
                })
                .size(18)
                .color(mocha::MAUVE),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(match lang {
                Language::English => "Compatibility settings for X11 applications and NVIDIA GPU stability quirks.",
                Language::Russian => "Совместимость с X11-приложениями и опции стабильности драйверов NVIDIA.",
            })
            .size(13)
            .color(mocha::SUBTEXT0),

            row![
                checkbox(
                    match lang {
                        Language::English => "Enable Xwayland satellite",
                        Language::Russian => "Включить Xwayland (поддержка X11)",
                    },
                    config.xwayland.enabled.unwrap_or(true)
                )
                .on_toggle(GeneralMessage::XwaylandToggled)
                .size(16),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(match lang {
                    Language::English => "Xwayland binary path:",
                    Language::Russian => "Путь к бинарнику xwayland-satellite:",
                }).size(13).color(mocha::TEXT).width(Length::Fixed(240.0)),
                text_input("xwayland-satellite", config.xwayland.path.as_deref().unwrap_or("xwayland-satellite"))
                    .on_input(GeneralMessage::XwaylandPathChanged)
                    .style(text_input_style)
                    .padding(6)
                    .width(Length::Fixed(240.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            checkbox(
                match lang {
                    Language::English => "wait_for_frame_completion (Force GPU-fence wait before every page flip — fixes NVIDIA flicker)",
                    Language::Russian => "wait_for_frame_completion (Ожидание GPU-fence перед переключением кадра — лечит мерцания на NVIDIA)",
                },
                config.backend.wait_for_frame_completion.unwrap_or(false)
            )
            .on_toggle(GeneralMessage::WaitForFrameToggled)
            .size(16),

            checkbox(
                match lang {
                    Language::English => "disable_direct_scanout (Force EGL composition — helps with driver crashes)",
                    Language::Russian => "disable_direct_scanout (Принудительная композиция EGL)",
                },
                config.backend.disable_direct_scanout.unwrap_or(false)
            )
            .on_toggle(GeneralMessage::DisableDirectScanoutToggled)
            .size(16),

            checkbox(
                match lang {
                    Language::English => "disable_hardware_cursor (Software cursor fallback for NVIDIA GPU tearing)",
                    Language::Russian => "disable_hardware_cursor (Программный курсор для устранения артефактов NVIDIA)",
                },
                config.backend.disable_hardware_cursor.unwrap_or(false)
            )
            .on_toggle(GeneralMessage::DisableHardwareCursorToggled)
            .size(16),

            row![
                text(match lang {
                    Language::English => "Max capture FPS (limit recording overhead):",
                    Language::Russian => "Лимит FPS записи экрана:",
                }).size(13).color(mocha::TEXT).width(Length::Fixed(240.0)),
                pick_list(
                    vec!["0 (Unlimited)".to_string(), "30".to_string(), "60".to_string(), "120".to_string(), "144".to_string()],
                    Some(match config.backend.max_capture_fps {
                        Some(0) | None => "0 (Unlimited)".to_string(),
                        Some(n) => n.to_string(),
                    }),
                    |val| {
                        let fps = if val.starts_with('0') { "0".to_string() } else { val };
                        GeneralMessage::MaxCaptureFpsChanged(fps)
                    }
                )
                .style(pick_list_style)
                .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    column![card_placement, card_navigation, card_session, card_autostart, card_xwayland_backend]
        .spacing(20)
        .into()
}
