use crate::config::*;

pub fn ensure_input_keyboard(cfg: &mut DriftwmConfig) {
    if cfg.input.is_none() {
        cfg.input = Some(InputConfig::default());
    }
    if cfg.input.as_mut().unwrap().keyboard.is_none() {
        cfg.input.as_mut().unwrap().keyboard = Some(KeyboardConfig::default());
    }
}

pub fn ensure_input_trackpad(cfg: &mut DriftwmConfig) {
    if cfg.input.is_none() {
        cfg.input = Some(InputConfig::default());
    }
    if cfg.input.as_mut().unwrap().trackpad.is_none() {
        cfg.input.as_mut().unwrap().trackpad = Some(TrackpadConfig::default());
    }
}

pub fn ensure_input_mouse(cfg: &mut DriftwmConfig) {
    if cfg.input.is_none() {
        cfg.input = Some(InputConfig::default());
    }
    if cfg.input.as_mut().unwrap().mouse.is_none() {
        cfg.input.as_mut().unwrap().mouse = Some(MouseConfig::default());
    }
}

pub fn ensure_navigation(cfg: &mut DriftwmConfig) {
    if cfg.navigation.is_none() {
        cfg.navigation = Some(NavigationConfig::default());
    }
}
