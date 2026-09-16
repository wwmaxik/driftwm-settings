#![allow(dead_code)]

use iced::widget::{button, container, pick_list, slider, text_input, toggler};
use iced::{border::Radius, Border, Color, Theme};

pub mod mocha {
    use iced::Color;

    pub const BASE: Color = Color::from_rgb(0.118, 0.118, 0.180); // #1e1e2e
    pub const MANTLE: Color = Color::from_rgb(0.094, 0.094, 0.145); // #181825
    pub const CRUST: Color = Color::from_rgb(0.067, 0.067, 0.106); // #11111b

    pub const SURFACE0: Color = Color::from_rgb(0.192, 0.196, 0.267); // #313244
    pub const SURFACE1: Color = Color::from_rgb(0.271, 0.278, 0.353); // #45475a
    pub const SURFACE2: Color = Color::from_rgb(0.345, 0.357, 0.439); // #585b70

    pub const OVERLAY0: Color = Color::from_rgb(0.424, 0.439, 0.525); // #6c7086
    pub const SUBTEXT0: Color = Color::from_rgb(0.651, 0.678, 0.784); // #a6adc8
    pub const TEXT: Color = Color::from_rgb(0.804, 0.839, 0.957); // #cdd6f4

    pub const LAVENDER: Color = Color::from_rgb(0.706, 0.745, 0.996); // #b4befe
    pub const BLUE: Color = Color::from_rgb(0.537, 0.706, 0.980); // #89b4fa
    pub const MAUVE: Color = Color::from_rgb(0.796, 0.651, 0.969); // #cba6f7
    pub const GREEN: Color = Color::from_rgb(0.651, 0.890, 0.631); // #a6e3a1
    pub const PEACH: Color = Color::from_rgb(0.980, 0.702, 0.529); // #fab387
    pub const RED: Color = Color::from_rgb(0.953, 0.545, 0.659); // #f38ba8
    pub const YELLOW: Color = Color::from_rgb(0.976, 0.886, 0.686); // #f9e2af
    pub const TEAL: Color = Color::from_rgb(0.580, 0.886, 0.835); // #94e2d5
}

/// Container style for content cards
pub fn card_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(mocha::TEXT),
        background: Some(mocha::SURFACE0.into()),
        border: Border {
            color: mocha::SURFACE1,
            width: 1.0,
            radius: Radius::from(10.0),
        },
        shadow: Default::default(),
    }
}

/// Sidebar container style
pub fn sidebar_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(mocha::TEXT),
        background: Some(mocha::MANTLE.into()),
        border: Border {
            color: mocha::SURFACE0,
            width: 1.0,
            radius: Radius::from(0.0),
        },
        shadow: Default::default(),
    }
}

/// Header bar container style
pub fn header_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(mocha::TEXT),
        background: Some(mocha::MANTLE.into()),
        border: Border {
            color: mocha::SURFACE0,
            width: 1.0,
            radius: Radius::from(0.0),
        },
        shadow: Default::default(),
    }
}

/// Alert banner container styles
pub fn alert_success_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(mocha::GREEN),
        background: Some(Color::from_rgba(0.651, 0.890, 0.631, 0.12).into()),
        border: Border {
            color: mocha::GREEN,
            width: 1.0,
            radius: Radius::from(8.0),
        },
        shadow: Default::default(),
    }
}

pub fn alert_warning_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(mocha::YELLOW),
        background: Some(Color::from_rgba(0.976, 0.886, 0.686, 0.12).into()),
        border: Border {
            color: mocha::YELLOW,
            width: 1.0,
            radius: Radius::from(8.0),
        },
        shadow: Default::default(),
    }
}

pub fn alert_error_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(mocha::RED),
        background: Some(Color::from_rgba(0.953, 0.545, 0.659, 0.12).into()),
        border: Border {
            color: mocha::RED,
            width: 1.0,
            radius: Radius::from(8.0),
        },
        shadow: Default::default(),
    }
}

/// Sidebar tab button style
pub fn nav_button_style(is_active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |_theme: &Theme, status: button::Status| {
        if is_active {
            button::Style {
                background: Some(mocha::MAUVE.into()),
                text_color: mocha::CRUST,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(8.0),
                },
                shadow: Default::default(),
            }
        } else {
            let bg = match status {
                button::Status::Hovered => Some(mocha::SURFACE0.into()),
                button::Status::Pressed => Some(mocha::SURFACE1.into()),
                _ => None,
            };
            button::Style {
                background: bg,
                text_color: mocha::TEXT,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(8.0),
                },
                shadow: Default::default(),
            }
        }
    }
}

