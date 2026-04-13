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

fn add_mouse_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Mouse Settings");

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
            .and_then(|i| i.mouse.as_ref())
            .and_then(|m| m.accel_speed)
            .unwrap_or(0.0),
    );

    let config_clone = config.clone();
    accel_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_mouse(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .mouse
            .as_mut()
            .unwrap()
            .accel_speed = Some(spin.value());
    });

    accel_box.append(&accel_spin);
    page.append(&accel_box);

    // Acceleration profile
    let profile_box = create_row();
    add_label(&profile_box, "Acceleration profile:", 200);
    let profile_entry = Entry::new();
    profile_entry.set_text(
        &config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.mouse.as_ref())
            .and_then(|m| m.accel_profile.clone())
            .unwrap_or_else(|| "flat".to_string()),
    );
    profile_entry.set_placeholder_text(Some("adaptive or flat"));
    profile_entry.set_hexpand(true);

    let config_clone = config.clone();
    profile_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_mouse(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .mouse
            .as_mut()
            .unwrap()
            .accel_profile = Some(entry.text().to_string());
    });

    profile_box.append(&profile_entry);
    page.append(&profile_box);

    // Natural scroll
    let natural_box = create_row();
    add_label(&natural_box, "Natural scroll:", 200);
    let natural_switch = Switch::new();
    natural_switch.set_active(
        config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.mouse.as_ref())
            .and_then(|m| m.natural_scroll)
            .unwrap_or(false),
    );

    let config_clone = config.clone();
    natural_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_mouse(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .mouse
            .as_mut()
            .unwrap()
            .natural_scroll = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    natural_box.append(&natural_switch);
    page.append(&natural_box);

    stack.add_titled(&page, Some("mouse"), "Mouse");
}

fn add_cursor_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Cursor Settings");

    // Theme
    let theme_box = create_row();
    add_label(&theme_box, "Cursor theme:", 200);
    let theme_entry = Entry::new();
    theme_entry.set_text(
        &config
            .borrow()
            .cursor
            .as_ref()
            .and_then(|c| c.theme.clone())
            .unwrap_or_else(|| "Adwaita".to_string()),
    );
    theme_entry.set_hexpand(true);

    let config_clone = config.clone();
    theme_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.cursor.is_none() {
            cfg.cursor = Some(CursorConfig::default());
        }
        cfg.cursor.as_mut().unwrap().theme = Some(entry.text().to_string());
    });

    theme_box.append(&theme_entry);
    page.append(&theme_box);

    // Size
    let size_box = create_row();
    add_label(&size_box, "Cursor size:", 200);
    let size_spin = SpinButton::new(
        Some(&Adjustment::new(24.0, 16.0, 64.0, 1.0, 4.0, 0.0)),
        1.0,
        0,
    );
    size_spin.set_value(
        config
            .borrow()
            .cursor
            .as_ref()
            .and_then(|c| c.size)
            .unwrap_or(24) as f64,
    );

    let config_clone = config.clone();
    size_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.cursor.is_none() {
            cfg.cursor = Some(CursorConfig::default());
        }
        cfg.cursor.as_mut().unwrap().size = Some(spin.value() as u32);
    });

    size_box.append(&size_spin);
    page.append(&size_box);

    // Inactive opacity
    let opacity_box = create_row();
    add_label(&opacity_box, "Inactive opacity:", 200);
    let opacity_spin =
        SpinButton::new(Some(&Adjustment::new(0.5, 0.0, 1.0, 0.1, 0.2, 0.0)), 0.1, 1);
    opacity_spin.set_value(
        config
            .borrow()
            .cursor
            .as_ref()
            .and_then(|c| c.inactive_opacity)
            .unwrap_or(0.5),
    );

    let config_clone = config.clone();
    opacity_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.cursor.is_none() {
            cfg.cursor = Some(CursorConfig::default());
        }
        cfg.cursor.as_mut().unwrap().inactive_opacity = Some(spin.value());
    });

    opacity_box.append(&opacity_spin);
    page.append(&opacity_box);

    stack.add_titled(&page, Some("cursor"), "Cursor");
}

