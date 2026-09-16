pub mod appearance;
pub mod background;
pub mod bookmarks;
pub mod general;
pub mod input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    General,
    Appearance,
    Background,
    Bookmarks,
    Input,
}

impl Tab {
    pub const ALL: [Tab; 5] = [
        Tab::General,
        Tab::Appearance,
        Tab::Background,
        Tab::Bookmarks,
        Tab::Input,
    ];

    pub fn title(&self) -> &'static str {
        match self {
            Tab::General => "General & Placement",
            Tab::Appearance => "Appearance & Deco",
            Tab::Background => "Background",
            Tab::Bookmarks => "Bookmarks & Nav",
            Tab::Input => "Input Devices",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Tab::General => "⚙",
            Tab::Appearance => "🎨",
            Tab::Background => "🖼",
            Tab::Bookmarks => "🔖",
            Tab::Input => "⌨",
        }
    }
}
