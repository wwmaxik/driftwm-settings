mod config;
mod config_helpers;
mod shader_editor;
mod ui_helpers;

use config::*;
use config_helpers::*;
use shader_editor::add_shader_editor_page;
use ui_helpers::*;

use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Adjustment, Application, ApplicationWindow, Box, Button, ComboBoxText, CssProvider, Entry,
    HeaderBar, Label, Orientation, ScrolledWindow, SpinButton, Stack, StackSidebar, Switch,
    TextView,
};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

const APP_ID: &str = "com.github.driftwm.settings";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        // Load CSS
        let provider = CssProvider::new();
        provider.load_from_data(include_str!("../style.css"));
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().expect("Could not connect to display"),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let config_path = get_config_path();
    let config = DriftwmConfig::load(&config_path).unwrap_or_default();
    let config_rc = Rc::new(RefCell::new(config));

    // Header Bar
    let header = HeaderBar::new();

    // Main horizontal box (sidebar + content)
    let main_hbox = Box::new(Orientation::Horizontal, 0);

    // Stack for different pages
    let stack = Stack::new();
    stack.set_transition_type(gtk4::StackTransitionType::SlideLeftRight);
    stack.set_transition_duration(150);

    // Sidebar
    let sidebar = StackSidebar::new();
    sidebar.set_stack(&stack);
    sidebar.set_width_request(180);

    // Add pages to stack with icons
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
    add_window_rules_page(&stack, config_rc.clone());
    add_backend_page(&stack, config_rc.clone());
    add_background_page(&stack, config_rc.clone());
    add_shader_editor_page(&stack, config_rc.clone());
    add_keybindings_page(&stack, config_rc.clone());
    add_autostart_page(&stack, config_rc.clone());

    // Scrolled window for stack content
    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&stack));
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);

    main_hbox.append(&sidebar);
    main_hbox.append(&scrolled);

    // Status bar at bottom
    let statusbar = Box::new(Orientation::Horizontal, 12);
    statusbar.set_margin_top(6);
    statusbar.set_margin_bottom(6);
    statusbar.set_margin_start(12);
    statusbar.set_margin_end(12);
    statusbar.add_css_class("statusbar");

    let path_label = Label::new(Some(&format!("{}", config_path.display())));
    path_label.add_css_class("dim-label");
    path_label.add_css_class("caption");
    path_label.set_halign(gtk4::Align::Start);
    path_label.set_hexpand(true);
    statusbar.append(&path_label);

    // Save button
    let save_button = Button::with_label("Save");
    save_button.add_css_class("suggested-action");

    let config_clone = config_rc.clone();
    let path_clone = config_path.clone();
    let button_clone = save_button.clone();
    save_button.connect_clicked(move |btn| {
        match config_clone.borrow().save(&path_clone) {
            Ok(_) => {
                println!("✓ Configuration saved to {}", path_clone.display());
                btn.set_label("✓ Saved!");
                btn.remove_css_class("suggested-action");
                btn.add_css_class("success");

                // Reset button after 2 seconds
                let btn_clone = button_clone.clone();
                glib::timeout_add_seconds_local(2, move || {
                    btn_clone.set_label("Save");
                    btn_clone.remove_css_class("success");
                    btn_clone.add_css_class("suggested-action");
                    glib::ControlFlow::Break
                });
            }
            Err(e) => {
                eprintln!("✗ Failed to save config: {}", e);
                btn.set_label("✗ Error!");
                btn.remove_css_class("suggested-action");
                btn.add_css_class("destructive-action");

                // Reset button after 2 seconds
                let btn_clone = button_clone.clone();
                glib::timeout_add_seconds_local(2, move || {
                    btn_clone.set_label("Save");
                    btn_clone.remove_css_class("destructive-action");
                    btn_clone.add_css_class("suggested-action");
                    glib::ControlFlow::Break
                });
            }
        }
    });

    statusbar.append(&save_button);

    // Main layout
    let vbox = Box::new(Orientation::Vertical, 0);
    vbox.append(&main_hbox);
    vbox.append(&statusbar);

    // Window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("driftwm Settings")
        .default_width(1000)
        .default_height(700)
        .child(&vbox)
        .build();

    window.set_titlebar(Some(&header));
    window.present();
}

