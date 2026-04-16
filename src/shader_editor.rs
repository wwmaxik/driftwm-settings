use gtk4::prelude::*;
use gtk4::{
    Adjustment, Box, Button, ComboBoxText, Label, Orientation, ScrolledWindow, SpinButton, Stack,
    Switch, TextView,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::config::*;
use crate::ui_helpers::*;

struct ShaderParams {
    template: ComboBoxText,
    color1_r: SpinButton,
    color1_g: SpinButton,
    color1_b: SpinButton,
    color2_r: SpinButton,
    color2_g: SpinButton,
    color2_b: SpinButton,
    speed: SpinButton,
    scale: SpinButton,
    complexity: SpinButton,
    vignette: Switch,
    glow: SpinButton,
}

impl ShaderParams {
    fn generate_shader(&self) -> String {
        generate_shader_from_params(
            self.template
                .active_id()
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("gradient"),
            self.color1_r.value(),
            self.color1_g.value(),
            self.color1_b.value(),
            self.color2_r.value(),
            self.color2_g.value(),
            self.color2_b.value(),
            self.speed.value(),
            self.scale.value(),
            self.complexity.value() as i32,
            self.vignette.is_active(),
            self.glow.value(),
        )
    }
}

pub fn add_shader_editor_page(stack: &Stack, config: Rc<RefCell<DriftwmConfig>>) {
    let page = create_page();

    add_header(&page, "Shader Editor");

    let info_label = Label::new(Some(
        "Create custom animated backgrounds with visual controls. Switch to Raw mode for advanced editing.",
    ));
    info_label.set_halign(gtk4::Align::Start);
    info_label.add_css_class("dim-label");
    page.append(&info_label);

    // Mode toggle (Visual / Raw)
    let mode_box = Box::new(Orientation::Horizontal, 12);
    mode_box.set_margin_top(12);
    mode_box.set_margin_bottom(12);

    let visual_mode_btn = Button::with_label("Visual Mode");
    visual_mode_btn.add_css_class("suggested-action");
    let raw_mode_btn = Button::with_label("Raw Mode");

    mode_box.append(&visual_mode_btn);
    mode_box.append(&raw_mode_btn);
    page.append(&mode_box);

    // Visual editor container
    let visual_container = Box::new(Orientation::Vertical, 12);
    visual_container.set_margin_start(12);
    visual_container.set_margin_end(12);

    // Template selector
    add_section_header(&visual_container, "Shader Template");

    let template_box = create_row();
    add_label(&template_box, "Base Template:", 200);
    let template_combo = ComboBoxText::new();
    template_combo.append(Some("gradient"), "Gradient");
    template_combo.append(Some("waves"), "Animated Waves");
    template_combo.append(Some("clouds"), "Clouds");
    template_combo.set_active_id(Some("gradient"));
    template_box.append(&template_combo);
    visual_container.append(&template_box);

    // Color controls
    add_section_header(&visual_container, "Colors");

    // Primary color
    let color1_box = create_row();
    add_label(&color1_box, "Primary Color:", 200);
    let color1_r = SpinButton::new(
        Some(&Adjustment::new(0.2, 0.0, 1.0, 0.01, 0.1, 0.0)),
        0.01,
        2,
    );
    let color1_g = SpinButton::new(
        Some(&Adjustment::new(0.1, 0.0, 1.0, 0.01, 0.1, 0.0)),
        0.01,
        2,
    );
    let color1_b = SpinButton::new(
        Some(&Adjustment::new(0.3, 0.0, 1.0, 0.01, 0.1, 0.0)),
        0.01,
        2,
    );
    color1_r.set_width_chars(6);
    color1_g.set_width_chars(6);
    color1_b.set_width_chars(6);

    color1_box.append(&Label::new(Some("R:")));
    color1_box.append(&color1_r);
    color1_box.append(&Label::new(Some("G:")));
    color1_box.append(&color1_g);
    color1_box.append(&Label::new(Some("B:")));
    color1_box.append(&color1_b);
    visual_container.append(&color1_box);

    // Secondary color
    let color2_box = create_row();
    add_label(&color2_box, "Secondary Color:", 200);
    let color2_r = SpinButton::new(
        Some(&Adjustment::new(0.1, 0.0, 1.0, 0.01, 0.1, 0.0)),
        0.01,
        2,
    );
    let color2_g = SpinButton::new(
        Some(&Adjustment::new(0.2, 0.0, 1.0, 0.01, 0.1, 0.0)),
        0.01,
        2,
    );
    let color2_b = SpinButton::new(
        Some(&Adjustment::new(0.4, 0.0, 1.0, 0.01, 0.1, 0.0)),
        0.01,
        2,
    );
    color2_r.set_width_chars(6);
    color2_g.set_width_chars(6);
    color2_b.set_width_chars(6);

    color2_box.append(&Label::new(Some("R:")));
    color2_box.append(&color2_r);
    color2_box.append(&Label::new(Some("G:")));
    color2_box.append(&color2_g);
    color2_box.append(&Label::new(Some("B:")));
    color2_box.append(&color2_b);
    visual_container.append(&color2_box);

    // Animation controls
    add_section_header(&visual_container, "Animation");

    let speed_box = create_row();
    add_label(&speed_box, "Animation Speed:", 200);
    let speed_spin = SpinButton::new(
        Some(&Adjustment::new(1.0, 0.0, 10.0, 0.1, 1.0, 0.0)),
        0.1,
        1,
    );
    speed_spin.set_tooltip_text(Some("0 = static, 1 = normal, 10 = very fast"));
    speed_box.append(&speed_spin);
    visual_container.append(&speed_box);

    let scale_box = create_row();
    add_label(&scale_box, "Pattern Scale:", 200);
    let scale_spin = SpinButton::new(Some(&Adjustment::new(1.0, 0.1, 5.0, 0.1, 0.5, 0.0)), 0.1, 1);
    scale_spin.set_tooltip_text(Some("Smaller = zoomed in, Larger = zoomed out"));
    scale_box.append(&scale_spin);
    visual_container.append(&scale_box);

    let complexity_box = create_row();
    add_label(&complexity_box, "Complexity:", 200);
    let complexity_spin =
        SpinButton::new(Some(&Adjustment::new(3.0, 1.0, 8.0, 1.0, 1.0, 0.0)), 1.0, 0);
    complexity_spin.set_tooltip_text(Some(
        "Number of detail layers (higher = more detailed but slower)",
    ));
    complexity_box.append(&complexity_spin);
    visual_container.append(&complexity_box);

    // Effects
    add_section_header(&visual_container, "Effects");

    let vignette_box = create_row();
    add_label(&vignette_box, "Vignette:", 200);
    let vignette_switch = Switch::new();
    vignette_switch.set_active(true);
    vignette_box.append(&vignette_switch);
    visual_container.append(&vignette_box);

    let glow_box = create_row();
    add_label(&glow_box, "Glow Intensity:", 200);
    let glow_spin = SpinButton::new(
        Some(&Adjustment::new(0.1, 0.0, 1.0, 0.05, 0.1, 0.0)),
        0.05,
        2,
    );
    glow_box.append(&glow_spin);
    visual_container.append(&glow_box);

    // Raw editor container (hidden by default)
    let raw_container = Box::new(Orientation::Vertical, 12);
    raw_container.set_visible(false);

    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_min_content_height(400);

    let text_view = TextView::new();
    text_view.set_monospace(true);
    text_view.set_left_margin(12);
    text_view.set_right_margin(12);
    text_view.set_top_margin(12);
    text_view.set_bottom_margin(12);
    text_view.set_wrap_mode(gtk4::WrapMode::None);

    let buffer = text_view.buffer();
    buffer.set_text("// Shader code will appear here when you switch to Raw mode");

    scrolled.set_child(Some(&text_view));
    raw_container.append(&scrolled);

    // Add both containers to page
    page.append(&visual_container);
    page.append(&raw_container);

    // Create params struct
    let params = Rc::new(ShaderParams {
        template: template_combo.clone(),
        color1_r: color1_r.clone(),
        color1_g: color1_g.clone(),
        color1_b: color1_b.clone(),
        color2_r: color2_r.clone(),
        color2_g: color2_g.clone(),
        color2_b: color2_b.clone(),
        speed: speed_spin.clone(),
        scale: scale_spin.clone(),
        complexity: complexity_spin.clone(),
        vignette: vignette_switch.clone(),
        glow: glow_spin.clone(),
    });

    // Mode switching
    setup_mode_switching(
        &visual_mode_btn,
        &raw_mode_btn,
        &visual_container,
        &raw_container,
        &buffer,
        &params,
    );

    // Button bar
    let button_box = Box::new(Orientation::Horizontal, 12);
    button_box.set_margin_top(12);

    setup_buttons(&button_box, &config, &visual_container, &buffer, &params);

    page.append(&button_box);

    stack.add_titled(&page, Some("shader_editor"), "Shader Editor");
}

fn setup_mode_switching(
    visual_btn: &Button,
    raw_btn: &Button,
    visual_container: &Box,
    raw_container: &Box,
    buffer: &gtk4::TextBuffer,
    params: &Rc<ShaderParams>,
) {
    // Visual mode button
    let visual_clone = visual_container.clone();
    let raw_clone = raw_container.clone();
    let visual_btn_clone = visual_btn.clone();
    let raw_btn_clone = raw_btn.clone();
    visual_btn.connect_clicked(move |_| {
        visual_clone.set_visible(true);
        raw_clone.set_visible(false);
        visual_btn_clone.add_css_class("suggested-action");
        raw_btn_clone.remove_css_class("suggested-action");
    });

    // Raw mode button
    let visual_clone = visual_container.clone();
    let raw_clone = raw_container.clone();
    let visual_btn_clone = visual_btn.clone();
    let raw_btn_clone = raw_btn.clone();
    let buffer_clone = buffer.clone();
    let params_clone = params.clone();

    raw_btn.connect_clicked(move |_| {
        let shader_code = params_clone.generate_shader();
        buffer_clone.set_text(&shader_code);

        visual_clone.set_visible(false);
        raw_clone.set_visible(true);
        visual_btn_clone.remove_css_class("suggested-action");
        raw_btn_clone.add_css_class("suggested-action");
    });
}

fn setup_buttons(
    button_box: &Box,
    config: &Rc<RefCell<DriftwmConfig>>,
    visual_container: &Box,
    buffer: &gtk4::TextBuffer,
    params: &Rc<ShaderParams>,
) {
    // Generate & Save button
    let save_shader_btn = Button::with_label("Generate & Save Shader");
    save_shader_btn.add_css_class("suggested-action");

    let buffer_clone = buffer.clone();
    let params_clone = params.clone();
    let visual_clone = visual_container.clone();

    save_shader_btn.connect_clicked(move |btn| {
        let shader_code = if visual_clone.is_visible() {
            params_clone.generate_shader()
        } else {
            let start = buffer_clone.start_iter();
            let end = buffer_clone.end_iter();
            buffer_clone.text(&start, &end, false).to_string()
        };

        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let shader_path = format!("{}/.config/driftwm/custom_shader.glsl", home);

        match std::fs::create_dir_all(format!("{}/.config/driftwm", home)) {
            Ok(_) => match std::fs::write(&shader_path, shader_code) {
                Ok(_) => {
                    btn.set_label("✓ Saved!");
                    let btn_clone = btn.clone();
                    gtk4::glib::timeout_add_seconds_local(2, move || {
                        btn_clone.set_label("Generate & Save Shader");
                        gtk4::glib::ControlFlow::Break
                    });
                }
                Err(e) => {
                    eprintln!("Failed to save shader: {}", e);
                    btn.set_label("✗ Error!");
                    let btn_clone = btn.clone();
                    gtk4::glib::timeout_add_seconds_local(2, move || {
                        btn_clone.set_label("Generate & Save Shader");
                        gtk4::glib::ControlFlow::Break
                    });
                }
            },
            Err(e) => {
                eprintln!("Failed to create directory: {}", e);
            }
        }
    });

    button_box.append(&save_shader_btn);

    // Apply shader button
    let apply_shader_btn = Button::with_label("Apply to Background");
    apply_shader_btn.add_css_class("suggested-action");

    let config_clone = config.clone();
    apply_shader_btn.connect_clicked(move |btn| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let shader_path = format!("{}/.config/driftwm/custom_shader.glsl", home);

        let mut cfg = config_clone.borrow_mut();
        if cfg.background.is_none() {
            cfg.background = Some(BackgroundConfig::default());
        }
        cfg.background.as_mut().unwrap().shader_path = Some(shader_path);
        drop(cfg);

        btn.set_label("✓ Applied!");
        let btn_clone = btn.clone();
        gtk4::glib::timeout_add_seconds_local(2, move || {
            btn_clone.set_label("Apply to Background");
            gtk4::glib::ControlFlow::Break
        });
    });

    button_box.append(&apply_shader_btn);
}

