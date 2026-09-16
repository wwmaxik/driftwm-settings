use crate::config::DriftwmConfig;
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

pub fn view(config: &DriftwmConfig) -> Element<'static, GeneralMessage> {
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
    let placement_desc = match current_placement.as_str() {
        "auto" => "Auto: Snap-place adjacent to an existing window cluster in view (falls back to center).",
        "cursor" => "Cursor: Spawn centered on the mouse pointer; camera stays stationary.",
        _ => "Center: Spawn at the focus placement point in the viewport; camera animates to new window.",
    };

    let card_placement = container(
        column![
            text("Window Placement & Modifiers")
                .size(18)
                .color(mocha::MAUVE),
            text("Controls how new windows spawn and which modifier key is used for window manager shortcuts.")
                .size(13)
                .color(mocha::SUBTEXT0),
            
            row![
                text("Modifier Key (mod_key):").size(14).color(mocha::TEXT).width(Length::Fixed(220.0)),
                pick_list(mod_keys, Some(current_mod_key), GeneralMessage::ModKeyChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text("Window Placement:").size(14).color(mocha::TEXT).width(Length::Fixed(220.0)),
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
                text("Focus Placement:").size(14).color(mocha::TEXT).width(Length::Fixed(220.0)),
                pick_list(focus_options, Some(current_focus), GeneralMessage::FocusPlacementChanged)
                    .style(pick_list_style)
                    .width(Length::Fixed(180.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            checkbox(
                "Focus follows mouse (Sloppy focus: pointer hover focuses window)",
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
            text("Navigation & Camera Reactions")
                .size(18)
                .color(mocha::MAUVE),
            text("Configure camera tracking when closing or activating windows.")
                .size(13)
                .color(mocha::SUBTEXT0),

            checkbox(
                "Auto-navigate on close: Pan to newly focused window when active window closes",
                config.navigation.auto_navigate_on_close.unwrap_or(true)
            )
            .on_toggle(GeneralMessage::AutoNavigateOnCloseToggled)
            .size(16),

            checkbox(
                "Auto-navigate on click: Pan partially off-screen window into view when clicked",
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
            text("Session Persistence")
                .size(18)
                .color(mocha::MAUVE),
            text("Manage state restoration across driftwm restarts (~/.local/state/driftwm/session.json).")
                .size(13)
                .color(mocha::SUBTEXT0),

            checkbox(
                "Suspend on close: Client window closes become suspended windows rather than destroyed",
                config.session.suspend_on_close.unwrap_or(false)
            )
            .on_toggle(GeneralMessage::SuspendOnCloseToggled)
            .size(16),

            checkbox(
                "Restore open windows: Reopen still-open applications as suspended windows on next launch",
                config.session.restore_windows.unwrap_or(false)
            )
            .on_toggle(GeneralMessage::RestoreWindowsToggled)
            .size(16),

            checkbox(
                "Restore camera position: Restore camera viewport coordinates and zoom from saved session",
                config.session.restore_camera.unwrap_or(false)
            )
            .on_toggle(GeneralMessage::RestoreCameraToggled)
            .size(16),

            checkbox(
                "Restore bookmarks: Persist runtime bookmark edits across restarts",
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
        .spacing(18)
        .width(Length::Fill)
        .into()
}
