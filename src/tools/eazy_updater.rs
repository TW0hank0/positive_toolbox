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

use positive_toolbox::shared;
use positive_toolbox::shared::FONT_NOTO_SANS_REG;

//const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
const TOOL_NAME: &str = "輕鬆更新";

fn main() -> iced::Result {
    let (icon,) = shared::init();
    log::info!("準備啟動「{}」...", TOOL_NAME);
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
    iced::application(SystemInfo::new, SystemInfo::update, SystemInfo::view)
        .theme(SystemInfo::theme)
        .title(SystemInfo::title)
        .window(window_settings)
        .default_font(FONT_NOTO_SANS_REG)
        .settings(app_settings)
        .run()
}

#[derive(Default)]
pub struct SystemInfo {}

#[derive(Debug, Clone)]
pub enum SystemInfoMsg {}

impl SystemInfo {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn update(&mut self, message: SystemInfoMsg) {}

    pub fn view(&self) -> Column<'_, SystemInfoMsg> {
        let mut layout = Column::new()
            .padding(5)
            .align_x(iced::alignment::Horizontal::Left)
            .width(iced::Length::Fill);
        //
        layout = layout.push(shared::view_title(TOOL_NAME));
        //
        return layout;
    }

    pub fn title(&self) -> String {
        return String::from(format!("{} ——— {}", TOOL_NAME, shared::PROJECT_NAME));
    }

    pub fn theme(&self) -> Option<iced::Theme> {
        Some(iced::Theme::Dark)
    }
}
