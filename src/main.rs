mod app_config;
mod config;
mod config_helpers;
mod i18n;
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
    let app_settings = app_config::AppSettings::load().unwrap_or_default();
    let lang = app_settings.language.clone();

    // Apply theme
    if let Some(display) = gtk4::gdk::Display::default() {
        let gtk_settings = gtk4::Settings::for_display(&display);
        println!("Applying theme from config: {}", app_settings.theme);
        match app_settings.theme.as_str() {
            "light" => {
                gtk_settings.set_property("gtk-application-prefer-dark-theme", false);
            }
            "dark" => {
                gtk_settings.set_property("gtk-application-prefer-dark-theme", true);
            }
            _ => {
                // System default: we don't force it
                gtk_settings.reset_property("gtk-application-prefer-dark-theme");
            }
        }
    }

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
    stack.set_vhomogeneous(false);
    stack.set_hhomogeneous(false);

    // Sidebar
    let sidebar = StackSidebar::new();
    sidebar.set_stack(&stack);
    sidebar.set_width_request(180);

    // Add pages to stack with icons
    add_general_page(&stack, config_rc.clone(), &lang);
    add_keyboard_page(&stack, config_rc.clone(), &lang);
    add_trackpad_page(&stack, config_rc.clone(), &lang);
    add_mouse_page(&stack, config_rc.clone(), &lang);
    add_cursor_page(&stack, config_rc.clone(), &lang);
    add_navigation_page(&stack, config_rc.clone(), &lang);
    add_zoom_page(&stack, config_rc.clone(), &lang);
    add_snap_page(&stack, config_rc.clone(), &lang);
    add_decorations_page(&stack, config_rc.clone(), &lang);
    add_effects_page(&stack, config_rc.clone(), &lang);
    add_window_rules_page(&stack, config_rc.clone(), &lang);
    add_backend_page(&stack, config_rc.clone(), &lang);
    add_background_page(&stack, config_rc.clone(), &lang);
    add_shader_editor_page(&stack, config_rc.clone(), &lang);
    add_keybindings_page(&stack, config_rc.clone(), &lang);
    add_autostart_page(&stack, config_rc.clone(), &lang);
    add_app_settings_page(&stack, &lang);

    // Scrolled window for stack content
    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&stack));
    scrolled.set_hexpand(true);
    scrolled.set_vexpand(true);

    // Reset scroll position when switching pages
    let scrolled_clone = scrolled.clone();
    stack.connect_visible_child_notify(move |_| {
        // Reset scroll to top when page changes
        scrolled_clone.vadjustment().set_value(0.0);
        scrolled_clone.hadjustment().set_value(0.0);
    });

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
    let save_button = Button::with_label(&crate::i18n::t("Save", lang));
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
        .default_width(900)
        .default_height(650)
        .child(&vbox)
        .build();

    window.set_titlebar(Some(&header));
    window.present();
}

fn add_general_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("General Settings", lang));

    // Modifier key
    let mod_box = create_row();
    add_label(&mod_box, &crate::i18n::t("Modifier key:", lang), 200);

    let mod_combo = ComboBoxText::new();
    mod_combo.append(Some("super"), &crate::i18n::t("Super (Windows key)", lang));
    mod_combo.append(Some("alt"), &crate::i18n::t("Alt", lang));
    mod_combo.append(Some("custom"), &crate::i18n::t("Custom...", lang));

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
    mod_entry.set_placeholder_text(Some(&crate::i18n::t("e.g., ctrl, shift", lang)));

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
    add_label(&focus_box, &crate::i18n::t("Focus follows mouse:", lang), 200);
    let focus_switch = Switch::new();
    focus_switch.set_active(config.borrow().focus_follows_mouse.unwrap_or(false));

    let config_clone = config.clone();
    focus_switch.connect_state_set(move |_, state| {
        config_clone.borrow_mut().focus_follows_mouse = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    focus_box.append(&focus_switch);
    page.append(&focus_box);

    stack.add_titled(&page, Some("general"), &crate::i18n::t("General", lang));
}

fn add_keyboard_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Keyboard Settings", lang));

    // Layout
    let layout_box = create_row();
    add_label(&layout_box, &crate::i18n::t("Layout:", lang), 200);
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
    layout_entry.set_placeholder_text(Some(&crate::i18n::t("us,ru", lang)));
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
    add_label(&variant_box, &crate::i18n::t("Variant:", lang), 200);
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
    variant_entry.set_placeholder_text(Some(&crate::i18n::t("dvorak", lang)));
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
    add_label(&options_box, &crate::i18n::t("Options:", lang), 200);

    let options_combo = ComboBoxText::new();
    options_combo.append(Some("grp:win_space_toggle"), &crate::i18n::t("Super+Space to switch layout", lang));
    options_combo.append(Some("grp:alt_shift_toggle"), &crate::i18n::t("Alt+Shift to switch layout", lang));
    options_combo.append(Some("grp:ctrl_shift_toggle"), &crate::i18n::t("Ctrl+Shift to switch layout", lang));
    options_combo.append(Some("caps:escape"), &crate::i18n::t("Caps Lock as Escape", lang));
    options_combo.append(Some("caps:ctrl_modifier"), &crate::i18n::t("Caps Lock as Ctrl", lang));
    options_combo.append(Some("custom"), &crate::i18n::t("Custom...", lang));

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
    add_label(&rate_box, &crate::i18n::t("Repeat rate (keys/sec):", lang), 200);
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
    add_label(&delay_box, &crate::i18n::t("Repeat delay (ms):", lang), 200);
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

    // Layout independent keybindings
    let independent_box = create_row();
    add_label(&independent_box, &crate::i18n::t("Layout independent keybindings:", lang), 200);
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
    independent_switch.set_tooltip_text(Some(
        "Match bindings by physical key position across layouts",
    ));

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

    // Num Lock
    let num_lock_box = create_row();
    add_label(&num_lock_box, &crate::i18n::t("Num Lock on startup:", lang), 200);
    let num_lock_switch = Switch::new();
    num_lock_switch.set_active(
        config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.keyboard.as_ref())
            .and_then(|k| k.num_lock)
            .unwrap_or(true),
    );

    let config_clone = config.clone();
    num_lock_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_keyboard(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .keyboard
            .as_mut()
            .unwrap()
            .num_lock = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    num_lock_box.append(&num_lock_switch);
    page.append(&num_lock_box);

    // Caps Lock
    let caps_lock_box = create_row();
    add_label(&caps_lock_box, &crate::i18n::t("Caps Lock on startup:", lang), 200);
    let caps_lock_switch = Switch::new();
    caps_lock_switch.set_active(
        config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.keyboard.as_ref())
            .and_then(|k| k.caps_lock)
            .unwrap_or(false),
    );

    let config_clone = config.clone();
    caps_lock_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_keyboard(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .keyboard
            .as_mut()
            .unwrap()
            .caps_lock = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    caps_lock_box.append(&caps_lock_switch);
    page.append(&caps_lock_box);

    stack.add_titled(&page, Some("keyboard"), &crate::i18n::t("Keyboard", lang));
}

