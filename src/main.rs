use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, process};

use iced;
use iced::widget::{Column, button, column, scrollable, text};

use image;

const FONT_NOTO_SANS_REGULAR_BYTES: &[u8] =
    include_bytes!("../assets/fonts/Noto_Sans_TC/static/NotoSansTC-Regular.ttf");

const FONT_NOTO_SANS_REG: iced::font::Font = iced::font::Font::with_name("Noto Sans TC");

pub fn main() -> iced::Result {
    //let project_path = pt::find_project_path(env!("CARGO_PKG_NAME"), None).unwrap();
    //let icon_path = project_path.clone().join("icon.png");
    const ICON_PNG: &[u8] = include_bytes!("../icon.png");
    let img = image::load_from_memory_with_format(ICON_PNG, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (img_width, img_height) = img.dimensions();
    let mut window_settings = iced::window::Settings::default();
    window_settings.maximized = true;
    window_settings.icon =
        iced::window::icon::from_rgba(img.into_raw(), img_width, img_height).ok();
    //
    let _ = iced::font::load(FONT_NOTO_SANS_REGULAR_BYTES);
    let mut app_settings = iced::Settings::default();
    app_settings.id = Some(String::from(env!("CARGO_PKG_NAME")));
    app_settings.default_text_size = iced::Pixels::from(26);
    app_settings.fonts = vec![FONT_NOTO_SANS_REGULAR_BYTES.into()];
    app_settings.default_font = FONT_NOTO_SANS_REG;
    //
    iced::application(Toolbox::new, Toolbox::update, Toolbox::view)
        .theme(Toolbox::theme)
        .title(Toolbox::title)
        .font(FONT_NOTO_SANS_REGULAR_BYTES)
        .window(window_settings)
        .settings(app_settings)
        .default_font(FONT_NOTO_SANS_REG)
        .run()
}

#[derive(Default)]
struct Toolbox {
    // project_path: PathBuf,
    tool_path_code_indenter: PathBuf,
    tool_path_unit_conversion: PathBuf,
    tools_ordered: HashMap<usize, Tool>,
}

#[derive(Debug, Clone)]
enum ToolboxMsg {
    OpenCodeIndenter,
    OpenUnitConversion,
}

struct Tool {
    name: &'static str,
    msg: ToolboxMsg,
}

impl Toolbox {
    pub fn new() -> Self {
        //
        let mut tools: HashMap<&str, ToolboxMsg> = HashMap::new();
        tools.insert("單位轉換器(not-finish)", ToolboxMsg::OpenUnitConversion);
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
        let tool_path_unit_conversion: PathBuf;
        #[cfg(target_os = "windows")]
        {
            tool_path_unit_conversion = tool_path.clone().join("unit_conversion.exe")
        }
        #[cfg(target_family = "unix")]
        {
            tool_path_unit_conversion = tool_path.clone().join("unit_conversion")
        }
        Self {
            //project_path: project_path.clone(),
            tool_path_code_indenter: tool_path_code_indenter,
            tool_path_unit_conversion: tool_path_unit_conversion,
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
            ToolboxMsg::OpenUnitConversion => {
                process::Command::new(self.tool_path_unit_conversion.clone())
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
        //let container_tool = container(scrollable_tools).align_left(20).;
        // FIXME: Need Fix
        /* let mut container_tool_palette = iced::theme::Palette::DARK;
        container_tool_palette.background = iced::Color::BLACK;
        container_tool.class(iced::theme::Theme::custom(
            "ptb_main_container_tool",
            container_tool_palette,
        ));
        container_tool. */
        //layout = layout.push(container_tool);
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
