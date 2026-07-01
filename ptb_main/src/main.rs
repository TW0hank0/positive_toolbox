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

use std::{self, collections::HashMap, env, fs, path::PathBuf, process};

use iced::{
    self,
    widget::{Column, Row, button, container, scrollable, text},
};

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
#[command(version, author)]
pub struct ToolBoxCli {
    #[clap(subcommand)]
    pub command: Option<ToolBoxCliCommands>,
}

impl Default for ToolBoxCli {
    fn default() -> Self {
        Self {
            command: Some(ToolBoxCliCommands::HomePage),
        }
    }
}

#[derive(clap::Subcommand)]
pub enum ToolBoxCliCommands {
    /// HomePage 工具
    #[clap(alias = "HomePage")]
    HomePage,
    /// SystemInfo 工具
    #[clap(alias = "SystemInfo")]
    SystemInfo,
    /// CodeIndenter 工具
    #[clap(alias = "CodeIndenter")]
    CodeIndenter,
    /// EazyUpdater 工具
    #[clap(alias = "EazyUpdater")]
    EazyUpdater,
    /// About 工具
    #[clap(alias = "About")]
    About,
}

impl Default for ToolBoxCliCommands {
    fn default() -> Self {
        Self::HomePage
    }
}

#[derive(Debug)]
struct Toolbox {
    current_tool: Tools,
    tool_state: ToolsStates,
    language: ptb_shared::languages::base::PTBLanguages,
    text_size_controler: shared::TextSizeControler,
}

#[derive(Debug)]
pub struct ToolsStates {
    home_page: Option<ptb_tools::home_page::HomePage>,
    code_indenter: Option<ptb_tools::code_indenter::CodeIndenter>,
    system_info: Option<ptb_tools::system_info::SystemInfo>,
    eazy_updater: Option<ptb_tools::eazy_updater::EazyUpdater>,
    about: Option<ptb_tools::about::About>,
}

impl Toolbox {
    pub fn new() -> Self {
        let cli_arg: ToolBoxCli = ToolBoxCli::parse();
        let current_tool: Tools;
        match cli_arg.command {
            Some(t)=> match t {
            ToolBoxCliCommands::HomePage => {
                current_tool = Tools::HomePage;
            }
            ToolBoxCliCommands::CodeIndenter => {
                current_tool = Tools::CodeIndenter;
            }
            ToolBoxCliCommands::SystemInfo => {
                current_tool = Tools::SystemInfo;
            }
            ToolBoxCliCommands::EazyUpdater => {
                current_tool = Tools::EazyUpdater;
            }
            ToolBoxCliCommands::About => {
                current_tool = Tools::About;
            }}
            None => {
                current_tool = Tools::HomePage;
            }
        }
        //
        let settings = Self::load_setting();
        let language = Toolbox::language_system(settings.clone());
        let text_size_system = shared::TextSizeControler::default();
        //
        let mut tb = Self {
            current_tool: current_tool.clone(),
            language: language,
            tool_state: ToolsStates {
                home_page: None,
                code_indenter: None,
                system_info: None,
                eazy_updater: None,
                about: None,
            },
            text_size_controler: text_size_system,
        };
        tb.init_tool_state(current_tool.clone());
        return tb;
        /*Self {
            current_tool: current_tool,
            tool_state: ToolsStates {
                home_page: ptb_tools::home_page::HomePage::new(language, text_size_system),
                code_indenter: ptb_tools::code_indenter::CodeIndenter::new(),
                system_info: ptb_tools::system_info::SystemInfo::new(),
                eazy_updater: ptb_tools::eazy_updater::EazyUpdater::new(),
                about: ptb_tools::about::About::new(),
            },
        }*/
    }

    fn init_tool_state(&mut self, tool: Tools) {
        match tool {
            Tools::About => {
                self.tool_state.about = Some(ptb_tools::about::About::new());
            }
            Tools::CodeIndenter => {
                self.tool_state.code_indenter = Some(ptb_tools::code_indenter::CodeIndenter::new());
            }
            Tools::EazyUpdater => {
                self.tool_state.eazy_updater = Some(ptb_tools::eazy_updater::EazyUpdater::new());
            }
            Tools::HomePage => {
                self.tool_state.home_page = Some(ptb_tools::home_page::HomePage::new(
                    self.language.clone(),
                    self.text_size_controler.clone(),
                ));
            }
            Tools::SystemInfo => {
                self.tool_state.system_info = Some(ptb_tools::system_info::SystemInfo::new());
            }
        }
    }