fn add_trackpad_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Trackpad Settings", lang));

    // Tap to click
    let tap_box = create_row();
    add_label(&tap_box, &crate::i18n::t("Tap to click:", lang), 200);
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
    add_label(&natural_box, &crate::i18n::t("Natural scroll:", lang), 200);
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
    add_label(&drag_box, &crate::i18n::t("Tap and drag:", lang), 200);
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
    add_label(&accel_box, &crate::i18n::t("Acceleration speed:", lang), 200);
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
    add_label(&profile_box, &crate::i18n::t("Acceleration profile:", lang), 200);

    let profile_combo = ComboBoxText::new();
    profile_combo.append(Some("adaptive"), &crate::i18n::t("Adaptive", lang));
    profile_combo.append(Some("flat"), &crate::i18n::t("Flat", lang));
    profile_combo.append(Some("custom"), &crate::i18n::t("Custom...", lang));

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
    add_label(&click_box, &crate::i18n::t("Click method:", lang), 200);

    let click_combo = ComboBoxText::new();
    click_combo.append(Some("none"), &crate::i18n::t("Device Default", lang));
    click_combo.append(Some("clickfinger"), &crate::i18n::t("Clickfinger (1=L, 2=R, 3=M)", lang));
    click_combo.append(Some("button_areas"), &crate::i18n::t("Button Areas (Bottom L/R)", lang));
    click_combo.append(Some("custom"), &crate::i18n::t("Custom...", lang));

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

    stack.add_titled(&page, Some("trackpad"), &crate::i18n::t("Trackpad", lang));
}

fn add_mouse_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Mouse Settings", lang));

    // Acceleration speed
    let accel_box = create_row();
    add_label(&accel_box, &crate::i18n::t("Acceleration speed:", lang), 200);
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
    add_label(&profile_box, &crate::i18n::t("Acceleration profile:", lang), 200);

    let profile_combo = ComboBoxText::new();
    profile_combo.append(Some("adaptive"), &crate::i18n::t("Adaptive", lang));
    profile_combo.append(Some("flat"), &crate::i18n::t("Flat", lang));
    profile_combo.append(Some("custom"), &crate::i18n::t("Custom...", lang));

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
    add_label(&natural_box, &crate::i18n::t("Natural scroll:", lang), 200);
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

    // Snapped decoration resize
    let resize_snapped_box = create_row();
    add_label(&resize_snapped_box, &crate::i18n::t("Cluster-aware resize:", lang), 200);
    let resize_snapped_switch = Switch::new();
    resize_snapped_switch.set_active(
        config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.mouse.as_ref())
            .and_then(|m| m.decoration_resize_snapped)
            .unwrap_or(false),
    );

    let config_clone = config.clone();
    resize_snapped_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_mouse(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .mouse
            .as_mut()
            .unwrap()
            .decoration_resize_snapped = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    resize_snapped_box.append(&resize_snapped_switch);
    page.append(&resize_snapped_box);

    // Snapped decoration fit
    let fit_snapped_box = create_row();
    add_label(&fit_snapped_box, &crate::i18n::t("Cluster-aware fit (maximize):", lang), 200);
    let fit_snapped_switch = Switch::new();
    fit_snapped_switch.set_active(
        config
            .borrow()
            .input
            .as_ref()
            .and_then(|i| i.mouse.as_ref())
            .and_then(|m| m.decoration_fit_snapped)
            .unwrap_or(false),
    );

    let config_clone = config.clone();
    fit_snapped_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        ensure_input_mouse(&mut cfg);
        cfg.input
            .as_mut()
            .unwrap()
            .mouse
            .as_mut()
            .unwrap()
            .decoration_fit_snapped = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    fit_snapped_box.append(&fit_snapped_switch);
    page.append(&fit_snapped_box);

    stack.add_titled(&page, Some("mouse"), &crate::i18n::t("Mouse", lang));
}

