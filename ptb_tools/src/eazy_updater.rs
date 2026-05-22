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

use std::{io, process, sync, thread};

use iced::widget::{Column, button, scrollable, text};

use log;

use ptb_shared::{lang_get, shared};

const TOOL_NAME: &str = "輕鬆更新";

/*fn main() -> iced::Result {
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
    iced::application(EazyUpdater::new, EazyUpdater::update, EazyUpdater::view)
        .theme(EazyUpdater::theme)
        .title(EazyUpdater::title)
        .window(window_settings)
        .default_font(FONT_NOTO_SANS_REG)
        .settings(app_settings)
        .run()
}*/

#[derive(Debug)]
pub struct ThreadProcessStatus {
    pub is_finish: bool,
    pub is_error: bool,
    pub err_msg: Option<io::Error>,
}

pub enum ThreadProcessTypes {
    NoReturn,
}

impl ThreadProcessStatus {
    pub fn update(&mut self, other: Self) {
        self.err_msg = other.err_msg;
        self.is_error = other.is_error;
        self.is_finish = other.is_finish;
    }
}

#[derive(Debug)]
pub struct EazyUpdater {
    pub current_scene: Scenes,
    pub thread_msgs: Vec<sync::Arc<sync::RwLock<ThreadProcessStatus>>>,
    pub process_threads: Vec<thread::JoinHandle<ThreadProcessTypes>>,
}

#[derive(Debug, Clone, Copy)]
pub enum ToolMsg {
    UpdateInstalledPkgList,
    UpdateThreadProcess,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Scenes {
    Installled,
}

impl Default for Scenes {
    fn default() -> Self {
        Self::Installled
    }
}

pub struct PkgInfo {
    name: String,
    version: String,
}

impl EazyUpdater {
    pub fn new() -> Self {
        let mut eu = Self {
            current_scene: Scenes::Installled,
            thread_msgs: Vec::new(),
            process_threads: Vec::new(),
        };
        eu.update(ToolMsg::UpdateInstalledPkgList);
        return eu;
    }

    pub fn update(&mut self, message: ToolMsg) {
        match message {
            ToolMsg::UpdateInstalledPkgList => {
                let msg_channel = sync::Arc::new(sync::RwLock::new(ThreadProcessStatus {
                    is_error: false,
                    is_finish: false,
                    err_msg: None,
                }));
                let thread_msg_channel = msg_channel.clone();
                let process_thread = thread::spawn(move || {
                    let mut dnf_process = process::Command::new("dnf")
                        .args(["list", "--installed", "--color", "never", "--quiet"])
                        .spawn()
                        .unwrap();
                    let process_result = dnf_process.wait();
                    match process_result {
                        Ok(exit_status) => {
                            if !exit_status.success() {
                                let thread_status = ThreadProcessStatus {
                                    is_error: true,
                                    is_finish: true,
                                    err_msg: Some(io::Error::new(
                                        io::ErrorKind::Other,
                                        "非零返回碼",
                                    )),
                                };
                                let mut guard = thread_msg_channel.write().unwrap();
                                guard.update(thread_status);
                            }
                        }
                        Err(e) => {
                            let thread_status = ThreadProcessStatus {
                                is_error: true,
                                is_finish: true,
                                err_msg: Some(e),
                            };
                            let mut guard = msg_channel.write().unwrap();
                            guard.update(thread_status);
                        }
                    }
                    return ThreadProcessTypes::NoReturn;
                });
                self.process_threads.push(process_thread);
                todo!("!!!")
            }
            ToolMsg::UpdateThreadProcess => {
                todo!("ToolMsg::UpdateThreadProcess")
            }
        }
    }

    pub fn view(&self) -> Column<'_, ToolMsg> {
        let mut layout = Column::new()
            .padding(5)
            .align_x(iced::alignment::Horizontal::Left)
            .width(iced::Length::Fill);
        //
        match self.current_scene {
            Scenes::Installled => {}
        }
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