fn add_general_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "General Settings");

    // Modifier key
    let mod_box = create_row();
    add_label(&mod_box, "Modifier key:", 200);

    let mod_combo = ComboBoxText::new();
    mod_combo.append(Some("super"), "Super (Windows key)");
    mod_combo.append(Some("alt"), "Alt");
    mod_combo.append(Some("custom"), "Custom...");

    let current_mod = config
        .borrow()
        .mod_key
        .clone()
        .unwrap_or_else(|| "super".to_string());
    if current_mod == "super" || current_mod == "alt" {
        mod_combo.set_active_id(Some(&current_mod));
    } else {
        mod_combo.set_active_id(Some("custom"));
    }

    let mod_entry = Entry::new();
    mod_entry.set_text(&current_mod);
    mod_entry.set_hexpand(true);
    mod_entry.set_visible(current_mod != "super" && current_mod != "alt");
    mod_entry.set_placeholder_text(Some("e.g., ctrl, shift"));

    let config_clone = config.clone();
    let entry_clone = mod_entry.clone();
    mod_combo.connect_changed(move |combo| {
        if let Some(id) = combo.active_id() {
            let id_str = id.as_str();
            if id_str == "custom" {
                entry_clone.set_visible(true);
            } else {
                entry_clone.set_visible(false);
                config_clone.borrow_mut().mod_key = Some(id_str.to_string());
            }
        }
    });

    let config_clone = config.clone();
    mod_entry.connect_changed(move |entry| {
        config_clone.borrow_mut().mod_key = Some(entry.text().to_string());
    });

    mod_box.append(&mod_combo);
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

    // Options with dropdown
    let options_box = create_row();
    add_label(&options_box, "Options:", 200);

    let options_combo = ComboBoxText::new();
    options_combo.append(Some("grp:win_space_toggle"), "Super+Space to switch layout");
    options_combo.append(Some("grp:alt_shift_toggle"), "Alt+Shift to switch layout");
    options_combo.append(Some("grp:ctrl_shift_toggle"), "Ctrl+Shift to switch layout");
    options_combo.append(Some("caps:escape"), "Caps Lock as Escape");
    options_combo.append(Some("caps:ctrl_modifier"), "Caps Lock as Ctrl");
    options_combo.append(Some("custom"), "Custom...");

    let current_options = config
        .borrow()
        .input
        .as_ref()
        .and_then(|i| i.keyboard.as_ref())
        .and_then(|k| k.options.clone())
        .unwrap_or_default();

    let is_predefined = [
        "grp:win_space_toggle",
        "grp:alt_shift_toggle",
        "grp:ctrl_shift_toggle",
        "caps:escape",
        "caps:ctrl_modifier",
    ]
    .contains(&current_options.as_str());

    if is_predefined {
        options_combo.set_active_id(Some(&current_options));
    } else if !current_options.is_empty() {
        options_combo.set_active_id(Some("custom"));
    }

    let options_entry = Entry::new();
    options_entry.set_text(&current_options);
    options_entry.set_hexpand(true);
    options_entry.set_visible(!is_predefined && !current_options.is_empty());

    let config_clone = config.clone();
    let entry_clone = options_entry.clone();
    options_combo.connect_changed(move |combo| {
        if let Some(id) = combo.active_id() {
            let id_str = id.as_str();
            if id_str == "custom" {
                entry_clone.set_visible(true);
            } else {
                entry_clone.set_visible(false);
                let mut cfg = config_clone.borrow_mut();
                ensure_input_keyboard(&mut cfg);
                cfg.input
                    .as_mut()
                    .unwrap()
                    .keyboard
                    .as_mut()
                    .unwrap()
                    .options = Some(id_str.to_string());
            }
        }
    });

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

    options_box.append(&options_combo);
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

    // Acceleration profile with dropdown
    let profile_box = create_row();
    add_label(&profile_box, "Acceleration profile:", 200);

    let profile_combo = ComboBoxText::new();
    profile_combo.append(Some("adaptive"), "Adaptive");
    profile_combo.append(Some("flat"), "Flat");
    profile_combo.append(Some("custom"), "Custom...");

    let current_profile = config
        .borrow()
        .input
        .as_ref()
        .and_then(|i| i.trackpad.as_ref())
        .and_then(|t| t.accel_profile.clone())
        .unwrap_or_else(|| "adaptive".to_string());

    if current_profile == "adaptive" || current_profile == "flat" {
        profile_combo.set_active_id(Some(&current_profile));
    } else {
        profile_combo.set_active_id(Some("custom"));
    }

    let profile_entry = Entry::new();
    profile_entry.set_text(&current_profile);
    profile_entry.set_hexpand(true);
    profile_entry.set_visible(current_profile != "adaptive" && current_profile != "flat");

    let config_clone = config.clone();
    let entry_clone = profile_entry.clone();
    profile_combo.connect_changed(move |combo| {
        if let Some(id) = combo.active_id() {
            let id_str = id.as_str();
            if id_str == "custom" {
                entry_clone.set_visible(true);
            } else {
                entry_clone.set_visible(false);
                let mut cfg = config_clone.borrow_mut();
                ensure_input_trackpad(&mut cfg);
                cfg.input
                    .as_mut()
                    .unwrap()
                    .trackpad
                    .as_mut()
                    .unwrap()
                    .accel_profile = Some(id_str.to_string());
            }
        }
    });

    let config_clone = config.clone();
    profile_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_trackpad(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .trackpad
            .as_mut()
            .unwrap()
            .accel_profile = Some(entry.text().to_string());
    });

    profile_box.append(&profile_combo);
    profile_box.append(&profile_entry);
    page.append(&profile_box);

    // Click method with dropdown
    let click_box = create_row();
    add_label(&click_box, "Click method:", 200);

    let click_combo = ComboBoxText::new();
    click_combo.append(Some("none"), "Device Default");
    click_combo.append(Some("clickfinger"), "Clickfinger (1=L, 2=R, 3=M)");
    click_combo.append(Some("button_areas"), "Button Areas (Bottom L/R)");
    click_combo.append(Some("custom"), "Custom...");

    let current_click = config
        .borrow()
        .input
        .as_ref()
        .and_then(|i| i.trackpad.as_ref())
        .and_then(|t| t.click_method.clone())
        .unwrap_or_else(|| "none".to_string());

    let is_predefined = ["none", "clickfinger", "button_areas"].contains(&current_click.as_str());

    if is_predefined {
        click_combo.set_active_id(Some(&current_click));
    } else {
        click_combo.set_active_id(Some("custom"));
    }

    let click_entry = Entry::new();
    click_entry.set_text(&current_click);
    click_entry.set_hexpand(true);
    click_entry.set_visible(!is_predefined);

    let config_clone = config.clone();
    let entry_clone = click_entry.clone();
    click_combo.connect_changed(move |combo| {
        if let Some(id) = combo.active_id() {
            let id_str = id.as_str();
            if id_str == "custom" {
                entry_clone.set_visible(true);
            } else {
                entry_clone.set_visible(false);
                let mut cfg = config_clone.borrow_mut();
                ensure_input_trackpad(&mut cfg);
                cfg.input
                    .as_mut()
                    .unwrap()
                    .trackpad
                    .as_mut()
                    .unwrap()
                    .click_method = Some(id_str.to_string());
            }
        }
    });

    let config_clone = config.clone();
    click_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_trackpad(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .trackpad
            .as_mut()
            .unwrap()
            .click_method = Some(entry.text().to_string());
    });

    click_box.append(&click_combo);
    click_box.append(&click_entry);
    page.append(&click_box);

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

    // Acceleration profile with dropdown
    let profile_box = create_row();
    add_label(&profile_box, "Acceleration profile:", 200);

    let profile_combo = ComboBoxText::new();
    profile_combo.append(Some("adaptive"), "Adaptive");
    profile_combo.append(Some("flat"), "Flat");
    profile_combo.append(Some("custom"), "Custom...");

    let current_profile = config
        .borrow()
        .input
        .as_ref()
        .and_then(|i| i.mouse.as_ref())
        .and_then(|m| m.accel_profile.clone())
        .unwrap_or_else(|| "flat".to_string());

    if current_profile == "adaptive" || current_profile == "flat" {
        profile_combo.set_active_id(Some(&current_profile));
    } else {
        profile_combo.set_active_id(Some("custom"));
    }

    let profile_entry = Entry::new();
    profile_entry.set_text(&current_profile);
    profile_entry.set_hexpand(true);
    profile_entry.set_visible(current_profile != "adaptive" && current_profile != "flat");

    let config_clone = config.clone();
    let entry_clone = profile_entry.clone();
    profile_combo.connect_changed(move |combo| {
        if let Some(id) = combo.active_id() {
            let id_str = id.as_str();
            if id_str == "custom" {
                entry_clone.set_visible(true);
            } else {
                entry_clone.set_visible(false);
                let mut cfg = config_clone.borrow_mut();
                ensure_input_mouse(&mut cfg);
                cfg.input
                    .as_mut()
                    .unwrap()
                    .mouse
                    .as_mut()
                    .unwrap()
                    .accel_profile = Some(id_str.to_string());
            }
        }
    });

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

    profile_box.append(&profile_combo);
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

    let theme_combo = ComboBoxText::new();
    theme_combo.append(Some("Adwaita"), "Adwaita (Default)");
    theme_combo.append(Some("Yaru"), "Yaru");
    theme_combo.append(Some("Breeze_Snow"), "Breeze Snow");
    theme_combo.append(Some("custom"), "Custom...");

    let current_theme = config
        .borrow()
        .cursor
        .as_ref()
        .and_then(|c| c.theme.clone())
        .unwrap_or_else(|| "Adwaita".to_string());

    let is_predefined = ["Adwaita", "Yaru", "Breeze_Snow"].contains(&current_theme.as_str());

    if is_predefined {
        theme_combo.set_active_id(Some(&current_theme));
    } else {
        theme_combo.set_active_id(Some("custom"));
    }

    let theme_entry = Entry::new();
    theme_entry.set_text(&current_theme);
    theme_entry.set_hexpand(true);
    theme_entry.set_visible(!is_predefined);

    let config_clone = config.clone();
    let entry_clone = theme_entry.clone();
    theme_combo.connect_changed(move |combo| {
        if let Some(id) = combo.active_id() {
            let id_str = id.as_str();
            if id_str == "custom" {
                entry_clone.set_visible(true);
            } else {
                entry_clone.set_visible(false);
                let mut cfg = config_clone.borrow_mut();
                if cfg.cursor.is_none() {
                    cfg.cursor = Some(CursorConfig::default());
                }
                cfg.cursor.as_mut().unwrap().theme = Some(id_str.to_string());
            }
        }
    });

    let config_clone = config.clone();
    theme_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.cursor.is_none() {
            cfg.cursor = Some(CursorConfig::default());
        }
        cfg.cursor.as_mut().unwrap().theme = Some(entry.text().to_string());
    });

    theme_box.append(&theme_combo);
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

    // Info section
    let info_label = Label::new(Some(
        "Blur effects are applied to windows with blur enabled in window rules.\nHigher values increase blur quality but may impact performance.",
    ));
    info_label.set_halign(gtk4::Align::Start);
    info_label.add_css_class("dim-label");
    page.append(&info_label);

    add_section_header(&page, "Blur Settings");

    // Blur radius
    let radius_box = create_row();
    add_label(&radius_box, "Blur radius (passes):", 200);
    let radius_spin = SpinButton::new(
        Some(&Adjustment::new(2.0, 0.0, 20.0, 1.0, 2.0, 0.0)),
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
    radius_spin.set_tooltip_text(Some(
        "Number of Kawase down+up passes. Default: 2. Higher = more blur, lower performance.",
    ));

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

    // Blur radius description
    let radius_desc = Label::new(Some(
        "Controls blur intensity through multiple passes (0 = disabled, 2 = default, 10+ = very strong)",
    ));
    radius_desc.set_halign(gtk4::Align::Start);
    radius_desc.set_margin_start(200);
    radius_desc.add_css_class("dim-label");
    radius_desc.add_css_class("caption");
    page.append(&radius_desc);

    // Blur strength
    let strength_box = create_row();
    add_label(&strength_box, "Blur strength (spread):", 200);
    let strength_spin =
        SpinButton::new(Some(&Adjustment::new(1.1, 0.1, 5.0, 0.1, 0.5, 0.0)), 0.1, 2);
    strength_spin.set_value(
        config
            .borrow()
            .effects
            .as_ref()
            .and_then(|e| e.blur_strength)
            .unwrap_or(1.1),
    );
    strength_spin.set_tooltip_text(Some(
        "Per-pass texel spread. Default: 1.1. Higher = wider blur spread per pass.",
    ));

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

    // Blur strength description
    let strength_desc = Label::new(Some(
        "Controls blur spread per pass (0.5 = tight, 1.1 = default, 3.0+ = very wide)",
    ));
    strength_desc.set_halign(gtk4::Align::Start);
    strength_desc.set_margin_start(200);
    strength_desc.add_css_class("dim-label");
    strength_desc.add_css_class("caption");
    page.append(&strength_desc);

    // Presets section
    add_section_header(&page, "Blur Presets");

    let presets_box = Box::new(Orientation::Horizontal, 12);
    presets_box.set_margin_top(6);
    presets_box.set_margin_bottom(12);
    presets_box.set_margin_start(12);
    presets_box.set_margin_end(12);

    // Preset buttons
    let preset_none = Button::with_label("None (0, 0)");
    let preset_light = Button::with_label("Light (1, 0.8)");
    let preset_default = Button::with_label("Default (2, 1.1)");
    let preset_medium = Button::with_label("Medium (4, 1.3)");
    let preset_strong = Button::with_label("Strong (6, 1.5)");
    let preset_extreme = Button::with_label("Extreme (10, 2.0)");

    let radius_clone = radius_spin.clone();
    let strength_clone = strength_spin.clone();
    preset_none.connect_clicked(move |_| {
        radius_clone.set_value(0.0);
        strength_clone.set_value(0.0);
    });

    let radius_clone = radius_spin.clone();
    let strength_clone = strength_spin.clone();
    preset_light.connect_clicked(move |_| {
        radius_clone.set_value(1.0);
        strength_clone.set_value(0.8);
    });

    let radius_clone = radius_spin.clone();
    let strength_clone = strength_spin.clone();
    preset_default.connect_clicked(move |_| {
        radius_clone.set_value(2.0);
        strength_clone.set_value(1.1);
    });

    let radius_clone = radius_spin.clone();
    let strength_clone = strength_spin.clone();
    preset_medium.connect_clicked(move |_| {
        radius_clone.set_value(4.0);
        strength_clone.set_value(1.3);
    });

    let radius_clone = radius_spin.clone();
    let strength_clone = strength_spin.clone();
    preset_strong.connect_clicked(move |_| {
        radius_clone.set_value(6.0);
        strength_clone.set_value(1.5);
    });

    let radius_clone = radius_spin.clone();
    let strength_clone = strength_spin.clone();
    preset_extreme.connect_clicked(move |_| {
        radius_clone.set_value(10.0);
        strength_clone.set_value(2.0);
    });

    presets_box.append(&preset_none);
    presets_box.append(&preset_light);
    presets_box.append(&preset_default);
    presets_box.append(&preset_medium);
    presets_box.append(&preset_strong);
    presets_box.append(&preset_extreme);
    page.append(&presets_box);

    // Usage note
    let usage_label = Label::new(Some(
        "Note: To enable blur for specific windows, add window rules with blur = true and opacity < 1.0",
    ));
    usage_label.set_halign(gtk4::Align::Start);
    usage_label.set_margin_top(12);
    usage_label.add_css_class("dim-label");
    usage_label.add_css_class("caption");
    page.append(&usage_label);

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

    // Quick access to Shader Editor
    let shader_editor_box = Box::new(Orientation::Horizontal, 12);
    shader_editor_box.set_margin_top(24);

    let shader_editor_btn = Button::with_label("Open Shader Editor →");
    shader_editor_btn.add_css_class("suggested-action");
    shader_editor_btn.set_tooltip_text(Some(
        "Create custom animated backgrounds with visual controls",
    ));

    let stack_clone = stack.clone();
    shader_editor_btn.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("shader_editor");
    });

    shader_editor_box.append(&shader_editor_btn);
    page.append(&shader_editor_box);

    stack.add_titled(&page, Some("background"), "Background");
}