fn add_cursor_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Cursor Settings", lang));

    // Theme
    let theme_box = create_row();
    add_label(&theme_box, &crate::i18n::t("Cursor theme:", lang), 200);

    let theme_combo = ComboBoxText::new();
    theme_combo.append(Some("Adwaita"), &crate::i18n::t("Adwaita (Default)", lang));
    theme_combo.append(Some("Yaru"), &crate::i18n::t("Yaru", lang));
    theme_combo.append(Some("Breeze_Snow"), &crate::i18n::t("Breeze Snow", lang));
    theme_combo.append(Some("custom"), &crate::i18n::t("Custom...", lang));

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
    add_label(&size_box, &crate::i18n::t("Cursor size:", lang), 200);
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
    add_label(&opacity_box, &crate::i18n::t("Inactive opacity:", lang), 200);
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

    stack.add_titled(&page, Some("cursor"), &crate::i18n::t("Cursor", lang));
}

fn add_navigation_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Navigation Settings", lang));

    // Trackpad speed
    let trackpad_speed_box = create_row();
    add_label(&trackpad_speed_box, &crate::i18n::t("Trackpad speed:", lang), 200);
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
    add_label(&mouse_speed_box, &crate::i18n::t("Mouse speed:", lang), 200);
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
    add_label(&friction_box, &crate::i18n::t("Friction:", lang), 200);
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
    add_label(&anim_box, &crate::i18n::t("Animation speed:", lang), 200);
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
    add_label(&nudge_box, &crate::i18n::t("Nudge step (px):", lang), 200);
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
    add_label(&pan_box, &crate::i18n::t("Pan step (px):", lang), 200);
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

    stack.add_titled(&page, Some("navigation"), &crate::i18n::t("Navigation", lang));
}

fn add_zoom_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Zoom Settings", lang));

    // Zoom step
    let step_box = create_row();
    add_label(&step_box, &crate::i18n::t("Zoom step:", lang), 200);
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
    add_label(&padding_box, &crate::i18n::t("Fit padding (px):", lang), 200);
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

    // Reset on new window
    let reset_new_box = create_row();
    add_label(&reset_new_box, &crate::i18n::t("Reset zoom on new window:", lang), 200);
    let reset_new_switch = Switch::new();
    reset_new_switch.set_active(
        config
            .borrow()
            .zoom
            .as_ref()
            .and_then(|z| z.reset_on_new_window)
            .unwrap_or(true),
    );
    reset_new_switch.set_tooltip_text(Some(&crate::i18n::t("Animate zoom to 1.0 when a new window is mapped", lang)));

    let config_clone = config.clone();
    reset_new_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.zoom.is_none() {
            cfg.zoom = Some(ZoomConfig::default());
        }
        cfg.zoom.as_mut().unwrap().reset_on_new_window = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    reset_new_box.append(&reset_new_switch);
    page.append(&reset_new_box);

    // Reset on activation
    let reset_activation_box = create_row();
    add_label(&reset_activation_box, &crate::i18n::t("Reset zoom on activation:", lang), 200);
    let reset_activation_switch = Switch::new();
    reset_activation_switch.set_active(
        config
            .borrow()
            .zoom
            .as_ref()
            .and_then(|z| z.reset_on_activation)
            .unwrap_or(true),
    );
    reset_activation_switch.set_tooltip_text(Some(
        "Animate zoom to 1.0 when an off-screen window requests focus",
    ));

    let config_clone = config.clone();
    reset_activation_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.zoom.is_none() {
            cfg.zoom = Some(ZoomConfig::default());
        }
        cfg.zoom.as_mut().unwrap().reset_on_activation = Some(state);
        gtk4::glib::Propagation::Proceed
    });

    reset_activation_box.append(&reset_activation_switch);
    page.append(&reset_activation_box);

    stack.add_titled(&page, Some("zoom"), &crate::i18n::t("Zoom", lang));
}

fn add_snap_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Snap Settings", lang));

    // Enabled
    let enabled_box = create_row();
    add_label(&enabled_box, &crate::i18n::t("Enable snapping:", lang), 200);
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
    add_label(&gap_box, &crate::i18n::t("Gap (px):", lang), 200);
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
    add_label(&distance_box, &crate::i18n::t("Distance (px):", lang), 200);
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
    add_label(&break_box, &crate::i18n::t("Break force (px):", lang), 200);
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
    add_label(&same_edge_box, &crate::i18n::t("Snap same edges:", lang), 200);
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

    stack.add_titled(&page, Some("snap"), &crate::i18n::t("Snap", lang));
}

