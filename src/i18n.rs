pub fn t(key: &str, lang: impl AsRef<str>) -> String {
    let lang_str = lang.as_ref();
    if lang_str == "en" {
        return key.to_string();
    }

    if lang_str == "ru" {
        let res = match key {
            "#303030" => "#303030",
            "#FFFFFF" => "#FFFFFF",
            "+ Add Keybinding" => "+ Добавить горячую клавишу",
            "+ Add Window Rule" => "+ Добавить правило окна",
            "0 = static, 1 = normal, 10 = very fast" => {
                "0 = статично, 1 = нормально, 10 = очень быстро"
            }
            "A configuration tool for driftwm." => "Инструмент настройки для driftwm.",
            "Acceleration profile:" => "Профиль ускорения:",
            "Acceleration speed:" => "Скорость ускорения:",
            "Adaptive" => "Адаптивный",
            "Adwaita (Default)" => "Adwaita (По умолчанию)",
            "Alt" => "Alt",
            "Alt+Shift to switch layout" => "Alt+Shift для переключения раскладки",
            "Animate zoom to 1.0 when a new window is mapped" => {
                "Анимировать масштаб до 1.0 при открытии нового окна"
            }
            "Animated Waves" => "Анимированные волны",
            "Animation" => "Анимация",
            "Animation Speed:" => "Скорость анимации:",
            "Animation speed:" => "Скорость анимации:",
            "App ID:" => "App ID:",
            "App Settings" => "Настройки приложения",
            "Apply to Background" => "Применить как обои",
            "Autostart" => "Автозапуск",
            "Autostart Programs" => "Программы в автозапуске",
            "B:" => "B:",
            "Backend" => "Бэкенд",
            "Backend Configuration" => "Настройки бэкенда",
            "Backend Settings" => "Настройки бэкенда",
            "Background" => "Фон",
            "Background color:" => "Цвет фона:",
            "Base Template:" => "Базовый шаблон:",
            "Blur Presets" => "Пресеты размытия",
            "Blur Settings" => "Настройки размытия",
            "Blur radius (passes):" => "Радиус размытия (проходы):",
            "Blur strength (spread):" => "Сила размытия (разброс):",
            "Borderless" => "Без рамок",
            "Borderless (SSD)" => "Без рамок (SSD)",
            "Break force (px):" => "Сила отрыва (px):",
            "Breeze Snow" => "Breeze Snow",
            "Button Areas (Bottom L/R)" => "Зоны кнопок (Снизу Л/П)",
            "Caps Lock as Ctrl" => "Caps Lock как Ctrl",
            "Caps Lock as Escape" => "Caps Lock как Escape",
            "Caps Lock on startup:" => "Caps Lock при запуске:",
            "Click and press keys to record" => "Нажмите сюда и введите комбинацию",
            "Click method:" => "Метод клика:",
            "Clickfinger (1=L, 2=R, 3=M)" => "Количество пальцев (1=Л, 2=П, 3=С)",
            "Client" => "Клиентские",
            "Client (CSD)" => "Клиентские (CSD)",
            "Clouds" => "Облака",
            "Cluster-aware fit (maximize):" => "Умное развертывание кластера:",
            "Cluster-aware resize:" => "Умное изменение размера кластера:",
            "Colors" => "Цвета",
            "Complexity:" => "Сложность:",
            "Corner radius:" => "Радиус закругления:",
            "Created by wwmaxik." => "Создано wwmaxik.",
            "Ctrl+Shift to switch layout" => "Ctrl+Shift для переключения раскладки",
            "Cursor" => "Курсор",
            "Cursor Settings" => "Настройки курсора",
            "Cursor size:" => "Размер курсора:",
            "Cursor theme:" => "Тема курсора:",
            "Custom..." => "Свой вариант...",
            "Decoration:" => "Декорация:",
            "Decorations" => "Оформление окон",
            "Default (2, 1.1)" => "По умолчанию (2, 1.1)",
            "Default mode:" => "Режим по умолчанию:",
            "Delete" => "Удалить",
            "Device Default" => "По умолчанию для устройства",
            "Disable Direct Scanout" => "Отключить Direct Scanout",
            "Distance (px):" => "Дистанция (px):",
            "Effects" => "Эффекты",
            "Enable blur:" => "Включить размытие:",
            "Enable snapping:" => "Включить прилипание:",
            "English" => "English",
            "Enter commands to run at startup (one per line):" => {
                "Введите команды для запуска при старте (по одной на строку):"
            }
            "Environment Variables" => "Переменные окружения",
            "Extreme (10, 2.0)" => "Экстремальное (10, 2.0)",
            "Fit padding (px):" => "Отступы при вмещении (px):",
            "Flat" => "Плоский (Flat)",
            "Focus follows mouse:" => "Фокус следует за мышью:",
            "Force EGL composition (disable direct scanout)" => {
                "Принудительный EGL-композитинг (отключает direct scanout)"
            }
            "Foreground color:" => "Цвет текста:",
            "Friction:" => "Трение:",
            "G:" => "G:",
            "Gap (px):" => "Отступ (px):",
            "General" => "Общие",
            "General Settings" => "Общие настройки",
            "Generate & Save Shader" => "Сгенерировать и сохранить шейдер",
            "Glow Intensity:" => "Интенсивность свечения:",
            "Gradient" => "Градиент",
            "Inactive opacity:" => "Неактивная прозрачность:",
            "Keybindings" => "Горячие клавиши",
            "Keyboard" => "Клавиатура",
            "Keyboard Settings" => "Настройки клавиатуры",
            "Language:" => "Язык:",
            "Theme:" => "Тема:",
            "Light" => "Светлая",
            "Dark" => "Темная",
            "System" => "Системная",
            "GitHub Repository" => "Репозиторий GitHub",
            "Layout independent keybindings:" => "Горячие клавиши не зависят от раскладки:",
            "Layout:" => "Раскладка:",
            "Light (1, 0.8)" => "Слабое (1, 0.8)",
            "Medium (4, 1.3)" => "Среднее (4, 1.3)",
            "Modifier key:" => "Клавиша-модификатор:",
            "Mouse" => "Мышь",
            "Mouse Settings" => "Настройки мыши",
            "Mouse speed:" => "Скорость мыши:",
            "Natural scroll:" => "Естественная прокрутка:",
            "Navigation" => "Навигация",
            "Navigation Settings" => "Настройки навигации",
            "None (0, 0)" => "Выкл (0, 0)",
            "None (Bare)" => "Нет (голые)",
            "None (SSD)" => "Нет (SSD)",
            "Nudge step (px):" => "Шаг перемещения (px):",
            "Num Lock on startup:" => "Num Lock при запуске:",
            "Opacity:" => "Непрозрачность:",
            "Open Shader Editor →" => "Открыть редактор шейдеров →",
            "Options:" => "Опции:",
            "Pan step (px):" => "Шаг прокрутки (px):",
            "Pass Keys:" => "Передавать клавиши:",
            "Pattern Scale:" => "Масштаб узора:",
            "Primary Color:" => "Основной цвет:",
            "R:" => "R:",
            "Raw Mode" => "Сырой режим (Код)",
            "Repeat delay (ms):" => "Задержка повтора (мс):",
            "Repeat rate (keys/sec):" => "Скорость повтора (наж/сек):",
            "Requires restart to fully apply" => "Для применения требуется перезапуск",
            "Reset zoom on activation:" => "Сброс масштаба при активации:",
            "Reset zoom on new window:" => "Сброс масштаба при новом окне:",
            "Save" => "Сохранить",
            "Secondary Color:" => "Вторичный цвет:",
            "Server" => "Серверные",
            "Shader Editor" => "Редактор шейдеров",
            "Shader Template" => "Шаблон шейдера",
            "Shader path:" => "Путь к шейдеру:",
            "Smaller = zoomed in, Larger = zoomed out" => "Меньше = приближено, Больше = отдалено",
            "Snap" => "Прилипание",
            "Snap Settings" => "Настройки прилипания окон",
            "Snap same edges:" => "Прилипать к одинаковым краям:",
            "Strong (6, 1.5)" => "Сильное (6, 1.5)",
            "Super (Windows key)" => "Super (клавиша Windows)",
            "Super+Space to switch layout" => "Super+Space для переключения раскладки",
            "Tap and drag:" => "Нажатие и перетаскивание:",
            "Tap to click:" => "Нажатие для клика:",
            "Tile path:" => "Путь к замостителю (изображению):",
            "Title:" => "Заголовок:",
            "Trackpad" => "Тачпад",
            "Trackpad Settings" => "Настройки тачпада",
            "Trackpad speed:" => "Скорость тачпада:",
            "Variant:" => "Вариант (Variant):",
            "Vignette:" => "Виньетка:",
            "Visual Effects" => "Визуальные эффекты",
            "Visual Mode" => "Визуальный режим",
            "WM_CLASS class (XWayland)" => "Класс WM_CLASS (XWayland)",
            "WM_CLASS instance (XWayland)" => "Экземпляр WM_CLASS (XWayland)",
            "Wait for Frame Completion" => "Ожидать завершения кадра",
            "Wait for GPU fences before page flip" => "Ожидать GPU перед переворачиванием страницы",
            "Widget (pinned):" => "Виджет (закреплен):",
            "Window Decorations" => "Декорации окон",
            "Window Rules" => "Правила окон",
            "Window title (supports * glob)" => "Заголовок окна (поддерживает маски *)",
            "X11 Class:" => "X11 Class:",
            "X11 Instance:" => "X11 Instance:",
            "Yaru" => "Yaru",
            "Zoom" => "Масштабирование",
            "Zoom Settings" => "Настройки масштабирования",
            "Zoom step:" => "Шаг масштабирования:",
            "driftwm-settings" => "driftwm-settings",
            "dvorak" => "dvorak",
            "e.g., Alacritty, firefox" => "напр. Alacritty, firefox",
            "e.g., ctrl, shift" => "напр. ctrl, shift",
            "exec alacritty" => "exec alacritty",
            "super+t" => "super+t",
            "true/false or mod+q, ctrl+q" => "true/false или mod+q, ctrl+q",
            "us,ru" => "us,ru",
            "~/.config/driftwm/bg.frag" => "~/.config/driftwm/bg.frag",
            "~/.config/driftwm/tile.png" => "~/.config/driftwm/tile.png",
            "Русский" => "Русский",
            "−" => "−",
            "⏺" => "⏺",
            "Blur effects are applied to windows with blur enabled in window rules.\nHigher values increase blur quality but may impact performance." => {
                "Эффекты размытия применяются к окнам с включенным размытием в правилах окон.\nБолее высокие значения улучшают качество размытия, но могут повлиять на производительность."
            }
            "Controls blur intensity through multiple passes (0 = disabled, 2 = default, 10+ = very strong)" => {
                "Управляет интенсивностью размытия через количество проходов (0 = выключено, 2 = по умолчанию, 10+ = очень сильное)"
            }
            "Controls blur spread per pass (0.5 = tight, 1.1 = default, 3.0+ = very wide)" => {
                "Управляет разбросом размытия за один проход (0.5 = узкое, 1.1 = по умолчанию, 3.0+ = очень широкое)"
            }
            "Note: To enable blur for specific windows, add window rules with blur = true and opacity < 1.0" => {
                "Примечание: Чтобы включить размытие для конкретных окон, добавьте правила окон с blur = true и opacity < 1.0"
            }
            "Configure per-window settings like blur, opacity, position, and decorations.\nFind app_id: cat $XDG_RUNTIME_DIR/driftwm/state" => {
                "Настройте параметры для каждого окна, такие как размытие, прозрачность, положение и декорации.\nУзнать app_id: cat $XDG_RUNTIME_DIR/driftwm/state"
            }
            "Hardware stability quirks. All default to false (opt-in).\nEnable these if you experience flickering, crashes, or rendering issues.\nParticularly useful on NVIDIA GPUs with proprietary drivers." => {
                "Исправления для стабильности оборудования. Все по умолчанию выключены (false).\nВключите их, если вы столкнулись с мерцанием, вылетами или проблемами с рендерингом.\nОсобенно полезно на видеокартах NVIDIA с проприетарными драйверами."
            }
            "Note: These flags must be set before launching driftwm. Changing them requires a restart." => {
                "Примечание: Эти флаги должны быть установлены до запуска driftwm. Изменение требует перезапуска."
            }
            "For additional NVIDIA-specific settings, set these environment variables\nin your session wrapper script or shell profile before starting driftwm:\n\n  export SMITHAY_USE_LEGACY=1          # Use legacy DRM API instead of atomic modesetting\n  export __GL_GSYNC_ALLOWED=0\n  export __GL_VRR_ALLOWED=0\n  export __GL_MaxFramesAllowed=1\n  export NVD_BACKEND=direct" => {
                "Для дополнительных настроек NVIDIA установите эти переменные окружения\nв скрипте запуска сессии или профиле оболочки перед запуском driftwm:\n\n  export SMITHAY_USE_LEGACY=1          # Использовать устаревший DRM API вместо atomic modesetting\n  export __GL_GSYNC_ALLOWED=0\n  export __GL_VRR_ALLOWED=0\n  export __GL_MaxFramesAllowed=1\n  export NVD_BACKEND=direct"
            }
            "Configure custom keyboard shortcuts. Click + to add new bindings." => {
                "Настройте пользовательские сочетания клавиш. Нажмите +, чтобы добавить новые привязки."
            }
            "Create custom animated backgrounds with visual controls. Switch to Raw mode for advanced editing." => {
                "Создавайте свои анимированные фоны с помощью визуальных элементов управления. Переключитесь в режим Raw для расширенного редактирования."
            }
            "Number of detail layers (higher = more detailed but slower)" => {
                "Количество слоев детализации (выше = детальнее, но медленнее)"
            }
            "// Shader code will appear here when you switch to Raw mode" => {
                "// Код шейдера появится здесь при переключении в режим Raw"
            }
            _ => key,
        };
        return res.to_string();
    }

    key.to_string()
}
