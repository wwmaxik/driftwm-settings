use crate::config::DriftwmConfig;
use crate::theme::{
    card_style, danger_button_style, input_style, mocha, primary_button_style,
    secondary_button_style, slider_style,
};
use iced::widget::{button, checkbox, column, container, row, slider, text, text_input};
use iced::{Alignment, Element, Length};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum BookmarksMessage {
    // New bookmark draft inputs
    NewBookmarkKeyChanged(String),
    NewBookmarkXChanged(String),
    NewBookmarkYChanged(String),
    AddBookmark,
    DeleteBookmark(String),
    ResetDefaultBookmarks,

    // New anchor draft inputs
    NewAnchorXChanged(String),
    NewAnchorYChanged(String),
    AddAnchor,
    DeleteAnchor(usize),

    // Navigation Dynamics
    CameraSpeedChanged(f32),
    DriftChanged(f32),
    PanStepChanged(f32),
    NudgeStepChanged(f32),
    ResizeStepChanged(f32),
    TrackpadSpeedChanged(f32),
    MouseSpeedChanged(f32),
    TouchSpeedChanged(f32),

    // Edge Pan
    EdgePanZoneChanged(f32),
    EdgePanSpeedMinChanged(f32),
    EdgePanSpeedMaxChanged(f32),
    CursorPanToggled(bool),
    CursorZoneChanged(f32),
    LatencyMsChanged(f32),

    // Zoom
    ZoomStepChanged(f32),
    FitPaddingChanged(f32),
    ResetOnNewWindowToggled(bool),
    ResetOnActivationToggled(bool),
    InteractMinChanged(f32),
}

#[derive(Debug, Clone, Default)]
pub struct BookmarksState {
    pub new_bm_key: String,
    pub new_bm_x: String,
    pub new_bm_y: String,
    pub new_anchor_x: String,
    pub new_anchor_y: String,
}

pub fn update(
    config: &mut DriftwmConfig,
    state: &mut BookmarksState,
    msg: BookmarksMessage,
) {
    match msg {
        BookmarksMessage::NewBookmarkKeyChanged(val) => state.new_bm_key = val,
        BookmarksMessage::NewBookmarkXChanged(val) => state.new_bm_x = val,
        BookmarksMessage::NewBookmarkYChanged(val) => state.new_bm_y = val,
        BookmarksMessage::AddBookmark => {
            let key = state.new_bm_key.trim().to_string();
            if !key.is_empty() {
                let x = state.new_bm_x.trim().parse::<f64>().unwrap_or(0.0);
                let y = state.new_bm_y.trim().parse::<f64>().unwrap_or(0.0);
                let map = config.navigation.bookmarks.get_or_insert_with(HashMap::new);
                map.insert(key, [x, y]);
                state.new_bm_key.clear();
                state.new_bm_x.clear();
                state.new_bm_y.clear();
            }
        }
        BookmarksMessage::DeleteBookmark(key) => {
            if let Some(map) = &mut config.navigation.bookmarks {
                map.remove(&key);
            }
        }
        BookmarksMessage::ResetDefaultBookmarks => {
            config.navigation.bookmarks = Some(HashMap::from([
                ("1".to_string(), [-1750.0, 1750.0]),
                ("2".to_string(), [1750.0, 1750.0]),
                ("3".to_string(), [1750.0, -1750.0]),
                ("4".to_string(), [-1750.0, -1750.0]),
            ]));
        }
        BookmarksMessage::NewAnchorXChanged(val) => state.new_anchor_x = val,
        BookmarksMessage::NewAnchorYChanged(val) => state.new_anchor_y = val,
        BookmarksMessage::AddAnchor => {
            let x = state.new_anchor_x.trim().parse::<f64>().unwrap_or(0.0);
            let y = state.new_anchor_y.trim().parse::<f64>().unwrap_or(0.0);
            let list = config.navigation.anchors.get_or_insert_with(Vec::new);
            list.push([x, y]);
            state.new_anchor_x.clear();
            state.new_anchor_y.clear();
        }
        BookmarksMessage::DeleteAnchor(idx) => {
            if let Some(list) = &mut config.navigation.anchors
                && idx < list.len()
            {
                list.remove(idx);
            }
        }
        BookmarksMessage::CameraSpeedChanged(val) => {
            config.navigation.camera_speed = Some(val as f64);
        }
        BookmarksMessage::DriftChanged(val) => {
            config.navigation.drift = Some(val as f64);
        }
        BookmarksMessage::PanStepChanged(val) => {
            config.navigation.pan_step = Some(val as f64);
        }
        BookmarksMessage::NudgeStepChanged(val) => {
            config.navigation.nudge_step = Some(val.round() as i32);
        }
        BookmarksMessage::ResizeStepChanged(val) => {
            config.navigation.resize_step = Some(val.round() as i32);
        }
        BookmarksMessage::TrackpadSpeedChanged(val) => {
            config.navigation.trackpad_speed = Some(val as f64);
        }
        BookmarksMessage::MouseSpeedChanged(val) => {
            config.navigation.mouse_speed = Some(val as f64);
        }
        BookmarksMessage::TouchSpeedChanged(val) => {
            config.navigation.touch_speed = Some(val as f64);
        }
        BookmarksMessage::EdgePanZoneChanged(val) => {
            config.navigation.edge_pan.zone = Some(val as f64);
        }
        BookmarksMessage::EdgePanSpeedMinChanged(val) => {
            config.navigation.edge_pan.speed_min = Some(val as f64);
        }
        BookmarksMessage::EdgePanSpeedMaxChanged(val) => {
            config.navigation.edge_pan.speed_max = Some(val as f64);
        }
        BookmarksMessage::CursorPanToggled(val) => {
            config.navigation.edge_pan.cursor_pan = Some(val);
        }
        BookmarksMessage::CursorZoneChanged(val) => {
            config.navigation.edge_pan.cursor_zone = Some(val as f64);
        }
        BookmarksMessage::LatencyMsChanged(val) => {
            config.navigation.edge_pan.latency_ms = Some(val.round() as u64);
        }
        BookmarksMessage::ZoomStepChanged(val) => {
            config.zoom.step = Some(val as f64);
        }
        BookmarksMessage::FitPaddingChanged(val) => {
            config.zoom.fit_padding = Some(val as f64);
        }
        BookmarksMessage::ResetOnNewWindowToggled(val) => {
            config.zoom.reset_on_new_window = Some(val);
        }
        BookmarksMessage::ResetOnActivationToggled(val) => {
            config.zoom.reset_on_activation = Some(val);
        }
        BookmarksMessage::InteractMinChanged(val) => {
            config.zoom.interact_min = Some(val as f64);
        }
    }
}