/// Primary button style (Mauve)
pub fn primary_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let (bg, text_color) = match status {
        button::Status::Hovered => (mocha::LAVENDER, mocha::CRUST),
        button::Status::Pressed => (mocha::BLUE, mocha::CRUST),
        button::Status::Disabled => (mocha::SURFACE1, mocha::OVERLAY0),
        _ => (mocha::MAUVE, mocha::CRUST),
    };

    button::Style {
        background: Some(bg.into()),
        text_color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(8.0),
        },
        shadow: Default::default(),
    }
}

/// Success button style (Green)
pub fn success_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Hovered => Color::from_rgb(0.72, 0.94, 0.70),
        button::Status::Pressed => mocha::TEAL,
        _ => mocha::GREEN,
    };

    button::Style {
        background: Some(bg.into()),
        text_color: mocha::CRUST,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(8.0),
        },
        shadow: Default::default(),
    }
}

/// Secondary/Outline button style
pub fn secondary_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Hovered => Some(mocha::SURFACE1.into()),
        button::Status::Pressed => Some(mocha::SURFACE2.into()),
        _ => Some(mocha::SURFACE0.into()),
    };

    button::Style {
        background: bg,
        text_color: mocha::TEXT,
        border: Border {
            color: mocha::SURFACE1,
            width: 1.0,
            radius: Radius::from(8.0),
        },
        shadow: Default::default(),
    }
}

/// Danger button style
pub fn danger_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Hovered => Color::from_rgb(0.98, 0.60, 0.70),
        button::Status::Pressed => mocha::RED,
        _ => Color::from_rgba(0.953, 0.545, 0.659, 0.2),
    };

    button::Style {
        background: Some(bg.into()),
        text_color: mocha::RED,
        border: Border {
            color: mocha::RED,
            width: 1.0,
            radius: Radius::from(8.0),
        },
        shadow: Default::default(),
    }
}

/// Text input style
pub fn input_style(_theme: &Theme, status: text_input::Status) -> text_input::Style {
    let border_color = match status {
        text_input::Status::Focused => mocha::MAUVE,
        text_input::Status::Hovered => mocha::SURFACE2,
        _ => mocha::SURFACE1,
    };

    text_input::Style {
        background: mocha::SURFACE0.into(),
        border: Border {
            color: border_color,
            width: 1.0,
            radius: Radius::from(6.0),
        },
        icon: mocha::SUBTEXT0,
        placeholder: mocha::OVERLAY0,
        value: mocha::TEXT,
        selection: mocha::SURFACE2,
    }
}

pub use input_style as text_input_style;

/// Dropdown pick list style
pub fn pick_list_style(_theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let border_color = match status {
        pick_list::Status::Hovered | pick_list::Status::Opened => mocha::MAUVE,
        _ => mocha::SURFACE1,
    };

    pick_list::Style {
        text_color: mocha::TEXT,
        placeholder_color: mocha::OVERLAY0,
        handle_color: mocha::SUBTEXT0,
        background: mocha::SURFACE0.into(),
        border: Border {
            color: border_color,
            width: 1.0,
            radius: Radius::from(6.0),
        },
    }
}

/// Slider style
pub fn slider_style(_theme: &Theme, _status: slider::Status) -> slider::Style {
    slider::Style {
        rail: slider::Rail {
            backgrounds: (mocha::MAUVE.into(), mocha::SURFACE1.into()),
            width: 5.0,
            border: Border::default(),
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 8.0 },
            background: mocha::LAVENDER.into(),
            border_width: 1.0,
            border_color: mocha::SURFACE0,
        },
    }
}

/// Toggler style
pub fn toggler_style(_theme: &Theme, status: toggler::Status) -> toggler::Style {
    let is_toggled = match status {
        toggler::Status::Active { is_toggled } | toggler::Status::Hovered { is_toggled } => is_toggled,
        toggler::Status::Disabled => false,
    };

    if is_toggled {
        toggler::Style {
            background: mocha::MAUVE,
            background_border_width: 1.0,
            background_border_color: mocha::MAUVE,
            foreground: mocha::BASE,
            foreground_border_width: 0.0,
            foreground_border_color: Color::TRANSPARENT,
        }
    } else {
        toggler::Style {
            background: mocha::SURFACE0,
            background_border_width: 1.0,
            background_border_color: mocha::SURFACE1,
            foreground: mocha::SUBTEXT0,
            foreground_border_width: 0.0,
            foreground_border_color: Color::TRANSPARENT,
        }
    }
}
