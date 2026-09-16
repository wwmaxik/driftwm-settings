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
}