#[allow(clippy::too_many_arguments)]
fn generate_shader_from_params(
    template: &str,
    c1_r: f64,
    c1_g: f64,
    c1_b: f64,
    c2_r: f64,
    c2_g: f64,
    c2_b: f64,
    speed: f64,
    scale: f64,
    complexity: i32,
    vignette: bool,
    glow: f64,
) -> String {
    let vignette_code = if vignette {
        "float vignette = 1.0 - length(uv - 0.5) * 0.5;\n    color *= vignette;"
    } else {
        ""
    };

    let glow_code = if glow > 0.01 {
        format!("color += vec3({:.2});", glow)
    } else {
        String::new()
    };

    match template {
        "gradient" => {
            format!(
                r#"// Generated Gradient Shader
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform float alpha;
uniform vec2 u_camera;
uniform float u_time;

void main() {{
    vec2 uv = v_coords;
    vec3 color1 = vec3({:.2}, {:.2}, {:.2});
    vec3 color2 = vec3({:.2}, {:.2}, {:.2});
    
    float t = uv.y + sin(uv.x * 3.14159 * {:.2} + u_time * {:.2}) * 0.1;
    vec3 color = mix(color1, color2, t);
    
    {}
    {}
    
    gl_FragColor = vec4(color, 1.0) * alpha;
}}
"#,
                c1_r, c1_g, c1_b, c2_r, c2_g, c2_b, scale, speed, vignette_code, glow_code
            )
        }
        "waves" => {
            format!(
                r#"// Generated Waves Shader
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform float alpha;
uniform vec2 u_camera;
uniform float u_time;

void main() {{
    vec2 screen_pixel = v_coords * size;
    vec2 canvas_pos = screen_pixel + u_camera;
    vec2 pos = canvas_pos / (100.0 * {:.2});
    
    float wave1 = sin(pos.x * 10.0 + u_time * {:.2}) * 0.5 + 0.5;
    float wave2 = sin(pos.y * 10.0 + u_time * {:.2} * 0.75) * 0.5 + 0.5;
    
    vec3 color1 = vec3({:.2}, {:.2}, {:.2});
    vec3 color2 = vec3({:.2}, {:.2}, {:.2});
    vec3 color = mix(color1, color2, wave1 * wave2);
    
    vec2 uv = v_coords;
    {}
    {}
    
    gl_FragColor = vec4(color, 1.0) * alpha;
}}
"#,
                scale, speed, speed, c1_r, c1_g, c1_b, c2_r, c2_g, c2_b, vignette_code, glow_code
            )
        }
        "clouds" => {
            format!(
                r#"// Generated Clouds Shader
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform float alpha;
uniform vec2 u_camera;
uniform float u_time;

vec2 hash2(vec2 p) {{
    p = vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)));
    return fract(sin(p) * 43758.5453);
}}

float noise(vec2 p) {{
    vec2 i = floor(p);
    vec2 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    vec2 a = hash2(i);
    vec2 b = hash2(i + vec2(1.0, 0.0));
    vec2 c = hash2(i + vec2(0.0, 1.0));
    vec2 d = hash2(i + vec2(1.0, 1.0));
    return mix(mix(a.x, b.x, f.x), mix(c.x, d.x, f.x), f.y);
}}

float fbm(vec2 p) {{
    float v = 0.0;
    float a = 0.5;
    mat2 rot = mat2(0.8, 0.6, -0.6, 0.8);
    for (int i = 0; i < {}; i++) {{
        v += a * noise(p);
        p = rot * p * 2.0;
        a *= 0.5;
    }}
    return v;
}}

void main() {{
    vec2 canvas = (v_coords * size + u_camera) * (0.003 / {:.2});
    float time = u_time * {:.2} * 0.08;
    
    float wx = fbm(canvas + time * 0.05);
    float wy = fbm(canvas + vec2(5.2, 1.3));
    vec2 warped = canvas + vec2(wx, wy) * 0.8;
    
    float clouds = fbm(warped + time * 0.02);
    
    vec3 color1 = vec3({:.2}, {:.2}, {:.2});
    vec3 color2 = vec3({:.2}, {:.2}, {:.2});
    vec3 color = mix(color1, color2, clouds);
    
    vec2 uv = v_coords;
    {}
    {}
    
    gl_FragColor = vec4(color, 1.0) * alpha;
}}
"#,
                complexity,
                scale,
                speed,
                c1_r,
                c1_g,
                c1_b,
                c2_r,
                c2_g,
                c2_b,
                vignette_code,
                glow_code
            )
        }
        "particles" => {
            format!(
                r#"// Generated Particles Shader
precision highp float;

varying vec2 v_coords;
uniform vec2 size;
uniform float alpha;
uniform vec2 u_camera;
uniform float u_time;

vec2 hash2(vec2 p) {{
    p = vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)));
    return fract(sin(p) * 43758.5453);
}}

void main() {{
    vec2 canvas = (v_coords * size + u_camera) * (0.01 / {:.2});
    vec3 color1 = vec3({:.2}, {:.2}, {:.2});
    vec3 color2 = vec3({:.2}, {:.2}, {:.2});
    vec3 color = color1;
    
    for (int i = 0; i < {}; i++) {{
        vec2 p = hash2(vec2(float(i), 0.0)) * 10.0;
        p.y += u_time * {:.2} * (hash2(vec2(float(i), 1.0)).x + 0.5);
        p.y = mod(p.y, 10.0);
        
        float dist = length(canvas - p);
        float particle = smoothstep(0.1, 0.0, dist);
        color = mix(color, color2, particle);
    }}
    
    vec2 uv = v_coords;
    {}
    {}
    
    gl_FragColor = vec4(color, 1.0) * alpha;
}}
"#,
                scale,
                c1_r,
                c1_g,
                c1_b,
                c2_r,
                c2_g,
                c2_b,
                complexity * 10,
                speed,
                vignette_code,
                glow_code
            )
        }
        _ => generate_shader_from_params(
            "gradient", c1_r, c1_g, c1_b, c2_r, c2_g, c2_b, speed, scale, complexity, vignette,
            glow,
        ),
    }
}
