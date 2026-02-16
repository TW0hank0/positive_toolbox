//use std;

use iced;
use iced::widget::{Column, button, scrollable, text};

//use image;

use log;

use sysinfo;

use positive_toolbox;
use positive_toolbox::shared;
use positive_toolbox::shared::FONT_NOTO_SANS_REG;

const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
const TOOL_NAME: &str = "系統資訊";

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
    iced::application(SystemInfo::new, SystemInfo::update, SystemInfo::view)
        .theme(SystemInfo::theme)
        .title(SystemInfo::title)
        .window(window_settings)
        .default_font(FONT_NOTO_SANS_REG)
        .settings(app_settings)
        .run()
}

#[derive(Default)]
pub struct SystemInfo {
    system_info_data: SystemInfoData,
}

#[derive(Default)]
pub struct SystemInfoData {
    system: SysInfoDataSystem,
    memory: SysInfoDataMemory,
    //disk: SysInfoDataDisk,
    //network: SysInfoDataNetwork,
}

#[derive(Default)]
pub struct SysInfoDataSystem {
    name: String,
    kernel_version: String,
    os_version: String,
    host_name: String,
    cpus_count: usize,
}

#[derive(Default)]
pub struct SysInfoDataMemory {
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
}

/* #[derive(Default, Clone)]
pub struct SysInfoDataDisk {
    disks: sysinfo::Disks,
} */

/* #[derive(Default)]
pub struct SysInfoDataNetwork {
    networks: sysinfo::Networks,
} */

#[derive(Debug, Clone)]
pub enum SystemInfoMsg {
    SyncSysInfo,
}

impl SystemInfo {
    pub fn new() -> Self {
        let system_info_data = SystemInfo::sync_sys_info();
        return Self {
            system_info_data: system_info_data,
        };
    }

    pub fn update(&mut self, message: SystemInfoMsg) {
        match message {
            SystemInfoMsg::SyncSysInfo => {
                self.system_info_data = SystemInfo::sync_sys_info();
            }
        }
    }

    pub fn sync_sys_info() -> SystemInfoData {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        //
        let system_name = format!(
            "{}",
            sysinfo::System::name().unwrap_or(String::from("無法取得"))
        );
        let system_kernel_version = format!(
            "{}",
            sysinfo::System::kernel_version().unwrap_or(String::from("無法取得"))
        );
        let system_os_version = format!(
            "{}",
            sysinfo::System::os_version().unwrap_or(String::from("無法取得"))
        );
        let system_host_name = format!(
            "{}",
            sysinfo::System::host_name().unwrap_or(String::from("無法取得"))
        );
        let system_cpus_count = sys.cpus().len();
        let data_system = SysInfoDataSystem {
            name: system_name,
            kernel_version: system_kernel_version,
            os_version: system_os_version,
            host_name: system_host_name,
            cpus_count: system_cpus_count,
        };
        //
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let total_swap = sys.total_swap();
        let used_swap = sys.used_swap();
        let data_memory = SysInfoDataMemory {
            total_memory: total_memory,
            used_memory: used_memory,
            total_swap: total_swap,
            used_swap: used_swap,
        };
        //
        //let disks = sysinfo::Disks::new_with_refreshed_list();
        //let data_disk = SysInfoDataDisk { disks: disks };
        //
        //let networks = sysinfo::Networks::new_with_refreshed_list();
        //let data_network = SysInfoDataNetwork { networks: networks };
        //
        /* let processes = sys.processes();
        let data_process = SysInfoDataProcess {
            processes: processes,
        }; */
        //
        let system_info_data = SystemInfoData {
            system: data_system,
            memory: data_memory,
            //disk: data_disk,
            //network: data_network,
        };
        return system_info_data;
    }