    pub fn language_system(
        settings: ptb_shared::settings::PTBSettings,
    ) -> ptb_shared::languages::base::PTBLanguages {
        match settings.normal.language {
            ptb_shared::languages::base::SupportedLanguages::Chinese => {
                return ptb_shared::languages::chinese::get_lang();
            }
            ptb_shared::languages::base::SupportedLanguages::English => {
                return ptb_shared::languages::english::get_lang();
            }
        }
    }

    pub fn load_setting() -> ptb_shared::settings::PTBSettings {
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

    pub fn update(&mut self, message: ToolBoxMsg) -> iced::Task<ToolBoxMsg> {
        match message.clone() {
            ToolBoxMsg::HomePageMsg(tool_msg) => {
                let current_state = self.tool_state.home_page.as_mut();
                match current_state {
                    Some(state) => {
                        return state.update(tool_msg);
                    }
                    None => {
                        self.init_tool_state(self.current_tool.clone());
                        return self.update(message);
                    }
                }
            }
            ToolBoxMsg::CodeIndenterMsg(tool_msg) => {
                //self.tool_state.code_indenter.update(tool_msg);
                let current_state = self.tool_state.code_indenter.as_mut();
                match current_state {
                    Some(state) => {
                        return state.update(tool_msg);
                    }
                    None => {
                        self.init_tool_state(self.current_tool.clone());
                        return self.update(message);
                    }
                }
            }
            ToolBoxMsg::AboutMsg(tool_msg) => {
                //self.tool_state.about.update(tool_msg);
                let current_state = self.tool_state.about.as_mut();
                match current_state {
                    Some(state) => {
                        return state.update(tool_msg);
                    }
                    None => {
                        self.init_tool_state(self.current_tool.clone());
                        return self.update(message);
                    }
                }
            }
            ToolBoxMsg::EazyUpdaterMsg(tool_msg) => {
                //self.tool_state.eazy_updater.update(tool_msg);
                let current_state = self.tool_state.eazy_updater.as_mut();
                match current_state {
                    Some(state) => {
                        return state.update(tool_msg);
                    }
                    None => {
                        self.init_tool_state(self.current_tool.clone());
                        return self.update(message);
                    }
                }
            }
            ToolBoxMsg::SystemInfoMsg(tool_msg) => {
                //self.tool_state.system_info.update(tool_msg);
                match self.tool_state.system_info.as_mut() {
                    Some(state) => {
                        return state.update(tool_msg);
                    }
                    None => {
                        self.init_tool_state(self.current_tool.clone());
                        return self.update(message);
                    }
                }
            }
            ToolBoxMsg::InitToolState(tool) => {
                self.init_tool_state(tool);
            }
        }
        return iced::Task::none();
    }

    pub fn view(&self) -> iced::widget::Column<'_, ToolBoxMsg> {
        let mut layout = Column::new().padding(5);
        let mut need_init_state = false;
        match self.current_tool {
            Tools::HomePage => {
                let current_state = self.tool_state.home_page.as_ref();
                match current_state {
                    Some(state) => layout = layout.push(state.view()),
                    None => {
                        need_init_state = true;
                    }
                }
            }
            Tools::CodeIndenter => {
                //layout = layout.push(self.tool_state.code_indenter.view());
                let current_state = self.tool_state.code_indenter.as_ref();
                match current_state {
                    Some(state) => layout = layout.push(state.view()),
                    None => {
                        need_init_state = true;
                    }
                }
            }
            Tools::About => {
                //layout = layout.push(self.tool_state.about.view());
                let current_state = self.tool_state.about.as_ref();
                match current_state {
                    Some(state) => layout = layout.push(state.view()),
                    None => {
                        need_init_state = true;
                    }
                }
            }
            Tools::EazyUpdater => {
                //layout = layout.push(self.tool_state.eazy_updater.view());
                let current_state = self.tool_state.eazy_updater.as_ref();
                match current_state {
                    Some(state) => layout = layout.push(state.view()),
                    None => {
                        need_init_state = true;
                    }
                }
            }
            Tools::SystemInfo => {
                //layout = layout.push(self.tool_state.system_info.view());
                let current_state = self.tool_state.system_info.as_ref();
                match current_state {
                    Some(state) => layout = layout.push(state.view()),
                    None => {
                        need_init_state = true;
                    }
                }
            }
        }
        if need_init_state {
            let mut init_tip_layout = Column::new();
            init_tip_layout = init_tip_layout.push(
                button(text("Init tool state"))
                    .on_press(ToolBoxMsg::InitToolState(self.current_tool.clone())),
            );
            layout = layout.push(init_tip_layout);
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