fn add_decorations_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Window Decorations", lang));

    // Background color
    let bg_box = create_row();
    add_label(&bg_box, &crate::i18n::t("Background color:", lang), 200);
    let bg_entry = Entry::new();
    bg_entry.set_text(
        &config
            .borrow()
            .decorations
            .as_ref()
            .and_then(|d| d.bg_color.clone())
            .unwrap_or_else(|| "#303030".to_string()),
    );
    bg_entry.set_placeholder_text(Some(&crate::i18n::t("#303030", lang)));
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
    add_label(&fg_box, &crate::i18n::t("Foreground color:", lang), 200);
    let fg_entry = Entry::new();
    fg_entry.set_text(
        &config
            .borrow()
            .decorations
            .as_ref()
            .and_then(|d| d.fg_color.clone())
            .unwrap_or_else(|| "#FFFFFF".to_string()),
    );
    fg_entry.set_placeholder_text(Some(&crate::i18n::t("#FFFFFF", lang)));
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
    add_label(&radius_box, &crate::i18n::t("Corner radius:", lang), 200);
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

    // Default mode
    let mode_box = create_row();
    add_label(&mode_box, &crate::i18n::t("Default mode:", lang), 200);
    let mode_combo = ComboBoxText::new();
    mode_combo.append(Some("client"), &crate::i18n::t("Client (CSD)", lang));
    mode_combo.append(Some("borderless"), &crate::i18n::t("Borderless (SSD)", lang));
    mode_combo.append(Some("none"), &crate::i18n::t("None (SSD)", lang));

    let current_mode = config
        .borrow()
        .decorations
        .as_ref()
        .and_then(|d| d.default_mode.clone())
        .unwrap_or_else(|| "client".to_string());
    mode_combo.set_active_id(Some(&current_mode));

    let config_clone = config.clone();
    mode_combo.connect_changed(move |combo| {
        if let Some(id) = combo.active_id() {
            let mut cfg = config_clone.borrow_mut();
            if cfg.decorations.is_none() {
                cfg.decorations = Some(DecorationsConfig::default());
            }
            cfg.decorations.as_mut().unwrap().default_mode = Some(id.to_string());
        }
    });

    mode_box.append(&mode_combo);
    page.append(&mode_box);

    stack.add_titled(&page, Some("decorations"), &crate::i18n::t("Decorations", lang));
}

fn add_effects_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Visual Effects", lang));

    // Info section
    let info_label = Label::new(Some(&crate::i18n::t(
        "Blur effects are applied to windows with blur enabled in window rules.\nHigher values increase blur quality but may impact performance.",
        lang,
    )));
    info_label.set_halign(gtk4::Align::Start);
    info_label.add_css_class("dim-label");
    page.append(&info_label);

    add_section_header(&page, &crate::i18n::t("Blur Settings", lang));

    // Blur radius
    let radius_box = create_row();
    add_label(&radius_box, &crate::i18n::t("Blur radius (passes):", lang), 200);
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
    let radius_desc = Label::new(Some(&crate::i18n::t(
        "Controls blur intensity through multiple passes (0 = disabled, 2 = default, 10+ = very strong)",
        lang,
    )));
    radius_desc.set_halign(gtk4::Align::Start);
    radius_desc.set_margin_start(200);
    radius_desc.add_css_class("dim-label");
    radius_desc.add_css_class("caption");
    page.append(&radius_desc);

    // Blur strength
    let strength_box = create_row();
    add_label(&strength_box, &crate::i18n::t("Blur strength (spread):", lang), 200);
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
    let strength_desc = Label::new(Some(&crate::i18n::t(
        "Controls blur spread per pass (0.5 = tight, 1.1 = default, 3.0+ = very wide)",
        lang,
    )));
    strength_desc.set_halign(gtk4::Align::Start);
    strength_desc.set_margin_start(200);
    strength_desc.add_css_class("dim-label");
    strength_desc.add_css_class("caption");
    page.append(&strength_desc);

    // Presets section
    add_section_header(&page, &crate::i18n::t("Blur Presets", lang));

    let presets_box = Box::new(Orientation::Horizontal, 12);
    presets_box.set_margin_top(6);
    presets_box.set_margin_bottom(12);
    presets_box.set_margin_start(12);
    presets_box.set_margin_end(12);

    // Preset buttons
    let preset_none = Button::with_label(&crate::i18n::t("None (0, 0)", lang));
    let preset_light = Button::with_label(&crate::i18n::t("Light (1, 0.8)", lang));
    let preset_default = Button::with_label(&crate::i18n::t("Default (2, 1.1)", lang));
    let preset_medium = Button::with_label(&crate::i18n::t("Medium (4, 1.3)", lang));
    let preset_strong = Button::with_label(&crate::i18n::t("Strong (6, 1.5)", lang));
    let preset_extreme = Button::with_label(&crate::i18n::t("Extreme (10, 2.0)", lang));

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
    let usage_label = Label::new(Some(&crate::i18n::t(
        "Note: To enable blur for specific windows, add window rules with blur = true and opacity < 1.0",
        lang,
    )));
    usage_label.set_halign(gtk4::Align::Start);
    usage_label.set_margin_top(12);
    usage_label.add_css_class("dim-label");
    usage_label.add_css_class("caption");
    page.append(&usage_label);

    stack.add_titled(&page, Some("effects"), &crate::i18n::t("Effects", lang));
}

