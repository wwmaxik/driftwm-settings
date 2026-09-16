mod config;
mod theme;
mod validator;
mod views;

use config::DriftwmConfig;
use std::path::PathBuf;
use toml_edit::DocumentMut;
use views::bookmarks::BookmarksState;
use views::Tab;

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length, Theme};

#[cfg(all(target_os = "linux", target_env = "gnu"))]
core::arch::global_asm!(
    ".symver acosf, acosf@GLIBC_2.2.5",
    ".symver atan2f, atan2f@GLIBC_2.2.5",
    ".symver hypotf, hypotf@GLIBC_2.2.5",
);

fn main() -> iced::Result {
    iced::application("driftwm settings", App::update, App::view)
        .theme(App::theme)
        .window_size(iced::Size::new(1040.0, 760.0))
        .run()
}


#[derive(Debug, Clone, PartialEq)]
enum BannerKind {
    Success,
    Warning,
    Error,
}

struct App {
    config: DriftwmConfig,
    doc: DocumentMut,
    config_path: PathBuf,
    current_tab: Tab,
    bookmarks_state: BookmarksState,
    status_banner: Option<(BannerKind, String)>,
}

#[derive(Debug, Clone)]
enum Message {
    TabSelected(Tab),
    General(views::general::GeneralMessage),
    Appearance(views::appearance::AppearanceMessage),
    Background(views::background::BackgroundMessage),
    Bookmarks(views::bookmarks::BookmarksMessage),
    Input(views::input::InputMessage),
    SaveConfig,
    ValidateConfig,
    ReloadConfig,
    DismissBanner,
}

impl Default for App {
    fn default() -> Self {
        let path = DriftwmConfig::default_path();
        let (config, doc) = DriftwmConfig::load(&path).unwrap_or_else(|_| {
            (
                DriftwmConfig::default_with_presets(),
                DriftwmConfig::create_default_document(),
            )
        });

        Self {
            config,
            doc,
            config_path: path,
            current_tab: Tab::General,
            bookmarks_state: BookmarksState::default(),
            status_banner: None,
        }
    }
}

