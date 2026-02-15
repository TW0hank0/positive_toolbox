use std;

use iced;
use iced::widget::{Column, Row, button, scrollable, text};

//use image;

use log;
//use log::{debug, error, info, trace, warn};

use positive_toolbox;
use positive_toolbox::shared;
use positive_toolbox::shared::FONT_NOTO_SANS_REG;

const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
const TOOL_NAME: &str = "about";

//const LICENSE: &str = include_str!("../../LICENSE");

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
pub enum AboutMsg {
    OpenLicense,
}

#[derive(Debug, Clone)]
pub enum Licenses {
    Agpl3,
    Apache2,
    MIT,
}

impl std::fmt::Display for Licenses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Agpl3 => "AGPL-3",
            Self::Apache2 => "Apache-2",
            Self::MIT => "MIT",
        })
    }
}
impl About {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn update(&mut self, message: AboutMsg) {
        match message {
            AboutMsg::OpenLicense => {
                let mut tool_path = std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("about_show_full_license");
                #[cfg(target_os = "windows")]
                {
                    tool_path =
                        std::path::PathBuf::from(format!("{}.exe", tool_path.to_str().unwrap()));
                }
                let _ = std::process::Command::new(tool_path).spawn();
            }
        }
    }

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
        let mut layout_license = Column::new().padding(15);
        layout_license = layout_license.push(create_license_info(
            String::from(shared::PROJECT_NAME),
            vec![String::from("TW0hank0")],
            String::from("AGPL-3.0"),
        ));
        //
        let mut layout_third_party = Column::new().padding(15);
        layout_third_party =
            layout_third_party.push(button("開啟完整內容").on_press(AboutMsg::OpenLicense));
        let third_party_license_infos = positive_toolbox::licenses::get_licenses();
        for license_info in third_party_license_infos {
            let mut authors = Vec::new();
            for author in license_info.authors {
                authors.push(String::from(author));
            }
            layout_third_party = layout_third_party.push(create_license_info(
                String::from(license_info.name),
                authors,
                String::from(license_info.license),
            ));
        }
        layout_license = layout_license.push(layout_third_party);
        let scrollable_license = scrollable(layout_license);
        //let scrollable_third_party = scrollable(layout_third_party);
        //layout = layout.push(scrollable_third_party);
        //
        layout = layout.push(scrollable_license);
        //
        return layout;
    }

    pub fn title(&self) -> String {
        return String::from(format!("{} — {}", TOOL_NAME, PROJECT_NAME));
    }

    pub fn theme(&self) -> Option<iced::Theme> {
        Some(iced::Theme::Dark)
    }
}

pub fn create_license_info(
    project_name: String,
    authors: Vec<String>,
    license_string: String,
) -> Column<'static, AboutMsg> {
    let mut layout = Column::new().padding(10);
    layout = layout
        .push(
            text(project_name)
                .size(24)
                .font(shared::FONT_NOTO_SANS_BOLD),
        )
        .spacing(20);
    layout = layout
        .push(text(format!("authors: {:?}", authors)).size(20))
        .spacing(20);
    //
    let binding = license_string.replace(" ", "");
    let mut license_vec: Vec<&str> = binding.split("OR").collect();
    license_vec.sort();
    let mut licenses: Vec<Licenses> = Vec::new();
    for license in license_vec {
        if license.starts_with("AGPL") {
            licenses.push(Licenses::Agpl3);
        } else if license.starts_with("Apache") {
            licenses.push(Licenses::Apache2);
        } else if license.starts_with("MIT") {
            licenses.push(Licenses::MIT);
        }
    }
    layout = layout.push(text(format!("license: {:?}", licenses)).size(20));
    /* let mut license_btn_layout = Row::new().padding(5);
    for license in licenses {
        match license {
            Licenses::Agpl3 => {
                let btn = button(text(format!("{}", license)).size(18))
                    .on_press(AboutMsg::OpenLicense(license))
                    .width(100)
                    .height(60);
                license_btn_layout = license_btn_layout.push(btn);
            }
        }
    }
    layout = layout.push(license_btn_layout); */
    //
    return layout;
}
