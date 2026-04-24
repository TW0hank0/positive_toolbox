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

use iced;
use iced::widget::{Column, button, scrollable, text};

use log;

#[cfg(not(target_arch = "wasm32"))]
use open;

use ptb_shared::shared;
use ptb_shared::shared::{FONT_NOTO_SANS_REG, PROJECT_NAME};

const TOOL_NAME: &str = "about_show_full_license";

const LICENSE_RUST: &str = include_str!("../../../auto_generated/ThirdPartyLicense-Rust.html");
const LICENSE_PYTHON: &str = include_str!("../../../auto_generated/ThirdPartyLicense-Python.html");

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
    OpenRustFile,
    OpenPythonFile,
}

impl About {
    pub fn new() -> Self {
        std::fs::write(
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("ThirdPartyLicense-Rust.html"),
            LICENSE_RUST,
        )
        .ok();
        std::fs::write(
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .join("ThirdPartyLicense-Python.html"),
            LICENSE_PYTHON,
        )
        .ok();
        return Self {};
    }

    pub fn update(&mut self, message: AboutMsg) {
        match message {
            AboutMsg::OpenRustFile => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    open::that_in_background(
                        std::env::current_exe()
                            .unwrap()
                            .parent()
                            .unwrap()
                            .join("ThirdPartyLicense-Rust.html"),
                    );
                }
                #[cfg(target_arch = "wasm32")]
                eprintln!("不支援WASM！");
            }
            AboutMsg::OpenPythonFile => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    open::that_in_background(
                        std::env::current_exe()
                            .unwrap()
                            .parent()
                            .unwrap()
                            .join("ThirdPartyLicense-Python.html"),
                    );
                }
                #[cfg(target_arch = "wasm32")]
                eprintln!("不支援WASM！");
            }
        }
    }

    pub fn view(&self) -> Column<'_, AboutMsg> {
        let mut layout = Column::new()
            .padding(5)
            .align_x(iced::alignment::Horizontal::Left)
            .width(iced::Length::Fill);
        #[cfg(not(target_arch = "wasm32"))]
        {
            layout = layout
                .push(button("開啟ThirdPartyLicense-Rust.html").on_press(AboutMsg::OpenRustFile))
                .push(
                    button("開啟ThirdPartyLicense-Python.html").on_press(AboutMsg::OpenPythonFile),
                );
        }
        layout = layout.push(
            text("ThirdPartyLicense-Rust")
                .size(iced::Pixels::from(35))
                .font(shared::FONT_NOTO_SANS_BOLD),
        );
        let license_text_rust = text(LICENSE_RUST).size(18);
        let scrollable_license_text_rust = scrollable(license_text_rust)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill);
        layout = layout.push(scrollable_license_text_rust).spacing(10);
        //
        layout = layout.push(
            text("ThirdPartyLicense-Python")
                .size(iced::Pixels::from(35))
                .font(shared::FONT_NOTO_SANS_BOLD),
        );
        let license_text_python = text(LICENSE_PYTHON).size(18);
        let scrollable_license_text_python = scrollable(license_text_python)
            .height(iced::Length::Fill)
            .width(iced::Length::Fill);
        layout = layout.push(scrollable_license_text_python).spacing(10);
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