fn add_navigation_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Navigation Settings");

    // Trackpad speed
    let trackpad_speed_box = create_row();
    add_label(&trackpad_speed_box, "Trackpad speed:", 200);
    let trackpad_speed_spin =
        SpinButton::new(Some(&Adjustment::new(1.5, 0.1, 5.0, 0.1, 0.5, 0.0)), 0.1, 1);
    trackpad_speed_spin.set_value(
        config
            .borrow()
            .navigation
            .as_ref()
            .and_then(|n| n.trackpad_speed)
            .unwrap_or(1.5),
    );

    let config_clone = config.clone();
    trackpad_speed_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        ensure_navigation(&mut cfg);
        cfg.navigation.as_mut().unwrap().trackpad_speed = Some(spin.value());
    });

    trackpad_speed_box.append(&trackpad_speed_spin);
    page.append(&trackpad_speed_box);

    // Mouse speed
    let mouse_speed_box = create_row();
    add_label(&mouse_speed_box, "Mouse speed:", 200);
    let mouse_speed_spin =
        SpinButton::new(Some(&Adjustment::new(1.0, 0.1, 5.0, 0.1, 0.5, 0.0)), 0.1, 1);
    mouse_speed_spin.set_value(
        config
            .borrow()
            .navigation
            .as_ref()
            .and_then(|n| n.mouse_speed)
            .unwrap_or(1.0),
    );

    let config_clone = config.clone();
    mouse_speed_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        ensure_navigation(&mut cfg);
        cfg.navigation.as_mut().unwrap().mouse_speed = Some(spin.value());
    });

    mouse_speed_box.append(&mouse_speed_spin);
    page.append(&mouse_speed_box);

    // Friction
    let friction_box = create_row();
    add_label(&friction_box, "Friction:", 200);
    let friction_spin = SpinButton::new(
        Some(&Adjustment::new(0.94, 0.80, 0.99, 0.01, 0.05, 0.0)),
        0.01,
        2,
    );
    friction_spin.set_value(
        config
            .borrow()
            .navigation
            .as_ref()
            .and_then(|n| n.friction)
            .unwrap_or(0.94),
    );

    let config_clone = config.clone();
    friction_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        ensure_navigation(&mut cfg);
        cfg.navigation.as_mut().unwrap().friction = Some(spin.value());
    });

    friction_box.append(&friction_spin);
    page.append(&friction_box);

    // Animation speed
    let anim_box = create_row();
    add_label(&anim_box, "Animation speed:", 200);
    let anim_spin = SpinButton::new(
        Some(&Adjustment::new(0.3, 0.1, 1.0, 0.05, 0.1, 0.0)),
        0.05,
        2,
    );
    anim_spin.set_value(
        config
            .borrow()
            .navigation
            .as_ref()
            .and_then(|n| n.animation_speed)
            .unwrap_or(0.3),
    );

    let config_clone = config.clone();
    anim_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        ensure_navigation(&mut cfg);
        cfg.navigation.as_mut().unwrap().animation_speed = Some(spin.value());
    });

    anim_box.append(&anim_spin);
    page.append(&anim_box);

    // Nudge step
    let nudge_box = create_row();
    add_label(&nudge_box, "Nudge step (px):", 200);
    let nudge_spin = SpinButton::new(
        Some(&Adjustment::new(20.0, 1.0, 100.0, 1.0, 10.0, 0.0)),
        1.0,
        0,
    );
    nudge_spin.set_value(
        config
            .borrow()
            .navigation
            .as_ref()
            .and_then(|n| n.nudge_step)
            .unwrap_or(20) as f64,
    );

    let config_clone = config.clone();
    nudge_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        ensure_navigation(&mut cfg);
        cfg.navigation.as_mut().unwrap().nudge_step = Some(spin.value() as i32);
    });

    nudge_box.append(&nudge_spin);
    page.append(&nudge_box);

    // Pan step
    let pan_box = create_row();
    add_label(&pan_box, "Pan step (px):", 200);
    let pan_spin = SpinButton::new(
        Some(&Adjustment::new(100.0, 10.0, 500.0, 10.0, 50.0, 0.0)),
        10.0,
        0,
    );
    pan_spin.set_value(
        config
            .borrow()
            .navigation
            .as_ref()
            .and_then(|n| n.pan_step)
            .unwrap_or(100.0),
    );

    let config_clone = config.clone();
    pan_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        ensure_navigation(&mut cfg);
        cfg.navigation.as_mut().unwrap().pan_step = Some(spin.value());
    });

    pan_box.append(&pan_spin);
    page.append(&pan_box);

    stack.add_titled(&page, Some("navigation"), "Navigation");
}