fn add_background_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Background", lang));

    // Shader path
    let shader_box = create_row();
    add_label(&shader_box, &crate::i18n::t("Shader path:", lang), 200);
    let shader_entry = Entry::new();
    shader_entry.set_text(
        &config
            .borrow()
            .background
            .as_ref()
            .and_then(|b| b.shader_path.clone())
            .unwrap_or_default(),
    );
    shader_entry.set_placeholder_text(Some(&crate::i18n::t("~/.config/driftwm/bg.frag", lang)));
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
    add_label(&tile_box, &crate::i18n::t("Tile path:", lang), 200);
    let tile_entry = Entry::new();
    tile_entry.set_text(
        &config
            .borrow()
            .background
            .as_ref()
            .and_then(|b| b.tile_path.clone())
            .unwrap_or_default(),
    );
    tile_entry.set_placeholder_text(Some(&crate::i18n::t("~/.config/driftwm/tile.png", lang)));
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

    let shader_editor_btn = Button::with_label(&crate::i18n::t("Open Shader Editor →", lang));
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

    stack.add_titled(&page, Some("background"), &crate::i18n::t("Background", lang));
}

fn add_window_rules_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Window Rules", lang));

    let info_label = Label::new(Some(&crate::i18n::t(
        "Configure per-window settings like blur, opacity, position, and decorations.\nFind app_id: cat $XDG_RUNTIME_DIR/driftwm/state",
        lang,
    )));
    info_label.set_halign(gtk4::Align::Start);
    info_label.add_css_class("dim-label");
    page.append(&info_label);

    // Rules container
    let rules_container = Box::new(Orientation::Vertical, 12);
    rules_container.set_margin_top(12);
    rules_container.set_margin_bottom(12);
    rules_container.set_margin_start(12);
    rules_container.set_margin_end(12);

    // Load existing rules
    let existing_rules = config.borrow().window_rules.clone().unwrap_or_default();

    for (idx, rule) in existing_rules.iter().enumerate() {
        add_window_rule_row(&rules_container, config.clone(), idx, rule.clone(), lang);
    }

    page.append(&rules_container);

    // Add button
    let add_button = Button::with_label(&crate::i18n::t("+ Add Window Rule", lang));
    add_button.set_halign(gtk4::Align::Start);
    add_button.add_css_class("suggested-action");

    let rules_container_clone = rules_container.clone();
    let config_clone = config.clone();
    let lang_clone = lang.to_string();
    add_button.connect_clicked(move |_| {
        let mut cfg = config_clone.borrow_mut();
        if cfg.window_rules.is_none() {
            cfg.window_rules = Some(Vec::new());
        }
        let new_rule = WindowRule::default();
        cfg.window_rules.as_mut().unwrap().push(new_rule.clone());
        let idx = cfg.window_rules.as_ref().unwrap().len() - 1;
        drop(cfg);

        add_window_rule_row(&rules_container_clone, config_clone.clone(), idx, new_rule, &lang_clone);
    });

    page.append(&add_button);

    stack.add_titled(&page, Some("window_rules"), &crate::i18n::t("Window Rules", lang));
}

