use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, process};

use iced;
use iced::widget::{Column, button, column, scrollable, text};

use image;

use positive_tool_rs::pt;

pub fn main() -> iced::Result {
    let project_path = pt::find_project_path(env!("CARGO_PKG_NAME"), None).unwrap();
    let icon_path = project_path.clone().join("icon.png");
    let img = image::open(icon_path).unwrap().into_rgb8();
    let (img_width, img_height) = img.dimensions();
    let mut window_settings = iced::window::Settings::default();
    window_settings.maximized = true;
    window_settings.icon =
        iced::window::icon::from_rgba(img.into_raw(), img_width, img_height).ok();
    //
    iced::application(Toolbox::new, Toolbox::update, Toolbox::view)
        .theme(Toolbox::theme)
        .title(Toolbox::title)
        .font(include_bytes!(
            "../assets/fonts/Noto_Sans_TC/static/NotoSansTC-Regular.ttf"
        ))
        .window(window_settings)
        .run()
}

#[derive(Default)]
struct Toolbox {
    // project_path: PathBuf,
    tool_path_code_indenter: PathBuf,
    tools_ordered: HashMap<usize, Tool>,
}

#[derive(Debug, Clone)]
enum ToolboxMsg {
    OpenCodeIndenter,
}

struct Tool {
    name: &'static str,
    msg: ToolboxMsg,
}

impl Toolbox {
    pub fn new() -> Self {
        //
        let mut tools: HashMap<&str, ToolboxMsg> = HashMap::new();
        tools.insert("單位轉換器", ToolboxMsg::OpenCodeIndenter);
        tools.insert("程式碼縮排", ToolboxMsg::OpenCodeIndenter);
        let mut tools_ordered: HashMap<usize, Tool> = HashMap::new();
        let mut tool_count: usize = 0;
        for tool in tools {
            let (tool_name, tool_msg) = tool;
            tools_ordered.insert(
                tool_count,
                Tool {
                    name: tool_name,
                    msg: tool_msg,
                },
            );
            tool_count += 1;
        }
        //
        //let project_path = pt::find_project_path("positive_toolbox", None).unwrap();
        let project_path = env::current_exe().unwrap().parent().unwrap().to_path_buf();
        let tool_path = project_path.clone(); //.join("tools");
        let tool_path_code_indenter: PathBuf;
        #[cfg(target_os = "windows")]
        {
            tool_path_code_indenter = tool_path.clone().join("code_indenter.exe")
        }
        #[cfg(target_family = "unix")]
        {
            tool_path_code_indenter = tool_path.clone().join("code_indenter")
        }
        Self {
            //project_path: project_path.clone(),
            tool_path_code_indenter: tool_path_code_indenter,
            tools_ordered: tools_ordered,
        }
    }

    pub fn update(&mut self, message: ToolboxMsg) {
        match message {
            ToolboxMsg::OpenCodeIndenter => {
                process::Command::new(self.tool_path_code_indenter.clone())
                    .spawn()
                    .unwrap();
            }
        }
    }

    pub fn view(&self) -> iced::widget::Column<'_, ToolboxMsg> {
        /* let mut tools: HashMap<&str, ToolboxMsg> = HashMap::new();
        tools.insert("單位轉換器", ToolboxMsg::OpenCodeIndenter);
        tools.insert("程式碼縮排", ToolboxMsg::OpenCodeIndenter);
        //
        let mut layout = column![text("positive toolbox").size(60),].padding(20);
        let mut layout_tool = Column::new().spacing(20).padding(20);
        for tool in tools {
            let (tool_name, tool_msg) = tool;
            layout_tool = layout_tool.push(button(tool_name).on_press(tool_msg));
        } */
        let mut layout = column![text("positive toolbox").size(70),].padding(50);
        let mut layout_tool = Column::new().spacing(20).padding(30).align_x(iced::Left);
        //
        for count in 0..self.tools_ordered.len() {
            let tool = self.tools_ordered.get(&count).unwrap();
            let tool_name = tool.name;
            let tool_msg = tool.msg.clone();
            let tool_btn = button(text(tool_name).size(40))
                .on_press(tool_msg)
                .width(200)
                .height(80);
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

/* #[cfg(test)]
mod tests {
    use super::*;
    use iced_test::{Error, simulator};

    #[test]
    fn it_counts() -> Result<(), Error> {
        let mut counter = Counter { value: 0 };
        let mut ui = simulator(counter.view());

        let _ = ui.click("Increment")?;
        let _ = ui.click("Increment")?;
        let _ = ui.click("Decrement")?;

        for message in ui.into_messages() {
            counter.update(message);
        }

        assert_eq!(counter.value, 1);

        let mut ui = simulator(counter.view());
        assert!(ui.find("1").is_ok(), "Counter should display 1!");

        Ok(())
    }
} */
