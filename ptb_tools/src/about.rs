// SPDX-License-Identifier: AGPL-3.0-only
// 著作權所有 (C) 2026 TW0hank0
//
// 本檔案屬於 positive_toolbox 專案的一部分。
// 專案儲存庫：https://github.com/TW0hank0/positive_toolbox
//
// 本程式為自由軟體：您可以根據自由軟體基金會發佈的 GNU Affero 通用公共授權條款
// 第 3 版（僅此版本）重新發佈及/或修改本程式。
//
// 本程式的發佈是希望它能發揮功用，但不提供任何擔保；
// 甚至沒有隱含的適銷性或特定目的適用性擔保。詳見 GNU Affero 通用公共授權條款。
//
// 您應該已經收到一份 GNU Affero 通用公共授權條款副本。
// 如果沒有，請參見 <https://www.gnu.org/licenses/>。

use std::{self, process};

use iced::widget::{Column, Row, button, scrollable, text};
use iced::{self, Length, widget};

use serde;

use log;

use ptb_shared::shared::{self, AboutMsg, PROJECT_NAME, ToolBoxMsg};

const TOOL_NAME: &str = "about";

const THIRD_PARTY_LICENSE_RUST: &str =
    include_str!("../../auto_generated/ThirdPartyLicense-Rust.json");
const THIRD_PARTY_LICENSE_PYTHON: &str =
    include_str!("../../auto_generated/ThirdPartyLicense-Python.json");

/*fn main() -> iced::Result {
    let (icon,) = shared::init();
    //
    let mut window_settings = iced::window::Settings::default();
    window_settings.maximized = true;
    window_settings.icon = icon;
    window_settings.min_size = Some(iced::Size::new(1080.0, 720.0));
    //
    let mut app_settings = iced::Settings::default();
    app_settings.id = Some(String::from(PROJECT_NAME));
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
}*/

#[derive(Debug, Clone)]
pub enum Licenses {
    AGPL3,
    Apache2,
    MIT,
    BSD2Clause,
    BSD3Clause,
    Other(String),
}

impl std::fmt::Display for Licenses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::AGPL3 => "AGPL-3",
            Self::Apache2 => "Apache-2",
            Self::MIT => "MIT",
            Self::BSD2Clause => "BSD-2-Clause",
            Self::BSD3Clause => "BSD-3-Clause",
            Self::Other(string) => string,
        })
    }
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct LicenseInfo {
    pub dependencies: Vec<LicenseDep>,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct LicenseDep {
    pub license_id: String,
    pub license_text: String,
    pub used_by: Vec<LicenseUsedBy>,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct LicenseUsedBy {
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub repository: String,
}

#[derive(Debug, Default)]
pub struct About {
    pub license_info_rust: LicenseInfo,
    pub license_info_python: LicenseInfo,
}

impl About {
    pub fn new() -> Self {
        //Rust
        let info_result_rust = serde_json::from_str(THIRD_PARTY_LICENSE_RUST);
        let info_rust: LicenseInfo;
        match info_result_rust {
            Ok(info) => {
                info_rust = info;
            }
            Err(e) => {
                log::error!("Json Parse Error:{}", e);
                process::exit(1);
            }
        }
        //Python
        let info_result_python = serde_json::from_str(THIRD_PARTY_LICENSE_PYTHON);
        let info_python: LicenseInfo;
        match info_result_python {
            Ok(info) => {
                info_python = info;
            }
            Err(e) => {
                log::error!("Json Parse Error:{}", e);
                process::exit(1);
            }
        }
        // State
        return Self {
            license_info_rust: info_rust,
            license_info_python: info_python,
        };
    }

    pub fn update(&mut self, message: AboutMsg) -> iced::task::Task<ToolBoxMsg> {
        match message {
            AboutMsg::OpenLicense => {
                let mut tool_file_name = String::new();
                //
                tool_file_name.push_str("about_show_full_license");
                #[cfg(target_os = "windows")]
                {
                    tool_file_name.push_str(".exe");
                }
                let tool_path = std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(tool_file_name);
                let _ = std::process::Command::new(tool_path).spawn();
            }
        }
        iced::task::Task::none()
    }

    pub fn view(&self) -> Column<'_, ToolBoxMsg> {
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
        let mut layout_license = Column::new().width(Length::Fill).padding(15);
        layout_license = layout_license.push(create_license_info(
            String::from(PROJECT_NAME),
            vec![String::from("TW0hank0")],
            String::from("AGPL-3.0"),
            shared::PROJECT_VERSION,
        ));
        //
        let mut layout_third_party = Column::new().padding(15);
        layout_third_party = layout_third_party
            .push(button("開啟完整內容").on_press(ToolBoxMsg::AboutMsg(AboutMsg::OpenLicense)));
        // Rust
        let third_party_license_infos_rust = self.license_info_rust.dependencies.clone();
        for license_info in third_party_license_infos_rust {
            for used_project in license_info.used_by {
                layout_third_party = layout_third_party.push(create_license_info(
                    used_project.name,
                    vec![String::from("Unknown")],
                    license_info.license_id.clone(),
                    used_project.version,
                ));
            }
        }
        // Python
        let third_party_license_infos_python = self.license_info_python.dependencies.clone();
        for license_info in third_party_license_infos_python {
            for used_project in license_info.used_by {
                layout_third_party = layout_third_party.push(create_license_info(
                    used_project.name,
                    vec![String::from("Unknown")],
                    license_info.license_id.clone(),
                    used_project.version,
                ));
            }
        }
        //layout
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
    version: String,
) -> iced::Element<'_, ToolBoxMsg> {
    let mut layout = Column::new().padding(10);
    layout = layout
        .push(
            text(format!("{} v{}", project_name, version))
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
    let binding2 = license_vec.join("");
    license_vec = binding2.split("/").collect();
    license_vec.sort();
    let mut licenses: Vec<Licenses> = Vec::new();
    for license in license_vec {
        if license.starts_with("AGPL") {
            licenses.push(Licenses::AGPL3);
        } else if license.starts_with("Apache") {
            licenses.push(Licenses::Apache2);
        } else if license.starts_with("MIT") {
            licenses.push(Licenses::MIT);
        } else if license.starts_with("BSD-3-Clause") {
            licenses.push(Licenses::BSD3Clause);
        } else if license.starts_with("BSD-2-Clause") {
            licenses.push(Licenses::BSD2Clause);
        } else {
            licenses.push(Licenses::Other(String::from(license)));
        }
    }
    //
    let mut layout_license = Row::new().padding(5);
    layout_license = layout_license.push(text("license: "));
    let mut licenses_texts = Vec::new();
    for license in licenses {
        licenses_texts.push(format!("{}", license));
    }
    layout_license = layout_license.push(text(licenses_texts.join("、")).size(20));
    layout = layout.push(layout_license);
    //
    return widget::container(layout)
        .style(|theme: &iced::Theme| {
            let ex_palette = theme.extended_palette();
            let mut style = widget::container::Style::default();
            style = style.background(ex_palette.primary.weak.color);
            style
        })
        .into();
}