fn add_window_rule_row(
    container: &Box,
    config: Rc<RefCell<DriftwmConfig>>,
    idx: usize,
    rule: WindowRule,
    lang: &str,
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

    let delete_btn = Button::with_label(&crate::i18n::t("Delete", lang));
    delete_btn.add_css_class("destructive-action");
    let config_clone = config.clone();
    let frame_clone = rule_frame.clone();
    delete_btn.connect_clicked(move |_| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut()
            && idx < rules.len()
        {
            rules.remove(idx);
        }
        drop(cfg);
        frame_clone.set_visible(false);
    });
    header_box.append(&delete_btn);
    rule_box.append(&header_box);

    // App ID
    let app_id_box = create_row();
    add_label(&app_id_box, &crate::i18n::t("App ID:", lang), 150);
    let app_id_entry = Entry::new();
    app_id_entry.set_text(&rule.app_id.clone().unwrap_or_default());
    app_id_entry.set_placeholder_text(Some(&crate::i18n::t("e.g., Alacritty, firefox", lang)));
    app_id_entry.set_hexpand(true);
    let config_clone = config.clone();
    app_id_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut()
            && let Some(rule) = rules.get_mut(idx)
        {
            let text = entry.text().to_string();
            rule.app_id = if text.is_empty() { None } else { Some(text) };
        }
    });
    app_id_box.append(&app_id_entry);
    rule_box.append(&app_id_box);

    // Title
    let title_box = create_row();
    add_label(&title_box, &crate::i18n::t("Title:", lang), 150);
    let title_entry = Entry::new();
    title_entry.set_text(&rule.title.clone().unwrap_or_default());
    title_entry.set_placeholder_text(Some(&crate::i18n::t("Window title (supports * glob)", lang)));
    title_entry.set_hexpand(true);
    let config_clone = config.clone();
    title_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut()
            && let Some(rule) = rules.get_mut(idx)
        {
            let text = entry.text().to_string();
            rule.title = if text.is_empty() { None } else { Some(text) };
        }
    });
    title_box.append(&title_entry);
    rule_box.append(&title_box);

    // XClass
    let xclass_box = create_row();
    add_label(&xclass_box, &crate::i18n::t("X11 Class:", lang), 150);
    let xclass_entry = Entry::new();
    xclass_entry.set_text(&rule.xclass.clone().unwrap_or_default());
    xclass_entry.set_placeholder_text(Some(&crate::i18n::t("WM_CLASS class (XWayland)", lang)));
    xclass_entry.set_hexpand(true);
    let config_clone = config.clone();
    xclass_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut()
            && let Some(rule) = rules.get_mut(idx)
        {
            let text = entry.text().to_string();
            rule.xclass = if text.is_empty() { None } else { Some(text) };
        }
    });
    xclass_box.append(&xclass_entry);
    rule_box.append(&xclass_box);

    // XInstance
    let xinstance_box = create_row();
    add_label(&xinstance_box, &crate::i18n::t("X11 Instance:", lang), 150);
    let xinstance_entry = Entry::new();
    xinstance_entry.set_text(&rule.xinstance.clone().unwrap_or_default());
    xinstance_entry.set_placeholder_text(Some(&crate::i18n::t("WM_CLASS instance (XWayland)", lang)));
    xinstance_entry.set_hexpand(true);
    let config_clone = config.clone();
    xinstance_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut()
            && let Some(rule) = rules.get_mut(idx)
        {
            let text = entry.text().to_string();
            rule.xinstance = if text.is_empty() { None } else { Some(text) };
        }
    });
    xinstance_box.append(&xinstance_entry);
    rule_box.append(&xinstance_box);

    // Pass Keys
    let passkeys_box = create_row();
    add_label(&passkeys_box, &crate::i18n::t("Pass Keys:", lang), 150);
    let passkeys_entry = Entry::new();
    let initial_passkeys = match &rule.pass_keys {
        Some(PassKeysConfig::Boolean(b)) => b.to_string(),
        Some(PassKeysConfig::List(l)) => l.join(", "),
        None => String::new(),
    };
    passkeys_entry.set_text(&initial_passkeys);
    passkeys_entry.set_placeholder_text(Some(&crate::i18n::t("true/false or mod+q, ctrl+q", lang)));
    passkeys_entry.set_hexpand(true);
    let config_clone = config.clone();
    passkeys_entry.connect_changed(move |entry| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut()
            && let Some(rule) = rules.get_mut(idx)
        {
            let text = entry.text().to_string();
            if text.is_empty() {
                rule.pass_keys = None;
            } else if text == "true" {
                rule.pass_keys = Some(PassKeysConfig::Boolean(true));
            } else if text == "false" {
                rule.pass_keys = Some(PassKeysConfig::Boolean(false));
            } else {
                rule.pass_keys = Some(PassKeysConfig::List(
                    text.split(',').map(|s| s.trim().to_string()).collect(),
                ));
            }
        }
    });
    passkeys_box.append(&passkeys_entry);
    rule_box.append(&passkeys_box);


    // Blur
    let blur_box = create_row();
    add_label(&blur_box, &crate::i18n::t("Enable blur:", lang), 150);
    let blur_switch = Switch::new();
    blur_switch.set_active(rule.blur.unwrap_or(false));
    let config_clone = config.clone();
    blur_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut()
            && let Some(rule) = rules.get_mut(idx)
        {
            rule.blur = Some(state);
        }
        gtk4::glib::Propagation::Proceed
    });
    blur_box.append(&blur_switch);
    rule_box.append(&blur_box);

    // Opacity
    let opacity_box = create_row();
    add_label(&opacity_box, &crate::i18n::t("Opacity:", lang), 150);
    let opacity_spin = SpinButton::new(
        Some(&Adjustment::new(1.0, 0.0, 1.0, 0.05, 0.1, 0.0)),
        0.05,
        2,
    );
    opacity_spin.set_value(rule.opacity.unwrap_or(1.0));
    let config_clone = config.clone();
    opacity_spin.connect_value_changed(move |spin| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut()
            && let Some(rule) = rules.get_mut(idx)
        {
            rule.opacity = Some(spin.value());
        }
    });
    opacity_box.append(&opacity_spin);
    rule_box.append(&opacity_box);

    // Decoration
    let decoration_box = create_row();
    add_label(&decoration_box, &crate::i18n::t("Decoration:", lang), 150);
    let decoration_combo = ComboBoxText::new();
    decoration_combo.append(Some("client"), &crate::i18n::t("Client", lang));
    decoration_combo.append(Some("server"), &crate::i18n::t("Server", lang));
    decoration_combo.append(Some("borderless"), &crate::i18n::t("Borderless", lang));
    decoration_combo.append(Some("none"), &crate::i18n::t("None (Bare)", lang));
    if let Some(dec) = &rule.decoration {
        decoration_combo.set_active_id(Some(dec));
    }
    let config_clone = config.clone();
    decoration_combo.connect_changed(move |combo| {
        if let Some(id) = combo.active_id() {
            let mut cfg = config_clone.borrow_mut();
            if let Some(rules) = cfg.window_rules.as_mut()
                && let Some(rule) = rules.get_mut(idx)
            {
                rule.decoration = Some(id.to_string());
            }
        }
    });
    decoration_box.append(&decoration_combo);
    rule_box.append(&decoration_box);

    // Widget
    let widget_box = create_row();
    add_label(&widget_box, &crate::i18n::t("Widget (pinned):", lang), 150);
    let widget_switch = Switch::new();
    widget_switch.set_active(rule.widget.unwrap_or(false));
    let config_clone = config.clone();
    widget_switch.connect_state_set(move |_, state| {
        let mut cfg = config_clone.borrow_mut();
        if let Some(rules) = cfg.window_rules.as_mut()
            && let Some(rule) = rules.get_mut(idx)
        {
            rule.widget = Some(state);
        }
        gtk4::glib::Propagation::Proceed
    });
    widget_box.append(&widget_switch);
    rule_box.append(&widget_box);

    rule_frame.set_child(Some(&rule_box));
    container.append(&rule_frame);
}