fn add_window_rules_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Window Rules");

    let info_label = Label::new(Some(
        "Configure per-window settings like blur, opacity, position, and decorations.\nFind app_id: cat $XDG_RUNTIME_DIR/driftwm/state",
    ));
    info_label.set_halign(gtk4::Align::Start);
    info_label.add_css_class("dim-label");
    page.append(&info_label);

    // Scrolled window for rules list
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_min_content_height(400);

    let rules_container = Box::new(Orientation::Vertical, 12);
    rules_container.set_margin_top(12);
    rules_container.set_margin_bottom(12);
    rules_container.set_margin_start(12);
    rules_container.set_margin_end(12);

    scrolled.set_child(Some(&rules_container));

    // Load existing rules
    let existing_rules = config.borrow().window_rules.clone().unwrap_or_default();

    for (idx, rule) in existing_rules.iter().enumerate() {
        add_window_rule_row(&rules_container, config.clone(), idx, rule.clone());
    }

    page.append(&scrolled);

    // Add button
    let add_button = Button::with_label("+ Add Window Rule");
    add_button.set_halign(gtk4::Align::Start);
    add_button.add_css_class("suggested-action");

    let rules_container_clone = rules_container.clone();
    let config_clone = config.clone();
    add_button.connect_clicked(move |_| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.window_rules.is_none() {
            cfg.window_rules = Some(Vec::new());
        }
        let new_rule = WindowRule::default();
        cfg.window_rules.as_mut().unwrap().push(new_rule.clone());
        let idx = cfg.window_rules.as_ref().unwrap().len() - 1;
        drop(cfg);

        add_window_rule_row(&rules_container_clone, config_clone.clone(), idx, new_rule);
    });

    page.append(&add_button);

    stack.add_titled(&page, Some("window_rules"), "Window Rules");
}

