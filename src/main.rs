mod app_settings;
mod config;
mod i18n;
mod icons;
mod theme;
mod validator;
mod views;

use app_settings::AppSettings;
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
        .window_size(iced::Size::new(1060.0, 780.0))
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
    settings: AppSettings,
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
    Settings(views::settings::SettingsMessage),
    SaveConfig,
    ValidateConfig,
    ReloadConfig,
    DismissBanner,
}

impl Default for App {
    fn default() -> Self {
        let settings = AppSettings::load();
        let path = if let Some(custom) = &settings.custom_config_path {
            PathBuf::from(custom)
        } else {
            DriftwmConfig::default_path()
        };

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
            settings,
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
        let lang = self.settings.language;

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
            Message::Settings(msg) => {
                views::settings::update(&mut self.settings, msg);
                let new_path = if let Some(custom) = &self.settings.custom_config_path {
                    PathBuf::from(custom)
                } else {
                    DriftwmConfig::default_path()
                };
                if new_path != self.config_path {
                    self.config_path = new_path;
                    if let Ok((cfg, doc)) = DriftwmConfig::load(&self.config_path) {
                        self.config = cfg;
                        self.doc = doc;
                    }
                }
            }
            Message::SaveConfig => {
                match self.config.save(&mut self.doc, &self.config_path) {
                    Ok(()) => {
                        if self.settings.auto_validate {
                            let status = validator::ConfigValidator::validate_file(&self.config_path);
                            match status {
                                validator::ValidationStatus::Success { warnings } => {
                                    if warnings.is_empty() {
                                        self.status_banner = Some((
                                            BannerKind::Success,
                                            format!(
                                                "{} ({})",
                                                lang.save_success(),
                                                self.config_path.display()
                                            ),
                                        ));
                                    } else {
                                        self.status_banner = Some((
                                            BannerKind::Warning,
                                            format!(
                                                "{} ({})\n{}",
                                                lang.save_warnings(),
                                                self.config_path.display(),
                                                warnings.join("\n")
                                            ),
                                        ));
                                    }
                                }
                                validator::ValidationStatus::Warning { warnings } => {
                                    self.status_banner = Some((
                                        BannerKind::Warning,
                                        format!(
                                            "{} ({})\n{}",
                                            lang.save_warnings(),
                                            self.config_path.display(),
                                            warnings.join("\n")
                                        ),
                                    ));
                                }
                                validator::ValidationStatus::Error { message } => {
                                    self.status_banner = Some((
                                        BannerKind::Error,
                                        format!("{}\n{}", lang.save_error(), message),
                                    ));
                                }
                            }
                        } else {
                            self.status_banner = Some((
                                BannerKind::Success,
                                format!(
                                    "{} ({})",
                                    lang.save_success(),
                                    self.config_path.display()
                                ),
                            ));
                        }
                    }
                    Err(e) => {
                        self.status_banner = Some((
                            BannerKind::Error,
                            format!("{} {}", lang.save_error(), e),
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
                                lang.check_success().to_string(),
                            ));
                        } else {
                            self.status_banner = Some((
                                BannerKind::Warning,
                                format!(
                                    "{} ({}):\n{}",
                                    lang.save_warnings(),
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
                                "{} ({})",
                                lang.reloaded(),
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
        let lang = self.settings.language;

        // --- Left Sidebar ---
        let brand_header = column![
            row![
                icons::icon_general(theme::mocha::MAUVE, 22.0),
                text("driftwm")
                    .size(22)
                    .color(theme::mocha::TEXT),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(lang.app_subtitle())
                .size(12)
                .color(theme::mocha::SUBTEXT0),
        ]
        .spacing(4);

        let mut nav_tabs = column![].spacing(6);
        for tab in Tab::ALL {
            let is_active = self.current_tab == tab;
            let icon_color = if is_active {
                theme::mocha::MAUVE
            } else {
                theme::mocha::SUBTEXT0
            };

            let icon_el: Element<'static, Message> = match tab {
                Tab::General => icons::icon_general(icon_color, 16.0).into(),
                Tab::Appearance => icons::icon_appearance(icon_color, 16.0).into(),
                Tab::Background => icons::icon_background(icon_color, 16.0).into(),
                Tab::Bookmarks => icons::icon_bookmarks(icon_color, 16.0).into(),
                Tab::Input => icons::icon_input(icon_color, 16.0).into(),
                Tab::Settings => icons::icon_settings(icon_color, 16.0).into(),
            };

            let btn = button(
                row![
                    icon_el,
                    text(tab.title(lang)).size(14),
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
                    icons::icon_save(theme::mocha::BASE, 15.0),
                    text(lang.save_changes()).size(13),
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
                    icons::icon_check(theme::mocha::BASE, 15.0),
                    text(lang.check_config()).size(13),
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
                    icons::icon_reload(theme::mocha::TEXT, 14.0),
                    text(lang.reload_file()).size(13),
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
        .width(Length::Fixed(240.0))
        .height(Length::Fill);

        // --- Main Content Area ---
        let tab_title = row![
            column![
                text(self.current_tab.title(lang))
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

            let icon_widget: Element<'static, Message> = match kind {
                BannerKind::Success => icons::icon_check(theme::mocha::GREEN, 16.0).into(),
                BannerKind::Warning => icons::icon_info(theme::mocha::YELLOW, 16.0).into(),
                BannerKind::Error => icons::icon_trash(theme::mocha::RED, 16.0).into(),
            };

            container(
                row![
                    icon_widget,
                    text(msg.as_str())
                        .size(13)
                        .width(Length::Fill),
                    button(text("✕").size(11))
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
            Tab::General => views::general::view(&self.config, lang).map(Message::General),
            Tab::Appearance => views::appearance::view(&self.config, lang).map(Message::Appearance),
            Tab::Background => views::background::view(&self.config, lang).map(Message::Background),
            Tab::Bookmarks => {
                views::bookmarks::view(&self.config, &self.bookmarks_state, lang).map(Message::Bookmarks)
            }
            Tab::Input => views::input::view(&self.config, lang).map(Message::Input),
            Tab::Settings => views::settings::view(&self.settings).map(Message::Settings),
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
