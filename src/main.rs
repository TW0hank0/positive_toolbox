use std;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, process};

use iced;
use iced::widget::{Column, button, column, scrollable, text};

//use image;

//use positive_tool_rs::pt;

use log;
//use log::{debug, error, info, trace, warn};

use positive_toolbox::shared;
use positive_toolbox::shared::FONT_NOTO_SANS_REG;

//const FONT_NOTO_SANS_REGULAR_BYTES: &[u8] = include_bytes!("../assets/fonts/Noto_Sans_TC/static/NotoSansTC-Regular.ttf");

//const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");

pub fn main() -> iced::Result {
    let (icon,) = shared::init();
    log::info!("已設定logger。");
    //
    //let project_path = pt::find_project_path(env!("CARGO_PKG_NAME"), None).unwrap();
    //let icon_path = project_path.clone().join("icon.png");
    //let icon_path_str = icon_path.to_str().unwrap();
    //const ICON_PNG: &[u8] = include_bytes!("../icon.png");
    /*
    let img = image::load_from_memory_with_format(ICON_PNG, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (img_width, img_height) = img.dimensions(); */
    let mut window_settings = iced::window::Settings::default();
    window_settings.maximized = true;
    window_settings.icon = icon;
    window_settings.min_size = Some(iced::Size::new(1080.0, 720.0));
    window_settings.position = iced::window::Position::Centered;
    //
    //let _ = iced::font::load(FONT_NOTO_SANS_REGULAR_BYTES);
    let mut app_settings = iced::Settings::default();
    app_settings.id = Some(String::from(env!("CARGO_PKG_NAME")));
    app_settings.default_text_size = iced::Pixels::from(26);
    //app_settings.fonts = vec![FONT_NOTO_SANS_REGULAR_BYTES.into()];
    app_settings.default_font = FONT_NOTO_SANS_REG;
    //
    log::debug!("執行iced...");
    iced::application(Toolbox::new, Toolbox::update, Toolbox::view)
        .theme(Toolbox::theme)
        .title(Toolbox::title)
        //font(FONT_NOTO_SANS_REGULAR_BYTES)
        .window(window_settings)
        .settings(app_settings)
        .default_font(FONT_NOTO_SANS_REG)
        .run()
}

#[derive(Default)]
struct Toolbox {
    // project_path: PathBuf,
    tool_paths: HashMap<String, PathBuf>,
    tools_ordered: HashMap<usize, Tool>,
}

#[derive(Debug, Clone)]
enum ToolboxMsg {
    OpenCodeIndenter,
    OpenSystemInfo,
    OpenAbout,
}

impl std::fmt::Display for ToolboxMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::OpenAbout => "about",
            Self::OpenCodeIndenter => "code_indenter",
            Self::OpenSystemInfo => "system_info",
        })
    }
}

#[derive(Debug, Clone)]
struct Tool {
    name: &'static str,
    file_name: &'static str,
    msg: ToolboxMsg,
}

impl Toolbox {
    pub fn new() -> Self {
        let mut all_tool: Vec<Tool> = Vec::new();
        all_tool.push(Tool {
            name: "程式碼縮排",
            file_name: "code_indenter",
            msg: ToolboxMsg::OpenCodeIndenter,
        });
        all_tool.push(Tool {
            name: "關於",
            file_name: "about",
            msg: ToolboxMsg::OpenAbout,
        });
        all_tool.push(Tool {
            name: "系統資訊 (開發中)",
            file_name: "system_info",
            msg: ToolboxMsg::OpenSystemInfo,
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
            let mut tool_path;
            tool_path = exec_path.clone().join(tool.file_name);
            #[cfg(target_os = "windows")]
            {
                tool_path = PathBuf::from(format!("{}.exe", tool_path.to_str().unwrap()));
            }
            tool_paths.insert(String::from(tool.file_name), tool_path);
        }
        //
        Self {
            tool_paths: tool_paths,
            tools_ordered: tools_ordered,
        }
    }

    pub fn update(&mut self, message: ToolboxMsg) {
        let file_name = format!("{}", message);
        process::Command::new(self.tool_paths.get(&file_name).unwrap().clone())
            .spawn()
            .ok();
    }

    pub fn view(&self) -> iced::widget::Column<'_, ToolboxMsg> {
        let mut layout = column![text("positive toolbox").size(70),].padding(50);
        layout = layout.spacing(30);
        let mut layout_tool = Column::new().spacing(20).padding(30).align_x(iced::Left);
        //
        for count in 0..self.tools_ordered.len() {
            let tool = self.tools_ordered.get(&count).unwrap();
            let tool_name = tool.name;
            let tool_msg = tool.msg.clone();
            let tool_btn = button(
                text(tool_name)
                    .size(30)
                    .align_y(iced::alignment::Vertical::Center)
                    .align_x(iced::alignment::Horizontal::Center),
            )
            .on_press(tool_msg)
            .width(190)
            .height(70);
            layout_tool = layout_tool.push(tool_btn);
        }
        //
        let scrollable_tools = scrollable(layout_tool);
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
