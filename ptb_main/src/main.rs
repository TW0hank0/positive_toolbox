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

//! 主程式，使用者界面入口

use std;
use std::collections::HashMap;
use std::{env, fs, path::PathBuf, process};

use iced;
use iced::widget::{Column, Row, button, container, scrollable, text};

use log;

use serde_json;

use clap::{self, Parser};

use ptb_shared::{
    self, lang_get,
    shared::{self, FONT_NOTO_SANS_REG, PROJECT_NAME, ToolBoxMsg, Tools},
};
use ptb_tools;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_start() -> iced::Result {
    console_error_panic_hook::set_once();
    main()
}

pub fn main() -> iced::Result {
    //
    let (icon,) = shared::init();
    log::info!("已設定logger。");
    //
    let mut window_settings = iced::window::Settings::default();
    window_settings.maximized = true;
    window_settings.icon = icon;
    window_settings.min_size = Some(iced::Size::new(1080.0, 720.0));
    window_settings.position = iced::window::Position::Specific(iced::Point::new(10.0, 10.0));
    //
    let mut app_settings = iced::Settings::default();
    app_settings.id = Some(String::from(PROJECT_NAME));
    app_settings.default_text_size = iced::Pixels::from(26);
    app_settings.default_font = FONT_NOTO_SANS_REG;
    //
    /* let cli_arg: ToolBoxCli = ToolBoxCli::parse();
    let current_tool: Tools;
    match cli_arg.command {
        ToolBoxCliCommands::HomePage => {
            current_tool = Tools::HomePage;
        }
    } */
    //
    log::debug!("執行iced...");
    iced::application(Toolbox::new, Toolbox::update, Toolbox::view)
        .theme(Toolbox::theme)
        .title(Toolbox::title)
        .subscription(Toolbox::subscription)
        .window(window_settings)
        .settings(app_settings)
        .default_font(FONT_NOTO_SANS_REG)
        .run()
}

#[derive(clap::Parser)]
pub struct ToolBoxCli {
    #[clap(subcommand)]
    pub command: ToolBoxCliCommands,
}

#[derive(clap::Subcommand)]
pub enum ToolBoxCliCommands {
    HomePage,
}

#[derive(Debug)]
struct Toolbox {
    current_tool: Tools,
    tool_state: ToolsStates,
    language: ptb_shared::languages::base::LangStruct,
    text_size_system: shared::TextSizeControler,
}

#[derive(Debug)]
pub struct ToolsStates {
    home_page: ptb_tools::home_page::HomePage,
    code_indenter: ptb_tools::code_indenter::CodeIndenter,
}

impl Toolbox {
    pub fn new() -> Self {
        let cli_arg: ToolBoxCli = ToolBoxCli::parse();
        let current_tool: Tools;
        match cli_arg.command {
            ToolBoxCliCommands::HomePage => {
                current_tool = Tools::HomePage;
            }
        }
        //
        let language = Toolbox::language_system();
        let text_size_system = shared::TextSizeControler::default();
        //
        Self {
            language: language,
            text_size_system: text_size_system.clone(),
            current_tool: current_tool,
            tool_state: ToolsStates {
                home_page: ptb_tools::home_page::HomePage::new(language, text_size_system),
                code_indenter: ptb_tools::code_indenter::CodeIndenter::new(),
            },
        }
    }

    pub fn language_system() -> ptb_shared::languages::base::LangStruct {
        //TODO:等待製作使用者設定
        ptb_shared::languages::chinese::LANG
    }

    fn load_setting() -> ptb_shared::settings::PTBSettings {
        let file_path = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join(ptb_shared::settings::SETTING_FILE_NAME);
        let setting: ptb_shared::settings::PTBSettings = if fs::exists(&file_path).unwrap_or(false)
        {
            let settings_string = fs::read_to_string(&file_path).unwrap_or(String::new());
            let value: ptb_shared::settings::PTBSettings = serde_json::from_str(&settings_string)
                .unwrap_or(ptb_shared::settings::PTBSettings::default());
            value
        } else {
            ptb_shared::settings::PTBSettings::default()
        };
        setting
    }

    pub fn update(&mut self, message: ToolBoxMsg) {
        //TODO
        match message {
            ToolBoxMsg::HomePageMsg(tool_msg) => {
                self.tool_state.home_page.update(tool_msg);
            }
            ToolBoxMsg::CodeIndenterMsg(tool_msg) => {
                self.tool_state.code_indenter.update(tool_msg);
            }
        }
    }

    pub fn view(&self) -> iced::widget::Column<'_, ToolBoxMsg> {
        let mut layout = Column::new().padding(5);
        match self.current_tool {
            Tools::HomePage => {
                layout = layout.push(self.tool_state.home_page.view());
            }
            Tools::CodeIndenter => {
                layout = layout.push(self.tool_state.code_indenter.view());
            }
            _ => {
                todo!("Not Finish!");
            }
        }
        return layout;
    }

    pub fn title(&self) -> String {
        return format!("{:?} - {}", self.current_tool, shared::PROJECT_NAME);
    }

    pub fn theme(&self) -> Option<iced::Theme> {
        Some(iced::Theme::TokyoNight)
    }

    pub fn subscription(&self) -> iced::Subscription<ToolBoxMsg> {
        match self.current_tool {
            Tools::CodeIndenter => {
                return iced::event::listen_with(|event, _status, _id| match event {
                    iced::Event::Window(wevent) => match wevent {
                        iced::window::Event::Resized(size) => Some(ToolBoxMsg::CodeIndenterMsg(
                            ptb_shared::shared::CodeIndenterMsg::WindowResized {
                                width: size.width as u32,
                                height: size.height as u32,
                            },
                        )),
                        _ => {
                            return None;
                        }
                    },
                    _ => {
                        return None;
                    }
                });
            }
            _ => {
                return iced::Subscription::none();
            }
        }
    }
}
