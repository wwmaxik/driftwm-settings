mod config;

use config::DriftwmConfig;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, Entry, Label, Orientation, ScrolledWindow, Switch,
};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

const APP_ID: &str = "com.github.driftwm.settings";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let config_path = get_config_path();
    let config = DriftwmConfig::load(&config_path).unwrap_or_default();
    let config_rc = Rc::new(RefCell::new(config));

    // Scrolled window for content
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);

    // Main container
    let main_box = Box::new(Orientation::Vertical, 12);
    main_box.set_margin_top(12);
    main_box.set_margin_bottom(12);
    main_box.set_margin_start(12);
    main_box.set_margin_end(12);

    // Header
    let header = Label::new(Some("driftwm Settings"));
    header.add_css_class("title-1");
    main_box.append(&header);

    // Config file path
    let path_label = Label::new(Some(&format!("Config: {}", config_path.display())));
    path_label.set_halign(gtk4::Align::Start);
    path_label.add_css_class("dim-label");
    main_box.append(&path_label);

    // Mod key section
    let mod_section = create_section("Modifier Key");
    let mod_box = Box::new(Orientation::Horizontal, 6);
    let mod_label = Label::new(Some("Mod key:"));
    mod_label.set_width_chars(20);
    mod_label.set_xalign(0.0);
    let mod_entry = Entry::new();
    mod_entry.set_text(
        &config_rc
            .borrow()
            .mod_key
            .clone()
            .unwrap_or_else(|| "super".to_string()),
    );
    mod_entry.set_placeholder_text(Some("super or alt"));
    mod_entry.set_hexpand(true);

    let config_clone = config_rc.clone();
    mod_entry.connect_changed(move |entry| {
        config_clone.borrow_mut().mod_key = Some(entry.text().to_string());
    });

    mod_box.append(&mod_label);
    mod_box.append(&mod_entry);
    mod_section.append(&mod_box);
    main_box.append(&mod_section);

    // Focus follows mouse
    let focus_section = create_section("Focus");
    let focus_box = Box::new(Orientation::Horizontal, 6);
    let focus_label = Label::new(Some("Focus follows mouse:"));
    focus_label.set_width_chars(20);
    focus_label.set_xalign(0.0);
    focus_label.set_hexpand(true);
    let focus_switch = Switch::new();
    focus_switch.set_active(config_rc.borrow().focus_follows_mouse.unwrap_or(false));

    let config_clone = config_rc.clone();
    focus_switch.connect_state_set(move |_, state| {
        config_clone.borrow_mut().focus_follows_mouse = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    focus_box.append(&focus_label);
    focus_box.append(&focus_switch);
    focus_section.append(&focus_box);
    main_box.append(&focus_section);

    // Trackpad settings
    let trackpad_section = create_section("Trackpad");

    let tap_box = Box::new(Orientation::Horizontal, 6);
    let tap_label = Label::new(Some("Tap to click:"));
    tap_label.set_width_chars(20);
    tap_label.set_xalign(0.0);
    tap_label.set_hexpand(true);
    let tap_switch = Switch::new();
    tap_switch.set_active(
        config_rc
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.trackpad.as_ref())
            .and_then(|t| t.tap_to_click)
            .unwrap_or(true),
    );

    let config_clone = config_rc.clone();
    tap_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.input.is_none() {
            cfg.input = Some(config::InputConfig::default());
        }
        if cfg.input.as_mut().unwrap().trackpad.is_none() {
            cfg.input.as_mut().unwrap().trackpad = Some(config::TrackpadConfig::default());
        }
        cfg.input
            .as_mut()
            .unwrap()
            .trackpad
            .as_mut()
            .unwrap()
            .tap_to_click = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    tap_box.append(&tap_label);
    tap_box.append(&tap_switch);
    trackpad_section.append(&tap_box);

    let natural_box = Box::new(Orientation::Horizontal, 6);
    let natural_label = Label::new(Some("Natural scroll:"));
    natural_label.set_width_chars(20);
    natural_label.set_xalign(0.0);
    natural_label.set_hexpand(true);
    let natural_switch = Switch::new();
    natural_switch.set_active(
        config_rc
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.trackpad.as_ref())
            .and_then(|t| t.natural_scroll)
            .unwrap_or(true),
    );

    let config_clone = config_rc.clone();
    natural_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.input.is_none() {
            cfg.input = Some(config::InputConfig::default());
        }
        if cfg.input.as_mut().unwrap().trackpad.is_none() {
            cfg.input.as_mut().unwrap().trackpad = Some(config::TrackpadConfig::default());
        }
        cfg.input
            .as_mut()
            .unwrap()
            .trackpad
            .as_mut()
            .unwrap()
            .natural_scroll = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    natural_box.append(&natural_label);
    natural_box.append(&natural_switch);
    trackpad_section.append(&natural_box);

    main_box.append(&trackpad_section);

    // Cursor settings
    let cursor_section = create_section("Cursor");

    let theme_box = Box::new(Orientation::Horizontal, 6);
    let theme_label = Label::new(Some("Cursor theme:"));
    theme_label.set_width_chars(20);
    theme_label.set_xalign(0.0);
    let theme_entry = Entry::new();
    theme_entry.set_text(
        &config_rc
            .borrow()
            .cursor
            .as_ref()
            .and_then(|c| c.theme.clone())
            .unwrap_or_else(|| "Adwaita".to_string()),
    );
    theme_entry.set_hexpand(true);

    let config_clone = config_rc.clone();
    theme_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.cursor.is_none() {
            cfg.cursor = Some(config::CursorConfig::default());
        }
        cfg.cursor.as_mut().unwrap().theme = Some(entry.text().to_string());
    });

    theme_box.append(&theme_label);
    theme_box.append(&theme_entry);
    cursor_section.append(&theme_box);
    main_box.append(&cursor_section);

    scrolled.set_child(Some(&main_box));

    // Bottom bar with buttons
    let button_box = Box::new(Orientation::Horizontal, 6);
    button_box.set_margin_top(6);
    button_box.set_margin_bottom(6);
    button_box.set_margin_start(6);
    button_box.set_margin_end(6);
    button_box.set_halign(gtk4::Align::End);

    let save_button = Button::with_label("Save Configuration");
    save_button.add_css_class("suggested-action");

    let config_clone = config_rc.clone();
    let path_clone = config_path.clone();
    save_button.connect_clicked(move |_| match config_clone.borrow().save(&path_clone) {
        Ok(_) => println!("✓ Configuration saved to {}", path_clone.display()),
        Err(e) => eprintln!("✗ Failed to save config: {}", e),
    });

    button_box.append(&save_button);

    // Main layout
    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.append(&scrolled);
    vbox.append(&button_box);

    // Window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("driftwm Settings")
        .default_width(700)
        .default_height(600)
        .child(&vbox)
        .build();

    window.present();
}

fn create_section(title: &str) -> Box {
    let section = Box::new(Orientation::Vertical, 6);
    let title_label = Label::new(Some(title));
    title_label.set_halign(gtk4::Align::Start);
    title_label.add_css_class("title-4");
    section.append(&title_label);
    section
}

fn get_config_path() -> PathBuf {
    let config_home = std::env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap()));
    PathBuf::from(config_home).join("driftwm/config.toml")
}