fn add_window_rule_row(
    container: &Box,
    config: Rc<RefCell<DriftwmConfig>>,
    idx: usize,
    rule: WindowRule,
) {
    let rule_frame = gtk4::Frame::new(None);
    rule_frame.add_css_class("card");

    let rule_box = Box::new(Orientation::Vertical, 6);
    rule_box.set_margin_top(12);
    rule_box.set_margin_bottom(12);
    rule_box.set_margin_start(12);
    rule_box.set_margin_end(12);

    // Header with delete button
    let header_box = Box::new(Orientation::Horizontal, 12);
    let rule_label = Label::new(Some(&format!("Rule #{}", idx + 1)));
    rule_label.add_css_class("heading");
    rule_label.set_halign(gtk4::Align::Start);
    rule_label.set_hexpand(true);
    header_box.append(&rule_label);

    let delete_btn = Button::with_label("Delete");
    delete_btn.add_css_class("destructive-action");
    let config_clone = config.clone();
    let frame_clone = rule_frame.clone();
    delete_btn.connect_clicked(move |_| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut() {
            if idx < rules.len() {
                rules.remove(idx);
            }
        }
        drop(cfg);
        frame_clone.set_visible(false);
    });
    header_box.append(&delete_btn);
    rule_box.append(&header_box);

    // App ID
    let app_id_box = create_row();
    add_label(&app_id_box, "App ID:", 150);
    let app_id_entry = Entry::new();
    app_id_entry.set_text(&rule.app_id.clone().unwrap_or_default());
    app_id_entry.set_placeholder_text(Some("e.g., Alacritty, firefox"));
    app_id_entry.set_hexpand(true);
    let config_clone = config.clone();
    app_id_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut() {
            if let Some(rule) = rules.get_mut(idx) {
                let text = entry.text().to_string();
                rule.app_id = if text.is_empty() { None } else { Some(text) };
            }
        }
    });
    app_id_box.append(&app_id_entry);
    rule_box.append(&app_id_box);

    // Title
    let title_box = create_row();
    add_label(&title_box, "Title:", 150);
    let title_entry = Entry::new();
    title_entry.set_text(&rule.title.clone().unwrap_or_default());
    title_entry.set_placeholder_text(Some("Window title (supports * glob)"));
    title_entry.set_hexpand(true);
    let config_clone = config.clone();
    title_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut() {
            if let Some(rule) = rules.get_mut(idx) {
                let text = entry.text().to_string();
                rule.title = if text.is_empty() { None } else { Some(text) };
            }
        }
    });
    title_box.append(&title_entry);
    rule_box.append(&title_box);

    // Blur
    let blur_box = create_row();
    add_label(&blur_box, "Enable blur:", 150);
    let blur_switch = Switch::new();
    blur_switch.set_active(rule.blur.unwrap_or(false));
    let config_clone = config.clone();
    blur_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut() {
            if let Some(rule) = rules.get_mut(idx) {
                rule.blur = Some(state);
            }
        }
        gtk4::glib::Propagation::Proceed
    });
    blur_box.append(&blur_switch);
    rule_box.append(&blur_box);

    // Opacity
    let opacity_box = create_row();
    add_label(&opacity_box, "Opacity:", 150);
    let opacity_spin = SpinButton::new(
        Some(&Adjustment::new(1.0, 0.0, 1.0, 0.05, 0.1, 0.0)),
        0.05,
        2,
    );
    opacity_spin.set_value(rule.opacity.unwrap_or(1.0));
    let config_clone = config.clone();
    opacity_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut() {
            if let Some(rule) = rules.get_mut(idx) {
                rule.opacity = Some(spin.value());
            }
        }
    });
    opacity_box.append(&opacity_spin);
    rule_box.append(&opacity_box);

    // Decoration
    let decoration_box = create_row();
    add_label(&decoration_box, "Decoration:", 150);
    let decoration_combo = ComboBoxText::new();
    decoration_combo.append(Some("client"), "Client");
    decoration_combo.append(Some("server"), "Server");
    decoration_combo.append(Some("none"), "None (Borderless)");
    if let Some(dec) = &rule.decoration {
        decoration_combo.set_active_id(Some(dec));
    }
    let config_clone = config.clone();
    decoration_combo.connect_changed(move |combo| {
        if let Some(id) = combo.active_id() {
            let mut cfg = config_clone.borrow_mut();
            if let Some(rules) = cfg.window_rules.as_mut() {
                if let Some(rule) = rules.get_mut(idx) {
                    rule.decoration = Some(id.to_string());
                }
            }
        }
    });
    decoration_box.append(&decoration_combo);
    rule_box.append(&decoration_box);

    // Widget
    let widget_box = create_row();
    add_label(&widget_box, "Widget (pinned):", 150);
    let widget_switch = Switch::new();
    widget_switch.set_active(rule.widget.unwrap_or(false));
    let config_clone = config.clone();
    widget_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut() {
            if let Some(rule) = rules.get_mut(idx) {
                rule.widget = Some(state);
            }
        }
        gtk4::glib::Propagation::Proceed
    });
    widget_box.append(&widget_switch);
    rule_box.append(&widget_box);

    rule_frame.set_child(Some(&rule_box));
    container.append(&rule_frame);
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

