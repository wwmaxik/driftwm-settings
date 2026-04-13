mod config;
mod config_helpers;
mod ui_helpers;

use config::*;
use config_helpers::*;
use ui_helpers::*;

use gtk4::prelude::*;
use gtk4::{
    Adjustment, Application, ApplicationWindow, Box, Button, Entry, Label, Orientation,
    ScrolledWindow, SpinButton, Stack, StackSidebar, Switch, TextBuffer, TextView,
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

    // Main horizontal box (sidebar + content)
    let main_hbox = Box::new(Orientation::Horizontal, 0);

    // Stack for different pages
    let stack = Stack::new();
    stack.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);

    // Sidebar
    let sidebar = StackSidebar::new();
    sidebar.set_stack(&stack);
    sidebar.set_width_request(200);

    // Add pages to stack
    add_general_page(&stack, config_rc.clone());
    add_keyboard_page(&stack, config_rc.clone());
    add_trackpad_page(&stack, config_rc.clone());
    add_mouse_page(&stack, config_rc.clone());
    add_cursor_page(&stack, config_rc.clone());
    add_navigation_page(&stack, config_rc.clone());
    add_zoom_page(&stack, config_rc.clone());
    add_snap_page(&stack, config_rc.clone());
    add_decorations_page(&stack, config_rc.clone());
    add_effects_page(&stack, config_rc.clone());
    add_background_page(&stack, config_rc.clone());
    add_autostart_page(&stack, config_rc.clone());

    // Scrolled window for stack content
    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&stack));
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);

    main_hbox.append(&sidebar);
    main_hbox.append(&scrolled);

    // Bottom bar with buttons
    let button_box = Box::new(Orientation::Horizontal, 6);
    button_box.set_margin_top(6);
    button_box.set_margin_bottom(6);
    button_box.set_margin_start(6);
    button_box.set_margin_end(6);
    button_box.set_halign(gtk4::Align::End);

    // Config path label
    let path_label = Label::new(Some(&format!("Config: {}", config_path.display())));
    path_label.add_css_class("dim-label");
    path_label.set_hexpand(true);
    path_label.set_halign(gtk4::Align::Start);
    button_box.append(&path_label);

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
    vbox.append(&main_hbox);
    vbox.append(&button_box);

    // Window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("driftwm Settings")
        .default_width(900)
        .default_height(700)
        .child(&vbox)
        .build();

    window.present();
}

