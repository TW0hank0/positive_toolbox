use std::{env, fs};

use time;
use time::{OffsetDateTime, UtcOffset};

use positive_tool_rs::pt;

pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    // 取得本地時區偏移（需啟用 time crate 的 local-offset feature）
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
    // 取得可執行檔所在目錄（替代方案：可考慮使用 std::env::current_dir()）
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