fn add_zoom_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Zoom Settings");

    // Zoom step
    let step_box = create_row();
    add_label(&step_box, "Zoom step:", 200);
    let step_spin = SpinButton::new(
        Some(&Adjustment::new(1.1, 1.01, 2.0, 0.01, 0.1, 0.0)),
        0.01,
        2,
    );
    step_spin.set_value(
        config
            .borrow()
            .zoom
            .as_ref()
            .and_then(|z| z.step)
            .unwrap_or(1.1),
    );

    let config_clone = config.clone();
    step_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.zoom.is_none() {
            cfg.zoom = Some(ZoomConfig::default());
        }
        cfg.zoom.as_mut().unwrap().step = Some(spin.value());
    });

    step_box.append(&step_spin);
    page.append(&step_box);

    // Fit padding
    let padding_box = create_row();
    add_label(&padding_box, "Fit padding (px):", 200);
    let padding_spin = SpinButton::new(
        Some(&Adjustment::new(100.0, 0.0, 500.0, 10.0, 50.0, 0.0)),
        10.0,
        0,
    );
    padding_spin.set_value(
        config
            .borrow()
            .zoom
            .as_ref()
            .and_then(|z| z.fit_padding)
            .unwrap_or(100.0),
    );

    let config_clone = config.clone();
    padding_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.zoom.is_none() {
            cfg.zoom = Some(ZoomConfig::default());
        }
        cfg.zoom.as_mut().unwrap().fit_padding = Some(spin.value());
    });

    padding_box.append(&padding_spin);
    page.append(&padding_box);

    stack.add_titled(&page, Some("zoom"), "Zoom");
}

fn add_snap_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Snap Settings");

    // Enabled
    let enabled_box = create_row();
    add_label(&enabled_box, "Enable snapping:", 200);
    let enabled_switch = Switch::new();
    enabled_switch.set_active(
        config
            .borrow()
            .snap
            .as_ref()
            .and_then(|s| s.enabled)
            .unwrap_or(true),
    );

    let config_clone = config.clone();
    enabled_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.snap.is_none() {
            cfg.snap = Some(SnapConfig::default());
        }
        cfg.snap.as_mut().unwrap().enabled = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    enabled_box.append(&enabled_switch);
    page.append(&enabled_box);

    // Gap
    let gap_box = create_row();
    add_label(&gap_box, "Gap (px):", 200);
    let gap_spin = SpinButton::new(
        Some(&Adjustment::new(12.0, 0.0, 100.0, 1.0, 10.0, 0.0)),
        1.0,
        0,
    );
    gap_spin.set_value(
        config
            .borrow()
            .snap
            .as_ref()
            .and_then(|s| s.gap)
            .unwrap_or(12.0),
    );

    let config_clone = config.clone();
    gap_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.snap.is_none() {
            cfg.snap = Some(SnapConfig::default());
        }
        cfg.snap.as_mut().unwrap().gap = Some(spin.value());
    });

    gap_box.append(&gap_spin);
    page.append(&gap_box);

    // Distance
    let distance_box = create_row();
    add_label(&distance_box, "Distance (px):", 200);
    let distance_spin = SpinButton::new(
        Some(&Adjustment::new(24.0, 1.0, 100.0, 1.0, 10.0, 0.0)),
        1.0,
        0,
    );
    distance_spin.set_value(
        config
            .borrow()
            .snap
            .as_ref()
            .and_then(|s| s.distance)
            .unwrap_or(24.0),
    );

    let config_clone = config.clone();
    distance_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.snap.is_none() {
            cfg.snap = Some(SnapConfig::default());
        }
        cfg.snap.as_mut().unwrap().distance = Some(spin.value());
    });

    distance_box.append(&distance_spin);
    page.append(&distance_box);

    // Break force
    let break_box = create_row();
    add_label(&break_box, "Break force (px):", 200);
    let break_spin = SpinButton::new(
        Some(&Adjustment::new(32.0, 1.0, 100.0, 1.0, 10.0, 0.0)),
        1.0,
        0,
    );
    break_spin.set_value(
        config
            .borrow()
            .snap
            .as_ref()
            .and_then(|s| s.break_force)
            .unwrap_or(32.0),
    );

    let config_clone = config.clone();
    break_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.snap.is_none() {
            cfg.snap = Some(SnapConfig::default());
        }
        cfg.snap.as_mut().unwrap().break_force = Some(spin.value());
    });

    break_box.append(&break_spin);
    page.append(&break_box);

    // Same edge
    let same_edge_box = create_row();
    add_label(&same_edge_box, "Snap same edges:", 200);
    let same_edge_switch = Switch::new();
    same_edge_switch.set_active(
        config
            .borrow()
            .snap
            .as_ref()
            .and_then(|s| s.same_edge)
            .unwrap_or(false),
    );

    let config_clone = config.clone();
    same_edge_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.snap.is_none() {
            cfg.snap = Some(SnapConfig::default());
        }
        cfg.snap.as_mut().unwrap().same_edge = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    same_edge_box.append(&same_edge_switch);
    page.append(&same_edge_box);

    stack.add_titled(&page, Some("snap"), "Snap");
}