fn add_autostart_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Autostart Programs", lang));

    let info_label = Label::new(Some(&crate::i18n::t("Enter commands to run at startup (one per line):", lang)));
    info_label.set_halign(gtk4::Align::Start);
    info_label.add_css_class("dim-label");
    page.append(&info_label);

    // Text view for autostart commands
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);

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

    stack.add_titled(&page, Some("autostart"), &crate::i18n::t("Autostart", lang));
}

fn add_backend_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Backend Configuration", lang));

    let info_label = Label::new(Some(&crate::i18n::t(
        "Hardware stability quirks. All default to false (opt-in).\nEnable these if you experience flickering, crashes, or rendering issues.\nParticularly useful on NVIDIA GPUs with proprietary drivers.",
        lang,
    )));
    info_label.set_halign(gtk4::Align::Start);
    info_label.add_css_class("dim-label");
    page.append(&info_label);

    let info_label2 = Label::new(Some(&crate::i18n::t(
        "Note: These flags must be set before launching driftwm. Changing them requires a restart.",
        lang,
    )));
    info_label2.set_halign(gtk4::Align::Start);
    info_label2.add_css_class("dim-label");
    info_label2.add_css_class("caption");
    page.append(&info_label2);

    add_section_header(&page, &crate::i18n::t("Environment Variables", lang));

    let env_info = Label::new(Some(&crate::i18n::t(
        "For additional NVIDIA-specific settings, set these environment variables\nin your session wrapper script or shell profile before starting driftwm:\n\n  export SMITHAY_USE_LEGACY=1          # Use legacy DRM API instead of atomic modesetting\n  export __GL_GSYNC_ALLOWED=0\n  export __GL_VRR_ALLOWED=0\n  export __GL_MaxFramesAllowed=1\n  export NVD_BACKEND=direct",
        lang,
    )));
    env_info.set_halign(gtk4::Align::Start);
    env_info.add_css_class("dim-label");
    env_info.set_selectable(true);
    page.append(&env_info);

    add_section_header(&page, &crate::i18n::t("Backend Settings", lang));

    let wait_frame = add_switch_row(
        &page, &crate::i18n::t("Wait for Frame Completion", lang), &crate::i18n::t("Wait for GPU fences before page flip", lang), config
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
        &page, &crate::i18n::t("Disable Direct Scanout", lang), &crate::i18n::t("Force EGL composition (disable direct scanout)", lang), config
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

    stack.add_titled(&page, Some("backend"), &crate::i18n::t("Backend", lang));
}

fn add_keybindings_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("Keybindings", lang));

    let info_label = Label::new(Some(&crate::i18n::t(
        "Configure custom keyboard shortcuts. Click + to add new bindings.",
        lang,
    )));
    info_label.set_halign(gtk4::Align::Start);
    info_label.add_css_class("dim-label");
    page.append(&info_label);

    // Keybindings container
    let bindings_box = Box::new(Orientation::Vertical, 6);

    // Load existing keybindings
    let existing_bindings = config.borrow().keybindings.clone().unwrap_or_default();

    for (key, action) in existing_bindings.iter() {
        add_keybinding_row(
            &bindings_box,
            config.clone(),
            Some(key.clone()),
            Some(action.clone()),
            lang,
        );
    }

    page.append(&bindings_box);

    // Add button
    let add_button = Button::with_label(&crate::i18n::t("+ Add Keybinding", lang));
    add_button.set_halign(gtk4::Align::Start);
    add_button.add_css_class("suggested-action");

    let bindings_box_clone = bindings_box.clone();
    let config_clone = config.clone();
    let lang_clone = lang.to_string();
    add_button.connect_clicked(move |_| {
        add_keybinding_row(&bindings_box_clone, config_clone.clone(), None, None, &lang_clone);
    });

    page.append(&add_button);

    stack.add_titled(&page, Some("keybindings"), &crate::i18n::t("Keybindings", lang));
}

