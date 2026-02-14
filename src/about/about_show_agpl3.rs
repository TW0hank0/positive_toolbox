use iced;
use iced::widget::{Column, scrollable, text};

use log;

use positive_toolbox::shared;
use positive_toolbox::shared::FONT_NOTO_SANS_REG;

const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
const TOOL_NAME: &str = "about_show_agpl3";

const LICENSE: &str = include_str!("../../LICENSE");

fn main() -> iced::Result {
    let (icon,) = shared::init();
    //
    let mut window_settings = iced::window::Settings::default();
    window_settings.maximized = true;
    window_settings.icon = icon;
    window_settings.min_size = Some(iced::Size::new(540.0, 360.0));
    //
    let mut app_settings = iced::Settings::default();
    app_settings.id = Some(String::from(env!("CARGO_PKG_NAME")));
    app_settings.default_text_size = iced::Pixels::from(26);
    app_settings.default_font = FONT_NOTO_SANS_REG;
    //
    log::info!("啟動iced");
    iced::application(About::new, About::update, About::view)
        .theme(About::theme)
        .title(About::title)
        .window(window_settings)
        .default_font(FONT_NOTO_SANS_REG)
        .settings(app_settings)
        .run()
}

#[derive(Default)]
pub struct About {}

#[derive(Debug, Clone)]
pub enum AboutMsg {}

impl About {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn update(&mut self, _message: AboutMsg) {}

    pub fn view(&self) -> Column<'_, AboutMsg> {
        let mut layout = Column::new()
            .padding(5)
            .align_x(iced::alignment::Horizontal::Left)
            .width(iced::Length::Fill);
        let license_text = text(LICENSE).size(18).color(iced::Color::BLACK);
        let scrollable_license_text = scrollable(license_text)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill);
        layout = layout.push(scrollable_license_text).spacing(10);
        return layout;
    }

    pub fn title(&self) -> String {
        return String::from(format!("{} — {}", TOOL_NAME, PROJECT_NAME));
    }

    pub fn theme(&self) -> Option<iced::Theme> {
        Some(iced::Theme::Dark)
    }
}
