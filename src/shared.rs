use std::{env, fs};

use time;
use time::{OffsetDateTime, UtcOffset};

use iced;

use image;

use positive_tool_rs::pt;

const ICON_PNG: &[u8] = include_bytes!("../icon.png");
const FONT_NOTO_SANS_REGULAR_BYTES: &[u8] =
    include_bytes!("../assets/fonts/Noto_Sans_TC/static/NotoSansTC-Regular.ttf");

pub const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");

pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
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
    setup_logger().ok();
    //
    return (icon,);
}

pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}
