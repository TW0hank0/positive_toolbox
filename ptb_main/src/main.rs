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

use std;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, process};

use iced;
use iced::widget::{Column, Row, button, container, scrollable, text};

use log;

//use positive_toolbox;
use pmj_shared::shared;
use positive_toolbox::shared::FONT_NOTO_SANS_REG;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_start() -> iced::Result {
    console_error_panic_hook::set_once();
    main()
}

pub fn main() -> iced::Result {
    let (icon,) = shared::init();
    log::info!("已設定logger。");
    //
    let mut window_settings = iced::window::Settings::default();
    window_settings.maximized = true;
    window_settings.icon = icon;
    window_settings.min_size = Some(iced::Size::new(1080.0, 720.0));
    window_settings.position = iced::window::Position::Centered;
    //
    let mut app_settings = iced::Settings::default();
    app_settings.id = Some(String::from(env!("CARGO_PKG_NAME")));
    app_settings.default_text_size = iced::Pixels::from(26);
    app_settings.default_font = FONT_NOTO_SANS_REG;
    //
    log::debug!("執行iced...");
    iced::application(Toolbox::new, Toolbox::update, Toolbox::view)
        .theme(Toolbox::theme)
        .title(Toolbox::title)
        .window(window_settings)
        .settings(app_settings)
        .default_font(FONT_NOTO_SANS_REG)
        .run()
}

#[derive(Default)]
struct Toolbox {
    tool_paths: HashMap<String, PathBuf>,
    tools_ordered: HashMap<usize, Tool>,
    language: positive_toolbox::languages::base_struct::LangStruct,
}

#[derive(Debug, Clone)]
enum ToolboxMsg {
    OpenCodeIndenter,
    OpenSystemInfo,
    OpenAbout,
    OpenEazyUpdater,
}

impl std::fmt::Display for ToolboxMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::OpenAbout => "about",
            Self::OpenCodeIndenter => "code_indenter",
            Self::OpenSystemInfo => "system_info",
            Self::OpenEazyUpdater => "eazy_updater",
        })
    }
}

#[derive(Debug, Clone)]
struct Tool {
    name: &'static str,
    file_name: &'static str,
    msg: ToolboxMsg,
    describe: Option<&'static str>,
}

impl Toolbox {
    pub fn new() -> Self {
        let language = Toolbox::language_system();
        //
        let mut all_tool: Vec<Tool> = Vec::new();
        all_tool.push(Tool {
            name: language.tool_name_code_indenter.unwrap_or("程式碼縮排"),
            file_name: "code_indenter",
            msg: ToolboxMsg::OpenCodeIndenter,
            describe: Some(language.tool_describe_code_indenter.unwrap_or("功能如其名")),
        });
        all_tool.push(Tool {
            name: language.tool_name_about.unwrap_or("關於"),
            file_name: "about",
            msg: ToolboxMsg::OpenAbout,
            describe: Some(
                language
                    .tool_describe_about
                    .unwrap_or("關於 positive_toolbox 及第三方專案"),
            ),
        });
        all_tool.push(Tool {
            name: language.tool_name_system_info.unwrap_or("系統資訊"),
            file_name: "system_info",
            msg: ToolboxMsg::OpenSystemInfo,
            describe: Some(
                language
                    .tool_describe_system_info
                    .unwrap_or("查看系統版本、記憶體等..."),
            ),
        });
        all_tool.push(Tool {
            name: "輕鬆更新",
            file_name: "eazy_updater",
            msg: ToolboxMsg::OpenEazyUpdater,
            describe: Some("(開發中) 系統更新工具的GUI包裝(wrap)"),
        });
        let mut tools_ordered: HashMap<usize, Tool> = HashMap::new();
        let mut tool_count: usize = 0;
        for tool in all_tool.clone() {
            tools_ordered.insert(tool_count, tool);
            tool_count += 1;
        }
        //
        let exec_path = env::current_exe().unwrap().parent().unwrap().to_path_buf();
        let mut tool_paths = HashMap::new();
        for tool in all_tool.clone() {
            let tool_path: PathBuf;
            #[cfg(target_os = "linux")]
            {
                tool_path = exec_path.clone().join(tool.file_name);
            }
            #[cfg(target_os = "windows")]
            {
                tool_path = PathBuf::from(format!(
                    "{}.exe",
                    exec_path.clone().join(tool.file_name).to_str().unwrap()
                ));
            }
            tool_paths.insert(String::from(tool.file_name), tool_path);
        }
        //
        Self {
            tool_paths: tool_paths,
            tools_ordered: tools_ordered,
            language: language,
        }
    }

    pub fn language_system() -> positive_toolbox::languages::base_struct::LangStruct {
        //TODO:等待使用者設定
        positive_toolbox::languages::chinese::get_lang()
    }

    pub fn update(&mut self, message: ToolboxMsg) {
        let file_name = format!("{}", message);
        process::Command::new(self.tool_paths.get(&file_name).unwrap().clone())
            .spawn()
            .ok();
    }

    pub fn view(&self) -> iced::widget::Column<'_, ToolboxMsg> {
        let mut layout = Column::new().padding(30);
        let mut layout_title = Row::new();
        layout_title = layout_title.push(
            iced::widget::image(iced::widget::image::Handle::from_bytes(shared::ICON_PNG))
                .width(70)
                .height(70)
                .filter_method(iced::widget::image::FilterMethod::Linear),
        );
        layout_title = layout_title.push(
            text(shared::PROJECT_NAME)
                .size(iced::Pixels::from(50))
                .font(shared::FONT_NOTO_SANS_BOLD),
        );
        layout = layout.push(layout_title).spacing(40);
        let mut layout_tools = Column::new().spacing(20).padding(40).align_x(iced::Left);
        //
        for count in 0..self.tools_ordered.len() {
            let mut layout_tool = Row::new().spacing(100);
            let tool = self.tools_ordered.get(&count).unwrap();
            let tool_name = tool.name;
            let tool_msg = tool.msg.clone();
            let tool_btn = button(
                text(tool_name)
                    .size(32)
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .on_press(tool_msg)
            .width(180)
            .height(65);
            layout_tool = layout_tool.push(tool_btn).spacing(40);
            let describe_text = text(
                tool.describe
                    .unwrap_or(self.language.main_ui_no_describe.unwrap_or("沒有簡介 @_@")),
            )
            .size(iced::Pixels::from(20));
            layout_tool = layout_tool.push(describe_text);
            let container_tool = container(layout_tool)
                .height(150)
                .width(iced::Length::Fill)
                .style(|_theme| {
                    return container::background(iced::Background::Color(iced::Color::from_rgb8(
                        58, 58, 58,
                    )))
                    .border(iced::border::rounded(iced::border::Radius::from(10)));
                })
                .align_x(iced::alignment::Horizontal::Left)
                .align_y(iced::alignment::Vertical::Center)
                .padding(30);
            layout_tools = layout_tools.push(container_tool).spacing(30);
        }
        //
        let container_tools = container(layout_tools)
            .style(|_theme| {
                return container::background(iced::Background::Color(iced::Color::BLACK))
                    .border(iced::border::rounded(iced::border::Radius::from(10)));
            })
            .width(iced::Length::Fill);
        let scrollable_tools = scrollable(container_tools);
        layout = layout.push(scrollable_tools);
        return layout;
    }

    pub fn title(&self) -> String {
        return String::from("positive_toolbox");
    }

    pub fn theme(&self) -> Option<iced::Theme> {
        Some(iced::Theme::Dark)
    }
}