fn add_backend_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Backend Configuration");

    let info_label = Label::new(Some(
        "Hardware stability quirks. Enable if you experience flickering or crashes.\nParticularly useful on NVIDIA GPUs.",
    ));
    info_label.set_halign(gtk4::Align::Start);
    info_label.add_css_class("dim-label");
    page.append(&info_label);

    add_section_header(&page, "DRM Settings");

    let force_legacy = add_switch_row(
        &page,
        "Force Legacy DRM",
        "Use legacy DRM API instead of atomic modesetting",
        config
            .borrow()
            .backend
            .as_ref()
            .and_then(|b| b.force_legacy_drm)
            .unwrap_or(false),
    );

    let config_clone = config.clone();
    force_legacy.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.backend.is_none() {
            cfg.backend = Some(Default::default());
        }
        cfg.backend.as_mut().unwrap().force_legacy_drm = Some(state);
        glib::Propagation::Proceed
    });

    let wait_frame = add_switch_row(
        &page,
        "Wait for Frame Completion",
        "Wait for GPU fences before page flip",
        config
            .borrow()
            .backend
            .as_ref()
            .and_then(|b| b.wait_for_frame_completion)
            .unwrap_or(false),
    );

    let config_clone = config.clone();
    wait_frame.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.backend.is_none() {
            cfg.backend = Some(Default::default());
        }
        cfg.backend.as_mut().unwrap().wait_for_frame_completion = Some(state);
        glib::Propagation::Proceed
    });

    let disable_scanout = add_switch_row(
        &page,
        "Disable Direct Scanout",
        "Force EGL composition (disable direct scanout)",
        config
            .borrow()
            .backend
            .as_ref()
            .and_then(|b| b.disable_direct_scanout)
            .unwrap_or(false),
    );

    let config_clone = config.clone();
    disable_scanout.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.backend.is_none() {
            cfg.backend = Some(Default::default());
        }
        cfg.backend.as_mut().unwrap().disable_direct_scanout = Some(state);
        glib::Propagation::Proceed
    });

    stack.add_titled(&page, Some("backend"), "Backend");
}