fn add_decorations_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Window Decorations");

    // Background color
    let bg_box = create_row();
    add_label(&bg_box, "Background color:", 200);
    let bg_entry = Entry::new();
    bg_entry.set_text(
        &config
            .borrow()
            .decorations
            .as_ref()
            .and_then(|d| d.bg_color.clone())
            .unwrap_or_else(|| "#303030".to_string()),
    );
    bg_entry.set_placeholder_text(Some("#303030"));
    bg_entry.set_hexpand(true);

    let config_clone = config.clone();
    bg_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.decorations.is_none() {
            cfg.decorations = Some(DecorationsConfig::default());
        }
        cfg.decorations.as_mut().unwrap().bg_color = Some(entry.text().to_string());
    });

    bg_box.append(&bg_entry);
    page.append(&bg_box);

    // Foreground color
    let fg_box = create_row();
    add_label(&fg_box, "Foreground color:", 200);
    let fg_entry = Entry::new();
    fg_entry.set_text(
        &config
            .borrow()
            .decorations
            .as_ref()
            .and_then(|d| d.fg_color.clone())
            .unwrap_or_else(|| "#FFFFFF".to_string()),
    );
    fg_entry.set_placeholder_text(Some("#FFFFFF"));
    fg_entry.set_hexpand(true);

    let config_clone = config.clone();
    fg_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.decorations.is_none() {
            cfg.decorations = Some(DecorationsConfig::default());
        }
        cfg.decorations.as_mut().unwrap().fg_color = Some(entry.text().to_string());
    });

    fg_box.append(&fg_entry);
    page.append(&fg_box);

    // Corner radius
    let radius_box = create_row();
    add_label(&radius_box, "Corner radius:", 200);
    let radius_spin = SpinButton::new(
        Some(&Adjustment::new(8.0, 0.0, 32.0, 1.0, 4.0, 0.0)),
        1.0,
        0,
    );
    radius_spin.set_value(
        config
            .borrow()
            .decorations
            .as_ref()
            .and_then(|d| d.corner_radius)
            .unwrap_or(8) as f64,
    );

    let config_clone = config.clone();
    radius_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.decorations.is_none() {
            cfg.decorations = Some(DecorationsConfig::default());
        }
        cfg.decorations.as_mut().unwrap().corner_radius = Some(spin.value() as i32);
    });

    radius_box.append(&radius_spin);
    page.append(&radius_box);

    stack.add_titled(&page, Some("decorations"), "Decorations");
}

