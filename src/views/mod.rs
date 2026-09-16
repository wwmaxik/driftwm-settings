pub mod appearance;
pub mod background;
pub mod bookmarks;
pub mod general;
pub mod input;
pub mod keybindings;
pub mod rules;
pub mod settings;

use crate::i18n::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    General,
    Appearance,
    Background,
    Bookmarks,
    Input,
    Keybindings,
    Rules,
    Settings,
}

impl Tab {
    pub const ALL: [Tab; 8] = [
        Tab::General,
        Tab::Appearance,
        Tab::Background,
        Tab::Bookmarks,
        Tab::Input,
        Tab::Keybindings,
        Tab::Rules,
        Tab::Settings,
    ];

    pub fn title(&self, lang: Language) -> &'static str {
        match self {
            Tab::General => lang.tab_general(),
            Tab::Appearance => lang.tab_appearance(),
            Tab::Background => lang.tab_background(),
            Tab::Bookmarks => lang.tab_bookmarks(),
            Tab::Input => lang.tab_input(),
            Tab::Keybindings => lang.tab_keybindings(),
            Tab::Rules => lang.tab_rules(),
            Tab::Settings => lang.tab_settings(),
        }
    }
}