fn add_keybindings_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Keybindings");

    let info_label = Label::new(Some(
        "Configure custom keyboard shortcuts. Click + to add new bindings.",
    ));
    info_label.set_halign(gtk4::Align::Start);
    info_label.add_css_class("dim-label");
    page.append(&info_label);

    // Scrolled window for keybindings list
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_min_content_height(400);

    let bindings_box = Box::new(Orientation::Vertical, 6);
    scrolled.set_child(Some(&bindings_box));

    // Load existing keybindings
    let existing_bindings = config.borrow().keybindings.clone().unwrap_or_default();

    for (key, action) in existing_bindings.iter() {
        add_keybinding_row(
            &bindings_box,
            config.clone(),
            Some(key.clone()),
            Some(action.clone()),
        );
    }

    page.append(&scrolled);

    // Add button
    let add_button = Button::with_label("+ Add Keybinding");
    add_button.set_halign(gtk4::Align::Start);
    add_button.add_css_class("suggested-action");

    let bindings_box_clone = bindings_box.clone();
    let config_clone = config.clone();
    add_button.connect_clicked(move |_| {
        add_keybinding_row(&bindings_box_clone, config_clone.clone(), None, None);
    });

    page.append(&add_button);

    stack.add_titled(&page, Some("keybindings"), "Keybindings");
}