fn add_effects_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Visual Effects");

    // Blur radius
    let radius_box = create_row();
    add_label(&radius_box, "Blur radius:", 200);
    let radius_spin = SpinButton::new(
        Some(&Adjustment::new(2.0, 0.0, 10.0, 1.0, 2.0, 0.0)),
        1.0,
        0,
    );
    radius_spin.set_value(
        config
            .borrow()
            .effects
            .as_ref()
            .and_then(|e| e.blur_radius)
            .unwrap_or(2) as f64,
    );

    let config_clone = config.clone();
    radius_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.effects.is_none() {
            cfg.effects = Some(EffectsConfig::default());
        }
        cfg.effects.as_mut().unwrap().blur_radius = Some(spin.value() as i32);
    });

    radius_box.append(&radius_spin);
    page.append(&radius_box);

    // Blur strength
    let strength_box = create_row();
    add_label(&strength_box, "Blur strength:", 200);
    let strength_spin =
        SpinButton::new(Some(&Adjustment::new(1.1, 0.5, 3.0, 0.1, 0.5, 0.0)), 0.1, 1);
    strength_spin.set_value(
        config
            .borrow()
            .effects
            .as_ref()
            .and_then(|e| e.blur_strength)
            .unwrap_or(1.1),
    );

    let config_clone = config.clone();
    strength_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.effects.is_none() {
            cfg.effects = Some(EffectsConfig::default());
        }
        cfg.effects.as_mut().unwrap().blur_strength = Some(spin.value());
    });

    strength_box.append(&strength_spin);
    page.append(&strength_box);

    stack.add_titled(&page, Some("effects"), "Effects");
}

fn add_background_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Background");

    // Shader path
    let shader_box = create_row();
    add_label(&shader_box, "Shader path:", 200);
    let shader_entry = Entry::new();
    shader_entry.set_text(
        &config
            .borrow()
            .background
            .as_ref()
            .and_then(|b| b.shader_path.clone())
            .unwrap_or_default(),
    );
    shader_entry.set_placeholder_text(Some("~/.config/driftwm/bg.frag"));
    shader_entry.set_hexpand(true);

    let config_clone = config.clone();
    shader_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.background.is_none() {
            cfg.background = Some(BackgroundConfig::default());
        }
        cfg.background.as_mut().unwrap().shader_path = Some(entry.text().to_string());
    });

    shader_box.append(&shader_entry);
    page.append(&shader_box);

    // Tile path
    let tile_box = create_row();
    add_label(&tile_box, "Tile path:", 200);
    let tile_entry = Entry::new();
    tile_entry.set_text(
        &config
            .borrow()
            .background
            .as_ref()
            .and_then(|b| b.tile_path.clone())
            .unwrap_or_default(),
    );
    tile_entry.set_placeholder_text(Some("~/.config/driftwm/tile.png"));
    tile_entry.set_hexpand(true);

    let config_clone = config.clone();
    tile_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.background.is_none() {
            cfg.background = Some(BackgroundConfig::default());
        }
        cfg.background.as_mut().unwrap().tile_path = Some(entry.text().to_string());
    });

    tile_box.append(&tile_entry);
    page.append(&tile_box);

    stack.add_titled(&page, Some("background"), "Background");
}

fn add_autostart_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Autostart Programs");

    let info_label = Label::new(Some("Enter commands to run at startup (one per line):"));
    info_label.set_halign(gtk4::Align::Start);
    info_label.add_css_class("dim-label");
    page.append(&info_label);

    // Text view for autostart commands
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_min_content_height(300);

    let text_view = TextView::new();
    text_view.set_monospace(true);
    text_view.set_left_margin(6);
    text_view.set_right_margin(6);
    text_view.set_top_margin(6);
    text_view.set_bottom_margin(6);

    let buffer = text_view.buffer();

    // Load existing autostart commands
    let autostart_text = config
        .borrow()
        .autostart
        .as_ref()
        .map(|cmds| cmds.join("\n"))
        .unwrap_or_default();
    buffer.set_text(&autostart_text);

    let config_clone = config.clone();
    buffer.connect_changed(move |buf| {
        let start = buf.start_iter();
        let end = buf.end_iter();
        let text = buf.text(&start, &end, false);

        let commands: Vec<String> = text
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let mut cfg = config_clone.borrow_mut();
        cfg.autostart = if commands.is_empty() {
            None
        } else {
            Some(commands)
        };
    });

    scrolled.set_child(Some(&text_view));
    page.append(&scrolled);

    stack.add_titled(&page, Some("autostart"), "Autostart");
}

fn get_config_path() -> PathBuf {
    let config_home = std::env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap()));
    PathBuf::from(config_home).join("driftwm/config.toml")
}