    pub fn view(&self) -> Column<'_, SystemInfoMsg> {
        let mut layout = Column::new()
            .padding(5)
            .align_x(iced::alignment::Horizontal::Left)
            .width(iced::Length::Fill);
        //
        layout = layout.push(shared::view_title(String::from(TOOL_NAME)));
        //
        layout = layout.push(button(text("重新整理")).on_press(SystemInfoMsg::SyncSysInfo));
        //
        let mut layout_system_info = Column::new();
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        //
        layout_system_info = layout_system_info.push(
            text("系統")
                .font(shared::FONT_NOTO_SANS_BOLD)
                .size(iced::Pixels::from(40)),
        );
        layout_system_info = layout_system_info.push(text(format!(
            "系統類型：{:?}",
            self.system_info_data.system.name
        )));
        layout_system_info = layout_system_info.push(text(format!(
            "系統內核版本：{:?}",
            self.system_info_data.system.kernel_version
        )));
        layout_system_info = layout_system_info.push(text(format!(
            "系統版本：{:?}",
            self.system_info_data.system.os_version
        )));
        layout_system_info = layout_system_info.push(text(format!(
            "電腦名：{:?}",
            self.system_info_data.system.host_name
        )));
        layout_system_info = layout_system_info.push(text(format!(
            "CPU邏輯處理器數量： {}",
            self.system_info_data.system.cpus_count
        )));
        //
        layout_system_info = layout_system_info.push(
            text("記憶體")
                .font(shared::FONT_NOTO_SANS_BOLD)
                .size(iced::Pixels::from(40)),
        );
        //
        let total_memory = self.system_info_data.memory.total_memory;
        let (total_memory_coned, total_memory_unit) = bytes_conversioner(total_memory.clone());
        layout_system_info = layout_system_info.push(text(format!(
            "記憶體：{}{} ({}bytes)",
            total_memory_coned, total_memory_unit, total_memory
        )));
        layout_system_info = layout_system_info.push(text(format!(
            "已使用的記憶體：{} bytes",
            self.system_info_data.memory.used_memory
        )));
        layout_system_info = layout_system_info.push(text(format!(
            "交換式記憶體：{} bytes",
            self.system_info_data.memory.total_swap
        )));
        layout_system_info = layout_system_info.push(text(format!(
            "已使用的交換式記憶體 {}bytes",
            self.system_info_data.memory.used_swap
        )));
        //
        layout_system_info = layout_system_info.push(
            text("硬碟")
                .font(shared::FONT_NOTO_SANS_BOLD)
                .size(iced::Pixels::from(40)),
        );
        let disks = sysinfo::Disks::new_with_refreshed_list();
        //layout_system_info = layout_system_info.push(text("所有硬碟"));
        for disk in &disks {
            layout_system_info = layout_system_info.push(
                text(format!("{}", disk.name().to_str().unwrap_or("未知")))
                    .size(iced::Pixels::from(32)),
            );
            layout_system_info =
                layout_system_info.push(text(format!("總空間：{}", disk.total_space())));
            layout_system_info =
                layout_system_info.push(text(format!("可用空間：{}", disk.available_space())));
            layout_system_info =
                layout_system_info.push(text(format!("硬碟類型：{}", disk.kind())));
            layout_system_info = layout_system_info.push(text(format!(
                "檔案系統類型：{}",
                disk.file_system().to_str().unwrap_or("未知")
            )));
            layout_system_info = layout_system_info.push(text(format!(
                "位子：{}",
                disk.mount_point().to_str().unwrap_or("未知")
            )));
            layout_system_info =
                layout_system_info.push(text(format!("唯讀：{}", disk.is_read_only())));
            layout_system_info =
                layout_system_info.push(text(format!("可移除：{}", disk.is_removable())));
            layout_system_info =
                layout_system_info.push(text(format!("使用狀態：{:?}", disk.usage())));
        }
        layout_system_info = layout_system_info.push(
            text("網路")
                .font(shared::FONT_NOTO_SANS_BOLD)
                .size(iced::Pixels::from(40)),
        );
        let networks = sysinfo::Networks::new_with_refreshed_list();
        for (interface_name, data) in &networks {
            layout_system_info = layout_system_info.push(text(format!(
                "{interface_name} => 上傳：{} Bytes  / {} Bytes 下載",
                data.total_received(),
                data.total_transmitted(),
            )));
        }
        layout_system_info = layout_system_info.push(
            text("程式")
                .font(shared::FONT_NOTO_SANS_BOLD)
                .size(iced::Pixels::from(40)),
        );
        let mut layout_system_info_process = Column::new().padding(10);
        for (pid, process) in sys.processes() {
            layout_system_info_process = layout_system_info_process.push(text(format!(
                "[{pid}] {:?} {:?}",
                process.name(),
                process.disk_usage()
            )));
        }
        let scrollable_system_info_process = scrollable(layout_system_info_process);
        layout_system_info = layout_system_info.push(scrollable_system_info_process);
        //
        let scrollable_system_info = scrollable(layout_system_info);
        layout = layout.push(scrollable_system_info);
        return layout;
    }

    pub fn title(&self) -> String {
        return String::from(format!("{} — {}", TOOL_NAME, PROJECT_NAME));
    }

    pub fn theme(&self) -> Option<iced::Theme> {
        Some(iced::Theme::Dark)
    }
}

pub enum Units {
    Bytes,
    KIB,
    MIB,
    GIB,
}

impl std::fmt::Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Bytes => "Bytes",
            Self::KIB => "KIB",
            Self::MIB => "MIB",
            Self::GIB => "GIB",
        })
    }
}

pub fn bytes_conversioner(value: u64) -> (f64, Units) {
    if value < 1024 {
        return (value as f64, Units::Bytes);
    } else if value >= 1024 && value < (1024 * 1024) {
        return (((value as f64) / 1024.0), Units::KIB);
    } else if value >= (1024 * 1024) && value < (1024 * 1024 * 1024) {
        return ((value as f64) / (1024.0 * 1024.0), Units::MIB);
    } else {
        return (((value as f64) / (1024.0 * 1024.0 * 1024.0)), Units::GIB);
    }
}