fn add_general_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "General Settings");

    // Mod key
    let mod_box = create_row();
    add_label(&mod_box, "Modifier key:", 200);
    let mod_entry = Entry::new();
    mod_entry.set_text(
        &config
            .borrow()
            .mod_key
            .clone()
            .unwrap_or_else(|| "super".to_string()),
    );
    mod_entry.set_placeholder_text(Some("super or alt"));
    mod_entry.set_hexpand(true);

    let config_clone = config.clone();
    mod_entry.connect_changed(move |entry| {
        config_clone.borrow_mut().mod_key = Some(entry.text().to_string());
    });

    mod_box.append(&mod_entry);
    page.append(&mod_box);

    // Focus follows mouse
    let focus_box = create_row();
    add_label(&focus_box, "Focus follows mouse:", 200);
    let focus_switch = Switch::new();
    focus_switch.set_active(config.borrow().focus_follows_mouse.unwrap_or(false));

    let config_clone = config.clone();
    focus_switch.connect_state_set(move |_, state| {
        config_clone.borrow_mut().focus_follows_mouse = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    focus_box.append(&focus_switch);
    page.append(&focus_box);

    stack.add_titled(&page, Some("general"), "General");
}

fn add_keyboard_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Keyboard Settings");

    // Layout
    let layout_box = create_row();
    add_label(&layout_box, "Layout:", 200);
    let layout_entry = Entry::new();
    layout_entry.set_text(
        &config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.keyboard.as_ref())
            .and_then(|k| k.layout.clone())
            .unwrap_or_else(|| "us".to_string()),
    );
    layout_entry.set_placeholder_text(Some("us,ru"));
    layout_entry.set_hexpand(true);

    let config_clone = config.clone();
    layout_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_keyboard(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .keyboard
            .as_mut()
            .unwrap()
            .layout = Some(entry.text().to_string());
    });

    layout_box.append(&layout_entry);
    page.append(&layout_box);

    // Variant
    let variant_box = create_row();
    add_label(&variant_box, "Variant:", 200);
    let variant_entry = Entry::new();
    variant_entry.set_text(
        &config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.keyboard.as_ref())
            .and_then(|k| k.variant.clone())
            .unwrap_or_default(),
    );
    variant_entry.set_placeholder_text(Some("dvorak"));
    variant_entry.set_hexpand(true);

    let config_clone = config.clone();
    variant_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_keyboard(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .keyboard
            .as_mut()
            .unwrap()
            .variant = Some(entry.text().to_string());
    });

    variant_box.append(&variant_entry);
    page.append(&variant_box);

    // Options
    let options_box = create_row();
    add_label(&options_box, "Options:", 200);
    let options_entry = Entry::new();
    options_entry.set_text(
        &config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.keyboard.as_ref())
            .and_then(|k| k.options.clone())
            .unwrap_or_default(),
    );
    options_entry.set_placeholder_text(Some("grp:win_space_toggle"));
    options_entry.set_hexpand(true);

    let config_clone = config.clone();
    options_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_keyboard(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .keyboard
            .as_mut()
            .unwrap()
            .options = Some(entry.text().to_string());
    });

    options_box.append(&options_entry);
    page.append(&options_box);

    // Repeat rate
    let rate_box = create_row();
    add_label(&rate_box, "Repeat rate (keys/sec):", 200);
    let rate_spin = SpinButton::new(
        Some(&Adjustment::new(25.0, 1.0, 100.0, 1.0, 5.0, 0.0)),
        1.0,
        0,
    );
    rate_spin.set_value(
        config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.keyboard.as_ref())
            .and_then(|k| k.repeat_rate)
            .unwrap_or(25) as f64,
    );

    let config_clone = config.clone();
    rate_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_keyboard(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .keyboard
            .as_mut()
            .unwrap()
            .repeat_rate = Some(spin.value() as i32);
    });

    rate_box.append(&rate_spin);
    page.append(&rate_box);

    // Repeat delay
    let delay_box = create_row();
    add_label(&delay_box, "Repeat delay (ms):", 200);
    let delay_spin = SpinButton::new(
        Some(&Adjustment::new(200.0, 100.0, 1000.0, 10.0, 50.0, 0.0)),
        10.0,
        0,
    );
    delay_spin.set_value(
        config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.keyboard.as_ref())
            .and_then(|k| k.repeat_delay)
            .unwrap_or(200) as f64,
    );

    let config_clone = config.clone();
    delay_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_keyboard(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .keyboard
            .as_mut()
            .unwrap()
            .repeat_delay = Some(spin.value() as i32);
    });

    delay_box.append(&delay_spin);
    page.append(&delay_box);

    // Layout independent
    let independent_box = create_row();
    add_label(&independent_box, "Layout independent:", 200);
    let independent_switch = Switch::new();
    independent_switch.set_active(
        config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.keyboard.as_ref())
            .and_then(|k| k.layout_independent)
            .unwrap_or(true),
    );

    let config_clone = config.clone();
    independent_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_keyboard(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .keyboard
            .as_mut()
            .unwrap()
            .layout_independent = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    independent_box.append(&independent_switch);
    page.append(&independent_box);

    stack.add_titled(&page, Some("keyboard"), "Keyboard");
}

