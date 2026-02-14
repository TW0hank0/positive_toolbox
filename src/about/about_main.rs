use iced;
use iced::widget::{Column, Row, scrollable, text};

//use image;

use log;
//use log::{debug, error, info, trace, warn};

use positive_toolbox::shared;
use positive_toolbox::shared::FONT_NOTO_SANS_REG;

const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
const TOOL_NAME: &str = "about";

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
        let mut layout_title = Row::new()
            .padding(10)
            .align_y(iced::alignment::Vertical::Bottom)
            .height(90);
        layout_title = layout_title.push(
            text(TOOL_NAME)
                .size(50)
                .align_x(iced::alignment::Horizontal::Left)
                .align_y(iced::alignment::Vertical::Bottom)
                .height(90),
        );
        layout_title = layout_title.spacing(10);
        layout_title = layout_title.push(
            text(format!("from {PROJECT_NAME}"))
                .size(20)
                .align_x(iced::alignment::Horizontal::Left)
                .align_y(iced::alignment::Vertical::Bottom)
                .height(90),
        );
        layout = layout.push(layout_title);
        layout = layout.spacing(60);
        //
        layout = layout.push(text(format!(
            "{} v{}",
            shared::PROJECT_NAME,
            shared::PROJECT_VERSION
        )));
        //
        let mut layout_license = Column::new().padding(5);
        let license_tip_text = text(format!("{} AGPL-3.0協議", shared::PROJECT_NAME));
        layout_license = layout_license.push(license_tip_text);
        let license_text = text(LICENSE).size(18).color(iced::Color::BLACK);
        layout_license = layout_license.push(license_text);
        //
        let scrollable_license_text = scrollable(layout_license)
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