fn add_keybinding_row(
    container: &Box,
    config: Rc<RefCell<DriftwmConfig>>,
    initial_key: Option<String>,
    initial_action: Option<String>,
) {
    let row = Box::new(Orientation::Horizontal, 12);
    row.set_margin_top(6);
    row.set_margin_bottom(6);

    // Key entry
    let key_entry = Entry::new();
    key_entry.set_placeholder_text(Some("super+t"));
    key_entry.set_width_chars(20);
    if let Some(key) = initial_key.clone() {
        key_entry.set_text(&key);
    }

    // Record button
    let record_button = Button::with_label("⏺");
    record_button.set_tooltip_text(Some("Click and press keys to record"));

    let key_entry_clone = key_entry.clone();
    let record_button_clone = record_button.clone();
    record_button.connect_clicked(move |btn| {
        btn.set_label("Press keys...");
        btn.add_css_class("accent");

        // Create event controller for key press
        let key_controller = gtk4::EventControllerKey::new();
        let btn_clone = record_button_clone.clone();
        let entry_clone = key_entry_clone.clone();

        key_controller.connect_key_pressed(move |controller, keyval, _keycode, state| {
            let mut parts = Vec::new();

            // Add modifiers
            if state.contains(gtk4::gdk::ModifierType::CONTROL_MASK) {
                parts.push("ctrl");
            }
            if state.contains(gtk4::gdk::ModifierType::ALT_MASK) {
                parts.push("alt");
            }
            if state.contains(gtk4::gdk::ModifierType::SHIFT_MASK) {
                parts.push("shift");
            }
            if state.contains(gtk4::gdk::ModifierType::SUPER_MASK) {
                parts.push("super");
            }

            // Add key name
            if let Some(name) = keyval.name() {
                let key_name = name.to_lowercase();
                // Skip modifier keys themselves
                if ![
                    "control_l",
                    "control_r",
                    "alt_l",
                    "alt_r",
                    "shift_l",
                    "shift_r",
                    "super_l",
                    "super_r",
                ]
                .contains(&key_name.as_str())
                {
                    parts.push(&key_name);

                    // Set the combo
                    let combo = parts.join("+");
                    entry_clone.set_text(&combo);

                    // Reset button
                    btn_clone.set_label("⏺");
                    btn_clone.remove_css_class("accent");

                    // Remove controller
                    if let Some(widget) = controller.widget() {
                        widget.remove_controller(controller);
                    }
                }
            }

            glib::Propagation::Stop
        });

        btn.add_controller(key_controller);
    });

    // Action entry
    let action_entry = Entry::new();
    action_entry.set_placeholder_text(Some("exec alacritty"));
    action_entry.set_hexpand(true);
    if let Some(action) = initial_action.clone() {
        action_entry.set_text(&action);
    }

    // Remove button
    let remove_button = Button::with_label("−");
    remove_button.add_css_class("destructive-action");

    let row_clone = row.clone();
    let config_clone = config.clone();
    let key_entry_clone = key_entry.clone();
    remove_button.connect_clicked(move |_| {
        // Remove from config
        let key_text = key_entry_clone.text().to_string();
        if !key_text.is_empty() {
            let mut cfg = config_clone.borrow_mut();
            if let Some(ref mut kb) = cfg.keybindings {
                kb.remove(&key_text);
                if kb.is_empty() {
                    cfg.keybindings = None;
                }
            }
        }
        // Remove from UI
        if let Some(parent) = row_clone.parent() {
            parent.downcast::<Box>().unwrap().remove(&row_clone);
        }
    });

    // Update config when entries change
    let config_clone = config.clone();
    let key_entry_clone = key_entry.clone();
    let action_entry_clone = action_entry.clone();
    let old_key = Rc::new(RefCell::new(initial_key.unwrap_or_default()));

    let update_binding = move || {
        let key_text = key_entry_clone.text().to_string().trim().to_string();
        let action_text = action_entry_clone.text().to_string().trim().to_string();

        let mut cfg = config_clone.borrow_mut();

        // Remove old key if it changed
        let old_key_val = old_key.borrow().clone();
        if !old_key_val.is_empty() && old_key_val != key_text {
            if let Some(ref mut kb) = cfg.keybindings {
                kb.remove(&old_key_val);
            }
        }

        // Add/update new binding
        if !key_text.is_empty() && !action_text.is_empty() {
            if cfg.keybindings.is_none() {
                cfg.keybindings = Some(std::collections::HashMap::new());
            }
            cfg.keybindings
                .as_mut()
                .unwrap()
                .insert(key_text.clone(), action_text);
            *old_key.borrow_mut() = key_text;
        }
    };

    let update_clone = Rc::new(update_binding);
    let update_clone2 = update_clone.clone();

    key_entry.connect_changed(move |_| update_clone());
    action_entry.connect_changed(move |_| update_clone2());

    row.append(&key_entry);
    row.append(&record_button);
    row.append(&action_entry);
    row.append(&remove_button);

    container.append(&row);
}

// Action entry

fn get_config_path() -> PathBuf {
    let config_home = std::env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| format!("{}/.config", std::env::var("HOME").unwrap()));
    PathBuf::from(config_home).join("driftwm/config.toml")
}
