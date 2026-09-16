#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Language {
    #[default]
    English,
    Russian,
}

impl Language {
    pub const ALL: [Language; 2] = [Language::English, Language::Russian];

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Russian => "Русский",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl Language {
    pub fn app_title(&self) -> &'static str {
        "DriftWM"
    }

    pub fn app_subtitle(&self) -> &'static str {
        match self {
            Language::English => "Settings Utility",
            Language::Russian => "Утилита настройки",
        }
    }

    // Tab titles
    pub fn tab_general(&self) -> &'static str {
        match self {
            Language::English => "General & Placement",
            Language::Russian => "Основные и размещение",
        }
    }

    pub fn tab_appearance(&self) -> &'static str {
        match self {
            Language::English => "Appearance & Deco",
            Language::Russian => "Внешний вид и рамки",
        }
    }

    pub fn tab_background(&self) -> &'static str {
        match self {
            Language::English => "Background & Shader",
            Language::Russian => "Фон и шейдеры",
        }
    }

    pub fn tab_bookmarks(&self) -> &'static str {
        match self {
            Language::English => "Bookmarks & Nav",
            Language::Russian => "Закладки и навигация",
        }
    }

    pub fn tab_input(&self) -> &'static str {
        match self {
            Language::English => "Input Devices",
            Language::Russian => "Устройства ввода",
        }
    }

    pub fn tab_keybindings(&self) -> &'static str {
        match self {
            Language::English => "Hotkeys & Actions",
            Language::Russian => "Горячие клавиши",
        }
    }

    pub fn tab_rules(&self) -> &'static str {
        match self {
            Language::English => "Window Rules",
            Language::Russian => "Правила окон",
        }
    }

    pub fn tab_shader_studio(&self) -> &'static str {
        match self {
            Language::English => "Shader Studio",
            Language::Russian => "Студия шейдеров",
        }
    }

    pub fn tab_settings(&self) -> &'static str {
        match self {
            Language::English => "App Settings",
            Language::Russian => "Настройки программы",
        }
    }

    // Sidebar buttons
    pub fn save_changes(&self) -> &'static str {
        match self {
            Language::English => "Save Changes",
            Language::Russian => "Сохранить",
        }
    }

    pub fn check_config(&self) -> &'static str {
        match self {
            Language::English => "Check Config",
            Language::Russian => "Проверить",
        }
    }

    pub fn reload_file(&self) -> &'static str {
        match self {
            Language::English => "Reload File",
            Language::Russian => "Сбросить",
        }
    }

    // Banner notifications
    pub fn unsaved_changes(&self) -> &'static str {
        match self {
            Language::English => "Unsaved changes in editor",
            Language::Russian => "Есть несохранённые изменения",
        }
    }

    pub fn save_success(&self) -> &'static str {
        match self {
            Language::English => "Configuration successfully saved and verified!",
            Language::Russian => "Конфигурация успешно сохранена и проверена!",
        }
    }

    pub fn save_warnings(&self) -> &'static str {
        match self {
            Language::English => "Configuration saved, but with warnings:",
            Language::Russian => "Конфигурация сохранена, но есть предупреждения:",
        }
    }

    pub fn save_error(&self) -> &'static str {
        match self {
            Language::English => "Failed to save configuration:",
            Language::Russian => "Ошибка сохранения конфигурации:",
        }
    }

    pub fn check_success(&self) -> &'static str {
        match self {
            Language::English => "Configuration is valid and syntax-checked!",
            Language::Russian => "Конфигурация корректна, синтаксис проверен!",
        }
    }

    pub fn reloaded(&self) -> &'static str {
        match self {
            Language::English => "Configuration reloaded from disk.",
            Language::Russian => "Конфигурация перезагружена с диска.",
        }
    }

    // App settings tab
    pub fn settings_heading(&self) -> &'static str {
        match self {
            Language::English => "Application Preferences",
            Language::Russian => "Параметры программы",
        }
    }

    pub fn settings_subheading(&self) -> &'static str {
        match self {
            Language::English => "Configure driftwm-settings interface language and behavior",
            Language::Russian => "Настройка языка интерфейса и параметров утилиты driftwm-settings",
        }
    }

    pub fn language_section(&self) -> &'static str {
        match self {
            Language::English => "Interface Language",
            Language::Russian => "Язык интерфейса",
        }
    }

    pub fn language_desc(&self) -> &'static str {
        match self {
            Language::English => "Select the display language for this configuration utility.",
            Language::Russian => "Выберите язык отображения для этой утилиты конфигурации.",
        }
    }

    pub fn config_path_section(&self) -> &'static str {
        match self {
            Language::English => "Configuration File Location",
            Language::Russian => "Расположение файла конфигурации",
        }
    }

    pub fn config_path_desc(&self) -> &'static str {
        match self {
            Language::English => "Path to ~/.config/driftwm/config.toml (saved losslessly with comments).",
            Language::Russian => "Путь к ~/.config/driftwm/config.toml (сохранение с сохранением комментариев).",
        }
    }

    pub fn reset_default(&self) -> &'static str {
        match self {
            Language::English => "Reset to Default Path",
            Language::Russian => "Сбросить на путь по умолчанию",
        }
    }

    pub fn live_validation_section(&self) -> &'static str {
        match self {
            Language::English => "Auto-Validation",
            Language::Russian => "Автопроверка синтаксиса",
        }
    }

    pub fn live_validation_desc(&self) -> &'static str {
        match self {
            Language::English => "Automatically check config syntax whenever saving changes.",
            Language::Russian => "Автоматически проверять синтаксис конфига при сохранении.",
        }
    }

    pub fn about_heading(&self) -> &'static str {
        match self {
            Language::English => "About DriftWM Settings",
            Language::Russian => "О программе DriftWM Settings",
        }
    }

    pub fn about_desc(&self) -> &'static str {
        match self {
            Language::English => "Modern, lightweight GUI utility for the driftwm 0.19.x tiling window manager.",
            Language::Russian => "Современная, легковесная графическая утилита для оконного менеджера driftwm 0.19.x.",
        }
    }

    pub fn about_version(&self) -> &'static str {
        match self {
            Language::English => "Version 0.5.0",
            Language::Russian => "Версия 0.5.0",
        }
    }

    pub fn about_tech(&self) -> &'static str {
        match self {
            Language::English => "Pure Rust & Iced 0.13 GUI (Catppuccin Mocha)",
            Language::Russian => "Чистый Rust и Iced 0.13 GUI (тема Catppuccin Mocha)",
        }
    }

    pub fn about_github(&self) -> &'static str {
        "https://github.com/wwmaxik/driftwm-settings"
    }

    // General tab
    pub fn general_placement_heading(&self) -> &'static str {
        match self {
            Language::English => "Window Placement",
            Language::Russian => "Размещение окон",
        }
    }

    pub fn general_placement_desc(&self) -> &'static str {
        match self {
            Language::English => "How new client windows are positioned on the infinite canvas",
            Language::Russian => "Способ размещения новых окон на бесконечном холсте",
        }
    }

    pub fn general_focus_heading(&self) -> &'static str {
        match self {
            Language::English => "Focus Management",
            Language::Russian => "Управление фокусом",
        }
    }

    pub fn general_modkey_heading(&self) -> &'static str {
        match self {
            Language::English => "Modifier Key (Mod)",
            Language::Russian => "Клавиша-модификатор (Mod Key)",
        }
    }

    pub fn general_session_heading(&self) -> &'static str {
        match self {
            Language::English => "Behavior & Session",
            Language::Russian => "Поведение и сессия",
        }
    }

    // Appearance tab
    pub fn deco_heading(&self) -> &'static str {
        match self {
            Language::English => "Window Decorations",
            Language::Russian => "Декорации окон",
        }
    }

    pub fn gaps_heading(&self) -> &'static str {
        match self {
            Language::English => "Gaps & Margins",
            Language::Russian => "Отступы между окнами",
        }
    }

    pub fn borders_heading(&self) -> &'static str {
        match self {
            Language::English => "Borders & Colors",
            Language::Russian => "Границы и цвета",
        }
    }

    pub fn opacity_blur_heading(&self) -> &'static str {
        match self {
            Language::English => "Opacity & Blur",
            Language::Russian => "Прозрачность и размытие",
        }
    }

    // Background tab
    pub fn bg_mode_heading(&self) -> &'static str {
        match self {
            Language::English => "Background Mode",
            Language::Russian => "Режим фона",
        }
    }

    pub fn bg_shader_heading(&self) -> &'static str {
        match self {
            Language::English => "GLSL Shader Canvas",
            Language::Russian => "GLSL Шейдерный холст",
        }
    }

    pub fn bg_wallpaper_heading(&self) -> &'static str {
        match self {
            Language::English => "Wallpaper Image",
            Language::Russian => "Обои рабочего стола",
        }
    }

    // Bookmarks tab
    pub fn bm_bookmarks_heading(&self) -> &'static str {
        match self {
            Language::English => "Canvas Bookmarks",
            Language::Russian => "Закладки на холсте",
        }
    }

    pub fn bm_anchors_heading(&self) -> &'static str {
        match self {
            Language::English => "Grid Anchors",
            Language::Russian => "Якоря сетки",
        }
    }

    pub fn bm_nav_heading(&self) -> &'static str {
        match self {
            Language::English => "Camera & Navigation Dynamics",
            Language::Russian => "Динамика камеры и навигации",
        }
    }

    // Input tab
    pub fn input_kb_heading(&self) -> &'static str {
        match self {
            Language::English => "Keyboard Configuration",
            Language::Russian => "Настройки клавиатуры",
        }
    }

    pub fn input_trackpad_heading(&self) -> &'static str {
        match self {
            Language::English => "Trackpad & Gestures",
            Language::Russian => "Тачпад и жесты",
        }
    }

    pub fn input_mouse_heading(&self) -> &'static str {
        match self {
            Language::English => "Mouse & Pointer",
            Language::Russian => "Мышь и курсор",
        }
    }

    pub fn add_bookmark(&self) -> &'static str {
        match self {
            Language::English => "+ Add Bookmark",
            Language::Russian => "+ Добавить закладку",
        }
    }

    pub fn add_anchor(&self) -> &'static str {
        match self {
            Language::English => "+ Add Anchor",
            Language::Russian => "+ Добавить якорь",
        }
    }

    pub fn delete(&self) -> &'static str {
        match self {
            Language::English => "Delete",
            Language::Russian => "Удалить",
        }
    }

    // Keybindings tab
    pub fn keybindings_heading(&self) -> &'static str {
        match self {
            Language::English => "Keyboard Shortcuts & Actions",
            Language::Russian => "Горячие клавиши и действия",
        }
    }

    pub fn keybindings_desc(&self) -> &'static str {
        match self {
            Language::English => "Map keyboard chords to window actions, commands, navigation, and scripts.",
            Language::Russian => "Настройка сочетаний клавиш для управления окнами, командами, навигацией и скриптами.",
        }
    }

    pub fn search_keybindings(&self) -> &'static str {
        match self {
            Language::English => "Search shortcuts or actions...",
            Language::Russian => "Поиск клавиш или действий...",
        }
    }

    pub fn add_keybinding(&self) -> &'static str {
        match self {
            Language::English => "+ Add Keybinding",
            Language::Russian => "+ Добавить горячую клавишу",
        }
    }

    pub fn populate_defaults(&self) -> &'static str {
        match self {
            Language::English => "Load Standard Defaults",
            Language::Russian => "Загрузить стандартные бинды",
        }
    }

    pub fn shortcut_key_header(&self) -> &'static str {
        match self {
            Language::English => "Key Combination",
            Language::Russian => "Сочетание клавиш",
        }
    }

    pub fn shortcut_action_header(&self) -> &'static str {
        match self {
            Language::English => "Action / Command",
            Language::Russian => "Действие / Команда",
        }
    }

    pub fn unbind(&self) -> &'static str {
        match self {
            Language::English => "Unbind",
            Language::Russian => "Отвязать",
        }
    }

    pub fn disable_defaults_heading(&self) -> &'static str {
        match self {
            Language::English => "Disable Built-in Defaults",
            Language::Russian => "Отключение стандартных биндов",
        }
    }

    pub fn disable_keys_label(&self) -> &'static str {
        match self {
            Language::English => "Disable default keyboard shortcuts (keys)",
            Language::Russian => "Отключить стандартные клавиши (keys)",
        }
    }

    pub fn disable_mouse_label(&self) -> &'static str {
        match self {
            Language::English => "Disable default mouse bindings (mouse)",
            Language::Russian => "Отключить стандартную мышь (mouse)",
        }
    }

    pub fn disable_gestures_label(&self) -> &'static str {
        match self {
            Language::English => "Disable default touchpad gestures (gestures)",
            Language::Russian => "Отключить стандартные жесты тачпада (gestures)",
        }
    }

    pub fn disable_touch_label(&self) -> &'static str {
        match self {
            Language::English => "Disable default touchscreen gestures (touch)",
            Language::Russian => "Отключить стандартные жесты экрана (touch)",
        }
    }

    // Rules tab
    pub fn rules_heading(&self) -> &'static str {
        match self {
            Language::English => "Window Rules",
            Language::Russian => "Правила для окон",
        }
    }

    pub fn rules_desc(&self) -> &'static str {
        match self {
            Language::English => "Configure custom behaviors, opacity, borders, and placement per application or window title.",
            Language::Russian => "Правила для отдельных приложений или заголовков (прозрачность, рамки, PiP, геометрия).",
        }
    }

    pub fn add_rule(&self) -> &'static str {
        match self {
            Language::English => "+ Add Window Rule",
            Language::Russian => "+ Добавить правило",
        }
    }

    pub fn rule_app_id(&self) -> &'static str {
        match self {
            Language::English => "App ID (exact, glob*, /regex/):",
            Language::Russian => "Идентификатор App ID (точно, glob*, /regex/):",
        }
    }

    pub fn rule_title(&self) -> &'static str {
        match self {
            Language::English => "Window Title:",
            Language::Russian => "Заголовок окна:",
        }
    }

    pub fn rule_decoration(&self) -> &'static str {
        match self {
            Language::English => "Decoration Mode:",
            Language::Russian => "Режим декораций:",
        }
    }

    pub fn rule_opacity(&self) -> &'static str {
        match self {
            Language::English => "Opacity (0.0 - 1.0):",
            Language::Russian => "Непрозрачность (0.0 - 1.0):",
        }
    }

    pub fn rule_blur(&self) -> &'static str {
        match self {
            Language::English => "Blur backdrop behind window",
            Language::Russian => "Размытие фона под окном",
        }
    }

    pub fn rule_pinned(&self) -> &'static str {
        match self {
            Language::English => "Pin to screen (PiP float, ignores pan/zoom)",
            Language::Russian => "Закрепить на экране (PiP, поверх всех, не скроллится)",
        }
    }

    pub fn rule_widget(&self) -> &'static str {
        match self {
            Language::English => "Widget (immovable background surface)",
            Language::Russian => "Виджет (неподвижное фоновое окно)",
        }
    }

    pub fn rule_fullscreen(&self) -> &'static str {
        match self {
            Language::English => "Start in Fullscreen",
            Language::Russian => "Запуск на весь экран",
        }
    }

    pub fn rule_focus_on_open(&self) -> &'static str {
        match self {
            Language::English => "Focus camera on open",
            Language::Russian => "Фокусировать камеру при создании",
        }
    }

    // Shader Studio
    pub fn shader_studio_desc(&self) -> &'static str {
        match self {
            Language::English => "Interactive GLSL wallpaper generator: tweak parameters live, preview in real time, and apply directly to driftwm.",
            Language::Russian => "Интерактивный генератор процедурных GLSL-обоев: изменяйте параметры, смотрите результат в реальном времени и применяйте в driftwm.",
        }
    }

    pub fn shader_archetype(&self) -> &'static str {
        match self {
            Language::English => "Shader Style / Archetype:",
            Language::Russian => "Стиль шейдера:",
        }
    }

    pub fn shader_palette_presets(&self) -> &'static str {
        match self {
            Language::English => "Color Palettes:",
            Language::Russian => "Цветовые палитры:",
        }
    }

    pub fn shader_preview_badge(&self) -> &'static str {
        match self {
            Language::English => "LIVE VECTOR SIMULATION PREVIEW",
            Language::Russian => "ИНТЕРАКТИВНЫЙ ПРЕДПРОСМОТР",
        }
    }

    pub fn shader_speed(&self) -> &'static str {
        match self {
            Language::English => "Animation Speed (u_time):",
            Language::Russian => "Скорость анимации (u_time):",
        }
    }

    pub fn shader_scale(&self) -> &'static str {
        match self {
            Language::English => "Pattern Scale / Density:",
            Language::Russian => "Масштаб / плотность сетки:",
        }
    }

    pub fn shader_parallax(&self) -> &'static str {
        match self {
            Language::English => "Canvas Parallax (u_camera):",
            Language::Russian => "Параллакс холста (u_camera):",
        }
    }

    pub fn shader_glow(&self) -> &'static str {
        match self {
            Language::English => "Glow & Intensity:",
            Language::Russian => "Яркость и свечение:",
        }
    }

    pub fn shader_zoom_reactive(&self) -> &'static str {
        match self {
            Language::English => "Zoom Reactive (scale features with u_zoom)",
            Language::Russian => "Масштабировать элементы вместе с зумом (u_zoom)",
        }
    }

    pub fn shader_transparent(&self) -> &'static str {
        match self {
            Language::English => "Transparent Shader (transparent_shader = true for external backdrop)",
            Language::Russian => "Прозрачный шейдер (transparent_shader = true для внешних обоев)",
        }
    }

    pub fn shader_color_primary(&self) -> &'static str {
        match self {
            Language::English => "Color 1 (Primary)",
            Language::Russian => "Цвет 1 (Основной)",
        }
    }

    pub fn shader_color_secondary(&self) -> &'static str {
        match self {
            Language::English => "Color 2 (Secondary)",
            Language::Russian => "Цвет 2 (Вторичный)",
        }
    }

    pub fn shader_color_accent(&self) -> &'static str {
        match self {
            Language::English => "Color 3 (Accent)",
            Language::Russian => "Цвет 3 (Акцент)",
        }
    }

    pub fn shader_color_bg(&self) -> &'static str {
        match self {
            Language::English => "Color 4 (Background)",
            Language::Russian => "Цвет 4 (Фон)",
        }
    }

    pub fn shader_code_title(&self) -> &'static str {
        match self {
            Language::English => "GLSL ES 1.0 Shader Source",
            Language::Russian => "Исходный код GLSL ES 1.0",
        }
    }

    pub fn shader_manual_mode(&self) -> &'static str {
        match self {
            Language::English => "Manual Code Edit Mode",
            Language::Russian => "Ручное редактирование кода",
        }
    }

    pub fn shader_reset_code(&self) -> &'static str {
        match self {
            Language::English => "Reset to Generator",
            Language::Russian => "Сбросить к генератору",
        }
    }

    pub fn shader_check_syntax(&self) -> &'static str {
        match self {
            Language::English => "Check Syntax",
            Language::Russian => "Проверить синтаксис",
        }
    }

    pub fn shader_save_button(&self) -> &'static str {
        match self {
            Language::English => "Save Shader",
            Language::Russian => "Сохранить шейдер",
        }
    }

    pub fn shader_apply_button(&self) -> &'static str {
        match self {
            Language::English => "Save & Set as Driftwm Background",
            Language::Russian => "Сохранить и применить как фон",
        }
    }

    pub fn shader_filename_label(&self) -> &'static str {
        match self {
            Language::English => "File Name (.glsl):",
            Language::Russian => "Имя файла (.glsl):",
        }
    }
}