impl App {
    fn theme(&self) -> Theme {
        Theme::CatppuccinMocha
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab) => {
                self.current_tab = tab;
            }
            Message::General(msg) => {
                views::general::update(&mut self.config, msg);
            }
            Message::Appearance(msg) => {
                views::appearance::update(&mut self.config, msg);
            }
            Message::Background(msg) => {
                views::background::update(&mut self.config, msg);
            }
            Message::Bookmarks(msg) => {
                views::bookmarks::update(&mut self.config, &mut self.bookmarks_state, msg);
            }
            Message::Input(msg) => {
                views::input::update(&mut self.config, msg);
            }
            Message::SaveConfig => {
                match self.config.save(&mut self.doc, &self.config_path) {
                    Ok(()) => {
                        let status = validator::ConfigValidator::validate_file(&self.config_path);
                        match status {
                            validator::ValidationStatus::Success { warnings } => {
                                if warnings.is_empty() {
                                    self.status_banner = Some((
                                        BannerKind::Success,
                                        format!(
                                            "Configuration successfully saved to {} and verified!",
                                            self.config_path.display()
                                        ),
                                    ));
                                } else {
                                    self.status_banner = Some((
                                        BannerKind::Warning,
                                        format!(
                                            "Saved to {}, but driftwm reported {} warning(s):\n{}",
                                            self.config_path.display(),
                                            warnings.len(),
                                            warnings.join("\n")
                                        ),
                                    ));
                                }
                            }
                            validator::ValidationStatus::Warning { warnings } => {
                                self.status_banner = Some((
                                    BannerKind::Warning,
                                    format!(
                                        "Saved to {}, but with {} warning(s):\n{}",
                                        self.config_path.display(),
                                        warnings.len(),
                                        warnings.join("\n")
                                    ),
                                ));
                            }
                            validator::ValidationStatus::Error { message } => {
                                self.status_banner = Some((
                                    BannerKind::Error,
                                    format!("File saved, but validation failed:\n{}", message),
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        self.status_banner = Some((
                            BannerKind::Error,
                            format!("Failed to save configuration: {}", e),
                        ));
                    }
                }
            }
            Message::ValidateConfig => {
                let status = validator::ConfigValidator::validate_file(&self.config_path);
                match status {
                    validator::ValidationStatus::Success { warnings } => {
                        if warnings.is_empty() {
                            self.status_banner = Some((
                                BannerKind::Success,
                                "Config OK: No errors or warnings detected!".to_string(),
                            ));
                        } else {
                            self.status_banner = Some((
                                BannerKind::Warning,
                                format!(
                                    "Config OK with {} warning(s):\n{}",
                                    warnings.len(),
                                    warnings.join("\n")
                                ),
                            ));
                        }
                    }
                    validator::ValidationStatus::Warning { warnings } => {
                        self.status_banner = Some((
                            BannerKind::Warning,
                            format!(
                                "Validation Warning ({}):\n{}",
                                warnings.len(),
                                warnings.join("\n")
                            ),
                        ));
                    }
                    validator::ValidationStatus::Error { message } => {
                        self.status_banner = Some((
                            BannerKind::Error,
                            format!("Validation Error:\n{}", message),
                        ));
                    }
                }
            }
            Message::ReloadConfig => {
                match DriftwmConfig::load(&self.config_path) {
                    Ok((cfg, doc)) => {
                        self.config = cfg;
                        self.doc = doc;
                        self.status_banner = Some((
                            BannerKind::Success,
                            format!(
                                "Configuration reloaded from {}",
                                self.config_path.display()
                            ),
                        ));
                    }
                    Err(e) => {
                        self.status_banner = Some((
                            BannerKind::Error,
                            format!("Failed to reload config: {}", e),
                        ));
                    }
                }
            }
            Message::DismissBanner => {
                self.status_banner = None;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // --- Left Sidebar ---
        let brand_header = column![
            row![
                text("⌘").size(24).color(theme::mocha::MAUVE),
                text("driftwm")
                    .size(22)
                    .color(theme::mocha::TEXT),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text("Settings Utility")
                .size(12)
                .color(theme::mocha::SUBTEXT0),
        ]
        .spacing(4);

        let mut nav_tabs = column![].spacing(6);
        for tab in Tab::ALL {
            let is_active = self.current_tab == tab;
            let btn = button(
                row![
                    text(tab.icon()).size(16),
                    text(tab.title()).size(14),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            )
            .on_press(Message::TabSelected(tab))
            .style(theme::nav_button_style(is_active))
            .width(Length::Fill)
            .padding(10);

            nav_tabs = nav_tabs.push(btn);
        }

        let sidebar_actions = column![
            button(
                row![
                    text("💾").size(15),
                    text("Save Changes").size(13),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .on_press(Message::SaveConfig)
            .style(theme::success_button_style)
            .width(Length::Fill)
            .padding(10),

            button(
                row![
                    text("✓").size(15),
                    text("Check Config").size(13),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .on_press(Message::ValidateConfig)
            .style(theme::primary_button_style)
            .width(Length::Fill)
            .padding(10),

            button(
                row![
                    text("↻").size(15),
                    text("Reload File").size(13),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .on_press(Message::ReloadConfig)
            .style(theme::secondary_button_style)
            .width(Length::Fill)
            .padding(8),
        ]
        .spacing(8);

        let sidebar = container(
            column![
                brand_header,
                nav_tabs,
                column![].height(Length::Fill),
                sidebar_actions,
            ]
            .spacing(16)
            .height(Length::Fill),
        )
        .style(theme::sidebar_style)
        .padding(16)
        .width(Length::Fixed(230.0))
        .height(Length::Fill);

        // --- Main Content Area ---
        let tab_title = row![
            column![
                text(self.current_tab.title())
                    .size(24)
                    .color(theme::mocha::MAUVE),
                text(format!("driftwm 0.19+ • {}", self.config_path.display()))
                    .size(12)
                    .color(theme::mocha::OVERLAY0),
            ]
            .spacing(4),
        ]
        .width(Length::Fill);

        let banner_view: Option<Element<Message>> = self.status_banner.as_ref().map(|(kind, msg)| {
            let style = match kind {
                BannerKind::Success => theme::alert_success_style,
                BannerKind::Warning => theme::alert_warning_style,
                BannerKind::Error => theme::alert_error_style,
            };

            let prefix = match kind {
                BannerKind::Success => "✓ ",
                BannerKind::Warning => "⚠ ",
                BannerKind::Error => "✕ ",
            };

            container(
                row![
                    text(format!("{}{}", prefix, msg))
                        .size(13)
                        .width(Length::Fill),
                    button(text("Dismiss").size(11))
                        .on_press(Message::DismissBanner)
                        .style(theme::secondary_button_style),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .padding(12)
            .style(style)
            .width(Length::Fill)
            .into()
        });

        let tab_content: Element<Message> = match self.current_tab {
            Tab::General => views::general::view(&self.config).map(Message::General),
            Tab::Appearance => views::appearance::view(&self.config).map(Message::Appearance),
            Tab::Background => views::background::view(&self.config).map(Message::Background),
            Tab::Bookmarks => {
                views::bookmarks::view(&self.config, &self.bookmarks_state).map(Message::Bookmarks)
            }
            Tab::Input => views::input::view(&self.config).map(Message::Input),
        };

        let mut content_column = column![tab_title].spacing(16);

        if let Some(banner) = banner_view {
            content_column = content_column.push(banner);
        }

        content_column = content_column.push(scrollable(tab_content).height(Length::Fill));

        let main_body = container(content_column)
            .padding(24)
            .width(Length::Fill)
            .height(Length::Fill);

        row![sidebar, main_body]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
