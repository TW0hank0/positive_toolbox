use std::collections::HashMap;
use std::path::PathBuf;

use iced;
use iced::widget::{Column, button, column, scrollable, text};

use positive_tool_rs::pt;

pub fn main() -> iced::Result {
    iced::application(Toolbox::new, Toolbox::update, Toolbox::view)
        .theme(Toolbox::theme)
        .title(Toolbox::title)
        .run()
}

#[derive(Default)]
struct Toolbox {project_path: PathBuf, tool_path_code_indenter: PathBuf};

#[derive(Debug, Clone)]
enum ToolboxMsg {
    OpenCodeIndenter,
}

impl Toolbox {
    fn new() -> Self {
        let project_path = pt::find_project_path("positive_toolbox", None).unwrap();
        let tool_path = project_path.clone().join("tools")
        let tool_path_code_indenter: PathBuf;
        #[cfg(target_os = "Windows")]
        {
            tool_path_code_indenter = tool_path.clone().join("code_indenter.exe")
        }
        #[cfg(target_family = "unix")]
        {
            tool_path_code_indenter = tool_path.clone().join("code_indenter")
        }
        Self {
            project_path: project_path.clone()
            tool_path_code_indenter: tool_path_code_indenter
        }
    }
    fn update(&mut self, message: ToolboxMsg){
        match message {
            ToolboxMsg::OpenCodeIndenter => {}
            _ => iced::task::Task::none(),
        }
    }

    fn view(&self) -> Column<'_, ToolboxMsg> {
        let mut tools: HashMap<&str, ToolboxMsg> = HashMap::new();
        tools.insert("單位轉換器", ToolboxMsg::OpenCodeIndenter);
        tools.insert("程式碼縮排", ToolboxMsg::OpenCodeIndenter);
        //
        let mut layout = column![text("positive toolbox").size(60),].padding(20);
        let mut layout_tool = Column::new().spacing(20).padding(20);
        for tool in tools {
            let (tool_name, tool_msg) = tool;
            layout_tool = layout_tool.push(button(tool_name).on_press(tool_msg));
        }
        let scrollable_tools = scrollable(layout_tool);
        layout = layout.push(scrollable_tools);
        return layout;
        /* column![
            button("加").on_press(ToolboxMsg::Increment),
            text(self.value).size(50),
            button("減").on_press(ToolboxMsg::Decrement)
        ]
        .padding(20)
        .align_x(Center) */
    }

    fn title(&self) -> String {
        return String::from("positive_toolbox");
    }

    fn theme(&self) -> Option<iced::Theme> {
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