fn add_trackpad_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Trackpad Settings");

    // Tap to click
    let tap_box = create_row();
    add_label(&tap_box, "Tap to click:", 200);
    let tap_switch = Switch::new();
    tap_switch.set_active(
        config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.trackpad.as_ref())
            .and_then(|t| t.tap_to_click)
            .unwrap_or(true),
    );

    let config_clone = config.clone();
    tap_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_trackpad(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .trackpad
            .as_mut()
            .unwrap()
            .tap_to_click = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    tap_box.append(&tap_switch);
    page.append(&tap_box);

    // Natural scroll
    let natural_box = create_row();
    add_label(&natural_box, "Natural scroll:", 200);
    let natural_switch = Switch::new();
    natural_switch.set_active(
        config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.trackpad.as_ref())
            .and_then(|t| t.natural_scroll)
            .unwrap_or(true),
    );

    let config_clone = config.clone();
    natural_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_trackpad(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .trackpad
            .as_mut()
            .unwrap()
            .natural_scroll = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    natural_box.append(&natural_switch);
    page.append(&natural_box);

    // Tap and drag
    let drag_box = create_row();
    add_label(&drag_box, "Tap and drag:", 200);
    let drag_switch = Switch::new();
    drag_switch.set_active(
        config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.trackpad.as_ref())
            .and_then(|t| t.tap_and_drag)
            .unwrap_or(true),
    );

    let config_clone = config.clone();
    drag_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_trackpad(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .trackpad
            .as_mut()
            .unwrap()
            .tap_and_drag = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    drag_box.append(&drag_switch);
    page.append(&drag_box);

    // Acceleration speed
    let accel_box = create_row();
    add_label(&accel_box, "Acceleration speed:", 200);
    let accel_spin = SpinButton::new(
        Some(&Adjustment::new(0.0, -1.0, 1.0, 0.1, 0.2, 0.0)),
        0.1,
        1,
    );
    accel_spin.set_value(
        config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.trackpad.as_ref())
            .and_then(|t| t.accel_speed)
            .unwrap_or(0.0),
    );

    let config_clone = config.clone();
    accel_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_trackpad(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .trackpad
            .as_mut()
            .unwrap()
            .accel_speed = Some(spin.value());
    });

    accel_box.append(&accel_spin);
    page.append(&accel_box);

    stack.add_titled(&page, Some("trackpad"), "Trackpad");
}

// Stub functions for remaining pages - will implement in next message
fn add_mouse_page(stack: &Stack, _config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();
    add_header(&page, "Mouse Settings");
    add_label(&page, "Coming soon...", 0);
    stack.add_titled(&page, Some("mouse"), "Mouse");
}

fn add_cursor_page(stack: &Stack, _config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();
    add_header(&page, "Cursor Settings");
    add_label(&page, "Coming soon...", 0);
    stack.add_titled(&page, Some("cursor"), "Cursor");
}

fn add_navigation_page(stack: &Stack, _config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();
    add_header(&page, "Navigation Settings");
    add_label(&page, "Coming soon...", 0);
    stack.add_titled(&page, Some("navigation"), "Navigation");
}

fn add_zoom_page(stack: &Stack, _config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();
    add_header(&page, "Zoom Settings");
    add_label(&page, "Coming soon...", 0);
    stack.add_titled(&page, Some("zoom"), "Zoom");
}

fn add_snap_page(stack: &Stack, _config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();
    add_header(&page, "Snap Settings");
    add_label(&page, "Coming soon...", 0);
    stack.add_titled(&page, Some("snap"), "Snap");
}

fn add_decorations_page(stack: &Stack, _config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();
    add_header(&page, "Decorations");
    add_label(&page, "Coming soon...", 0);
    stack.add_titled(&page, Some("decorations"), "Decorations");
}

fn add_effects_page(stack: &Stack, _config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();
    add_header(&page, "Effects");
    add_label(&page, "Coming soon...", 0);
    stack.add_titled(&page, Some("effects"), "Effects");
}

fn add_background_page(stack: &Stack, _config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();
    add_header(&page, "Background");
    add_label(&page, "Coming soon...", 0);
    stack.add_titled(&page, Some("background"), "Background");
}

fn add_autostart_page(stack: &Stack, _config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();
    add_header(&page, "Autostart");
    add_label(&page, "Coming soon...", 0);
    stack.add_titled(&page, Some("autostart"), "Autostart");
}

fn get_config_path() -> PathBuf {
    let config_home = std::env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap()));
    PathBuf::from(config_home).join("driftwm/config.toml")
}
