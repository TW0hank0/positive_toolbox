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

use std::{env, fs};

use time;
use time::{OffsetDateTime, UtcOffset};

use iced;

use image;

use log;

use serde;

#[cfg(not(target_arch = "wasm32"))]
use positive_tool_rs::pt;

#[cfg(target_arch = "wasm32")]
use console_log;

pub const ICON_PNG: &[u8] = include_bytes!("../../assets/icon.png");
const FONT_NOTO_SANS_REGULAR_BYTES: &[u8] =
    include_bytes!("../../assets/fonts/Noto_Sans_TC/static/NotoSansTC-Regular.ttf");

pub const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");
pub const FONT_NOTO_SANS_BOLD: iced::font::Font = iced::font::Font {
    family: iced::font::Family::Name("Noto Sans TC"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const PROJECT_NAME: &str = "positive_toolbox";
pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn init() -> (Option<iced::window::Icon>,) {
    let _ = iced::font::load(FONT_NOTO_SANS_REGULAR_BYTES);
    //
    let img = image::load_from_memory_with_format(ICON_PNG, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (img_width, img_height) = img.dimensions();
    let icon = iced::window::icon::from_rgba(img.into_raw(), img_width, img_height).ok();
    //
    setup_logger();
    //
    return (icon,);
}

#[cfg(target_arch = "wasm32")]
pub fn setup_logger() {
    console_log::init_with_level(log::Level::Trace).ok();
}

#[cfg(not(target_arch = "wasm32"))]
pub fn setup_logger() {
    // 取得本地時區
    let time_offset: UtcOffset =
        UtcOffset::local_offset_at(OffsetDateTime::UNIX_EPOCH).unwrap_or(UtcOffset::UTC);
    // 取得本地時間並格式化（YYYY-MM-DD_HH-MM-SS）
    let time_now = OffsetDateTime::now_utc().to_offset(time_offset);
    let time_now_formatted = time_now
        .format(
            &time::format_description::parse("[year]-[month]-[day]_[hour]-[minute]-[second]")
                .unwrap(),
        )
        .unwrap();
    // 取得可執行檔所在目錄
    let project_path = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    // 建立 logs 目錄（若不存在）
    let log_dir_path = project_path.join(".logs");
    if !log_dir_path.exists() {
        fs::create_dir_all(&log_dir_path).ok();
    }
    // 產生 log 檔案路徑
    let log_file_path = log_dir_path.join(format!("ptb_{}.log", time_now_formatted));
    // 初始化 logger
    pt::build_logger(log_file_path, Some(log::LevelFilter::Info)).ok();
}

/* pub fn view_title<Message, T: Into<String>>(tool_name: T) -> iced::widget::Row<'static, Message> {
    let mut layout_title = iced::widget::Row::new()
        .padding(10)
        .align_y(iced::alignment::Vertical::Bottom)
        .height(90);
    layout_title = layout_title.push(
        iced::widget::image(iced::widget::image::Handle::from_bytes(ICON_PNG))
            .width(60)
            .height(60)
            .filter_method(iced::widget::image::FilterMethod::Linear),
    );
    layout_title = layout_title.push(
        iced::widget::text(tool_name.into())
            .size(40)
            .align_x(iced::alignment::Horizontal::Left)
            .align_y(iced::alignment::Vertical::Bottom)
            .height(90)
            .font(FONT_NOTO_SANS_BOLD),
    );
    layout_title = layout_title.spacing(30);
    layout_title = layout_title.push(
        iced::widget::text(format!("這是{PROJECT_NAME}專案的一部分"))
            .size(20)
            .align_x(iced::alignment::Horizontal::Left)
            .align_y(iced::alignment::Vertical::Bottom)
            .height(90),
    );
    return layout_title;
} */

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct StyleControlSystem {
    pub text: TextSizeControler,
    pub radius: RadiusControler,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RadiusControler {
    normal: u32,
    strong: u32,
}

impl Default for RadiusControler {
    fn default() -> Self {
        Self {
            normal: 12,
            strong: 18,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct TextSizeControler {
    pub normal: u32,
    pub subtitle: u32,
    pub title: u32,
}

impl Default for TextSizeControler {
    fn default() -> Self {
        Self {
            normal: 26,
            subtitle: 35,
            title: 40,
        }
    }
}

impl TextSizeControler {
    pub fn new(normal: u32, subtitle: u32, title: u32) -> Self {
        Self {
            normal,
            subtitle,
            title,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ToolBoxMsg {
    HomePageMsg(HomePageMsg),
    CodeIndenterMsg(CodeIndenterMsg),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tools {
    CodeIndenter,
    SystemInfo,
    About,
    EazyUpdater,
    HomePage,
}

#[derive(Debug, Clone)]
pub enum CodeIndenterMsg {
    OrigCodeChange(iced::widget::text_editor::Action),
    UnitConversion,
    CodeIndenter,
    IndentCodeNow,
    IndentedCodeChange(iced::widget::text_editor::Action),
    LangSelected(ProgramLanguages),
    WindowResized { width: u32, height: u32 },
}

#[derive(Debug, Clone)]
pub enum HomePageMsg {
    OpenTool(Tools),
}

#[derive(Clone, Debug)]
pub enum ProgramLanguages {
    Json,
    Xml,
}

impl ProgramLanguages {
    pub fn all() -> Vec<ProgramLanguages> {
        vec![ProgramLanguages::Json, ProgramLanguages::Xml]
    }
}

impl std::fmt::Display for ProgramLanguages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Json => "json",
            Self::Xml => "xml",
        })
    }
}