fn add_keybinding_row(
    container: &Box,
    config: Rc<RefCell<DriftwmConfig>>,
    initial_key: Option<String>,
    initial_action: Option<String>,
    lang: &str,
) {
    let row = Box::new(Orientation::Horizontal, 12);
    row.set_margin_top(6);
    row.set_margin_bottom(6);

    // Key entry
    let key_entry = Entry::new();
    key_entry.set_placeholder_text(Some(&crate::i18n::t("super+t", lang)));
    key_entry.set_width_chars(20);
    if let Some(key) = initial_key.clone() {
        key_entry.set_text(&key);
    }

    // Record button
    let record_button = Button::with_label(&crate::i18n::t("⏺", lang));
    record_button.set_tooltip_text(Some(&crate::i18n::t("Click and press keys to record", lang)));

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
    action_entry.set_placeholder_text(Some(&crate::i18n::t("exec alacritty", lang)));
    action_entry.set_hexpand(true);
    if let Some(action) = initial_action.clone() {
        action_entry.set_text(&action);
    }

    // Remove button
    let remove_button = Button::with_label(&crate::i18n::t("−", lang));
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
        if !old_key_val.is_empty()
            && old_key_val != key_text
            && let Some(ref mut kb) = cfg.keybindings
        {
            kb.remove(&old_key_val);
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

fn add_app_settings_page(stack: &Stack, lang: &str) {
    let page = create_page();

    add_header(&page, &crate::i18n::t("App Settings", lang));

    // About section
    let about_box = Box::new(Orientation::Vertical, 6);
    about_box.set_margin_top(12);
    about_box.set_margin_bottom(24);
    
    let title_label = Label::new(Some(&crate::i18n::t("driftwm-settings", lang)));
    title_label.add_css_class("heading");
    title_label.set_halign(gtk4::Align::Start);
    about_box.append(&title_label);

    let desc_label = Label::new(Some(&crate::i18n::t("A configuration tool for driftwm.", lang)));
    desc_label.set_halign(gtk4::Align::Start);
    about_box.append(&desc_label);

    let author_label = Label::new(Some(&crate::i18n::t("Created by wwmaxik.", lang)));
    author_label.set_halign(gtk4::Align::Start);
    author_label.add_css_class("dim-label");
    about_box.append(&author_label);

    let github_link = gtk4::LinkButton::with_label("https://github.com/wwmaxik/driftwm-settings", &crate::i18n::t("GitHub Repository", lang));
    github_link.set_halign(gtk4::Align::Start);
    about_box.append(&github_link);

    page.append(&about_box);

    // Language section
    add_section_header(&page, &crate::i18n::t("Language:", lang));

    let lang_box = create_row();
    
    let lang_combo = gtk4::ComboBoxText::new();
    lang_combo.append(Some("en"), &crate::i18n::t("English", lang));
    lang_combo.append(Some("ru"), &crate::i18n::t("Русский", lang));
    
    lang_combo.set_active_id(Some(lang));

    lang_combo.connect_changed(move |combo| {
        if let Some(id) = combo.active_id() {
            let mut settings = crate::app_config::AppSettings::load().unwrap_or_default();
            settings.language = id.to_string();
            let _ = settings.save();
            
            // Show a restart notice or just print to terminal
            println!("Language changed to {}. Please restart the application to fully apply.", id);
        }
    });

    lang_box.append(&lang_combo);
    page.append(&lang_box);

    // Theme section
    add_section_header(&page, &crate::i18n::t("Theme:", lang));

    let theme_box = create_row();
    let theme_combo = gtk4::ComboBoxText::new();
    theme_combo.append(Some("system"), &crate::i18n::t("System", lang));
    theme_combo.append(Some("light"), &crate::i18n::t("Light", lang));
    theme_combo.append(Some("dark"), &crate::i18n::t("Dark", lang));

    let settings = crate::app_config::AppSettings::load().unwrap_or_default();
    theme_combo.set_active_id(Some(&settings.theme));

    theme_combo.connect_changed(move |combo| {
        if let Some(id) = combo.active_id() {
            let mut settings = crate::app_config::AppSettings::load().unwrap_or_default();
            settings.theme = id.to_string();
            let _ = settings.save();

            if let Some(display) = gtk4::gdk::Display::default() {
                let gtk_settings = gtk4::Settings::for_display(&display);
                println!("Theme changed in UI to: {}", id);
                match id.as_str() {
                    "light" => {
                        gtk_settings.set_property("gtk-application-prefer-dark-theme", false);
                    }
                    "dark" => {
                        gtk_settings.set_property("gtk-application-prefer-dark-theme", true);
                    }
                    _ => {
                        gtk_settings.reset_property("gtk-application-prefer-dark-theme");
                    }
                }
            }
        }
    });

    theme_box.append(&theme_combo);
    page.append(&theme_box);

    let notice_label = Label::new(Some(&crate::i18n::t("Requires restart to fully apply", lang)));
    notice_label.set_halign(gtk4::Align::Start);
    notice_label.add_css_class("dim-label");
    notice_label.add_css_class("caption");
    notice_label.set_margin_top(12);
    page.append(&notice_label);

    page.append(&lang_box);

    stack.add_titled(&page, Some("app_settings"), &crate::i18n::t("App Settings", lang));
}