pub fn view<'a>(
    config: &'a DriftwmConfig,
    state: &'a BookmarksState,
) -> Element<'a, BookmarksMessage> {
    // Card 1: Bookmarks table
    let mut bm_col = column![
        row![
            column![
                text("Canvas Bookmarks")
                    .size(18)
                    .color(mocha::MAUVE),
                text("Named coordinate points [x, y] for quick jump shortcuts (Mod+1..4) and IPC.")
                    .size(13)
                    .color(mocha::SUBTEXT0),
            ],
            button("Reset to Default 4 Corners")
                .on_press(BookmarksMessage::ResetDefaultBookmarks)
                .style(secondary_button_style),
        ]
        .spacing(12)
        .align_y(Alignment::Center),
    ]
    .spacing(12);

    if let Some(bms) = &config.navigation.bookmarks {
        let mut sorted: Vec<(&String, &[f64; 2])> = bms.iter().collect();
        sorted.sort_by_key(|(k, _)| (*k).clone());

        for (name, coords) in sorted {
            let n = name.clone();
            let row_item = container(
                row![
                    text(format!("Bookmark \"{}\"", name))
                        .size(14)
                        .color(mocha::TEXT)
                        .width(Length::Fixed(180.0)),
                    text(format!("X: {:.0},  Y: {:.0}", coords[0], coords[1]))
                        .size(14)
                        .color(mocha::LAVENDER)
                        .width(Length::Fill),
                    button(text("Delete").size(12))
                        .on_press(BookmarksMessage::DeleteBookmark(n))
                        .style(danger_button_style),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .padding(8);
            bm_col = bm_col.push(row_item);
        }
    }

    // Add bookmark row
    let add_bm_row = row![
        text_input("Name (e.g. 5 or work)", &state.new_bm_key)
            .on_input(BookmarksMessage::NewBookmarkKeyChanged)
            .style(input_style)
            .width(Length::Fixed(180.0)),
        text_input("X coord", &state.new_bm_x)
            .on_input(BookmarksMessage::NewBookmarkXChanged)
            .style(input_style)
            .width(Length::Fixed(110.0)),
        text_input("Y coord", &state.new_bm_y)
            .on_input(BookmarksMessage::NewBookmarkYChanged)
            .style(input_style)
            .width(Length::Fixed(110.0)),
        button(text("+ Add Bookmark").size(13))
            .on_press(BookmarksMessage::AddBookmark)
            .style(primary_button_style),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    bm_col = bm_col.push(add_bm_row);

    let card_bookmarks = container(bm_col)
        .padding(20)
        .style(card_style)
        .width(Length::Fill);

    // Card 2: Anchors
    let mut anchors_col = column![
        text("Canvas Anchors")
            .size(18)
            .color(mocha::MAUVE),
        text("Unoccupied canvas coordinates discoverable by center-nearest actions (Mod+Arrow / 4-finger swipe).")
            .size(13)
            .color(mocha::SUBTEXT0),
    ]
    .spacing(12);

    if let Some(anchors) = &config.navigation.anchors {
        for (i, pt) in anchors.iter().enumerate() {
            let row_item = container(
                row![
                    text(format!("Anchor #{}:", i + 1))
                        .size(14)
                        .color(mocha::TEXT)
                        .width(Length::Fixed(140.0)),
                    text(format!("X: {:.0},  Y: {:.0}", pt[0], pt[1]))
                        .size(14)
                        .color(mocha::LAVENDER)
                        .width(Length::Fill),
                    button(text("Delete").size(12))
                        .on_press(BookmarksMessage::DeleteAnchor(i))
                        .style(danger_button_style),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .padding(8);
            anchors_col = anchors_col.push(row_item);
        }
    }

    let add_anchor_row = row![
        text_input("X coordinate", &state.new_anchor_x)
            .on_input(BookmarksMessage::NewAnchorXChanged)
            .style(input_style)
            .width(Length::Fixed(150.0)),
        text_input("Y coordinate", &state.new_anchor_y)
            .on_input(BookmarksMessage::NewAnchorYChanged)
            .style(input_style)
            .width(Length::Fixed(150.0)),
        button(text("+ Add Anchor").size(13))
            .on_press(BookmarksMessage::AddAnchor)
            .style(primary_button_style),
    ]
    .spacing(10)
    .align_y(Alignment::Center);

    anchors_col = anchors_col.push(add_anchor_row);

    let card_anchors = container(anchors_col)
        .padding(20)
        .style(card_style)
        .width(Length::Fill);

    // Card 3: Camera Dynamics
    let cam_spd = config.navigation.camera_speed.unwrap_or(0.3) as f32;
    let drift_val = config.navigation.drift.unwrap_or(0.5) as f32;
    let pan_step = config.navigation.pan_step.unwrap_or(100.0) as f32;
    let nudge_step = config.navigation.nudge_step.unwrap_or(20) as f32;
    let resize_step = config.navigation.resize_step.unwrap_or(20) as f32;
    let trackpad_spd = config.navigation.trackpad_speed.unwrap_or(1.5) as f32;
    let mouse_spd = config.navigation.mouse_speed.unwrap_or(1.0) as f32;
    let touch_spd = config.navigation.touch_speed.unwrap_or(1.0) as f32;

    let card_dynamics = container(
        column![
            text("Camera & Pan Dynamics")
                .size(18)
                .color(mocha::MAUVE),
            text("Smoothness, coasting momentum and step multipliers for viewport motion.")
                .size(13)
                .color(mocha::SUBTEXT0),

            row![
                text(format!("Camera Speed: {:.2}", cam_spd)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.05..=1.0, cam_spd, BookmarksMessage::CameraSpeedChanged)
                    .step(0.05_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Lerp factor (1 = instant pan)").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Drift Momentum: {:.2}", drift_val)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.0..=1.0, drift_val, BookmarksMessage::DriftChanged)
                    .step(0.05_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Coasting friction (0 = off, 1 = floatiest)").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Pan Step: {:.0} px", pan_step)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(20.0..=400.0, pan_step, BookmarksMessage::PanStepChanged)
                    .step(10.0_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Mod+Ctrl+Arrow viewport pan step").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Nudge Step: {:.0} px", nudge_step)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(5.0..=100.0, nudge_step, BookmarksMessage::NudgeStepChanged)
                    .step(5.0_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Mod+Shift+Arrow window nudge step").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Resize Step: {:.0} px", resize_step)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(5.0..=100.0, resize_step, BookmarksMessage::ResizeStepChanged)
                    .step(5.0_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Grow/shrink keyboard step").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Trackpad Pan Speed: {:.2}", trackpad_spd)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.1..=3.0, trackpad_spd, BookmarksMessage::TrackpadSpeedChanged)
                    .step(0.1_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Mouse Drag Speed: {:.2}", mouse_spd)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.1..=3.0, mouse_spd, BookmarksMessage::MouseSpeedChanged)
                    .step(0.1_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Touch Pan Speed: {:.2}", touch_spd)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.1..=3.0, touch_spd, BookmarksMessage::TouchSpeedChanged)
                    .step(0.1_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    // Card 4: Edge Pan & Zoom
    let ep_zone = config.navigation.edge_pan.zone.unwrap_or(100.0) as f32;
    let ep_min = config.navigation.edge_pan.speed_min.unwrap_or(4.0) as f32;
    let ep_max = config.navigation.edge_pan.speed_max.unwrap_or(10.0) as f32;
    let cursor_zone = config.navigation.edge_pan.cursor_zone.unwrap_or(20.0) as f32;
    let latency_ms = config.navigation.edge_pan.latency_ms.unwrap_or(120) as f32;
    let z_step = config.zoom.step.unwrap_or(1.1) as f32;
    let z_fit_pad = config.zoom.fit_padding.unwrap_or(80.0) as f32;
    let z_interact = config.zoom.interact_min.unwrap_or(0.0) as f32;

    let card_edge_zoom = container(
        column![
            text("Edge Pan & Zoom Controls")
                .size(18)
                .color(mocha::MAUVE),
            text("Configure viewport edge-push panning thresholds and canvas zoom factors.")
                .size(13)
                .color(mocha::SUBTEXT0),

            row![
                text(format!("Edge Pan Zone: {:.0} px", ep_zone)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(20.0..=200.0, ep_zone, BookmarksMessage::EdgePanZoneChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Edge Speed (Min): {:.1} px/f", ep_min)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(1.0..=20.0, ep_min, BookmarksMessage::EdgePanSpeedMinChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Edge Speed (Max): {:.1} px/f", ep_max)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(1.0..=40.0, ep_max, BookmarksMessage::EdgePanSpeedMaxChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            checkbox("Bare cursor edge pan (pan when cursor touches screen edge)", config.navigation.edge_pan.cursor_pan.unwrap_or(false))
                .on_toggle(BookmarksMessage::CursorPanToggled)
                .size(16),

            row![
                text(format!("Cursor Zone: {:.0} px", cursor_zone)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(5.0..=50.0, cursor_zone, BookmarksMessage::CursorZoneChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Edge Latency: {:.0} ms", latency_ms)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.0..=500.0, latency_ms, BookmarksMessage::LatencyMsChanged)
                    .step(10.0_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Delay at edges bordering other displays").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Zoom Step: {:.2}", z_step)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(1.01..=1.5, z_step, BookmarksMessage::ZoomStepChanged)
                    .step(0.01_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Multiplier per zoom keypress").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            row![
                text(format!("Fit Padding: {:.0} px", z_fit_pad)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.0..=200.0, z_fit_pad, BookmarksMessage::FitPaddingChanged)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Padding around windows for zoom-to-fit").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),

            checkbox("Reset zoom to 1.0 when new window maps", config.zoom.reset_on_new_window.unwrap_or(true))
                .on_toggle(BookmarksMessage::ResetOnNewWindowToggled)
                .size(16),

            checkbox("Reset zoom to 1.0 on window focus activation", config.zoom.reset_on_activation.unwrap_or(true))
                .on_toggle(BookmarksMessage::ResetOnActivationToggled)
                .size(16),

            row![
                text(format!("Interact Min Zoom: {:.2}", z_interact)).size(14).color(mocha::TEXT).width(Length::Fixed(200.0)),
                slider(0.0..=1.0, z_interact, BookmarksMessage::InteractMinChanged)
                    .step(0.05_f32)
                    .style(slider_style)
                    .width(Length::Fixed(220.0)),
                text("Below which click navigates instead of touching (0 = off)").size(12).color(mocha::SUBTEXT0),
            ]
            .spacing(12)
            .align_y(Alignment::Center),
        ]
        .spacing(14),
    )
    .padding(20)
    .style(card_style)
    .width(Length::Fill);

    column![card_bookmarks, card_anchors, card_dynamics, card_edge_zoom]
        .spacing(18)
        .width(Length::Fill)
        .into()
}
