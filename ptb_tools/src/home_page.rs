use std::{collections::HashMap, env, process};

use iced::{
    self,
    widget::{Column, button, container, scrollable, text},
};

use ptb_shared::{
    self, lang_get,
    shared::{self, HomePageMsg, ToolBoxMsg, Tools},
};

#[derive(Debug)]
pub struct HomePage {
    tools_ordered: HashMap<usize, Tool>,
    language: ptb_shared::languages::base::LangStruct,
    text_size_system: shared::TextSizeControler,
}

#[derive(Debug, Clone)]
pub struct Tool {
    name: &'static str,
    file_name: &'static str,
    msg: Tools,
    describe: Option<&'static str>,
}

impl HomePage {
    pub fn new(
        language: ptb_shared::languages::base::LangStruct,
        text_size_system: shared::TextSizeControler,
    ) -> Self {
        let mut all_tool: Vec<Tool> = Vec::new();
        all_tool.push(Tool {
            name: lang_get!(language, tool_name_code_indenter),
            file_name: "code_indenter",
            msg: Tools::CodeIndenter,
            describe: Some(lang_get!(language, tool_describe_code_indenter)),
        });
        all_tool.push(Tool {
            name: lang_get!(language, tool_name_about),
            file_name: "about",
            msg: Tools::About,
            describe: Some(lang_get!(language, tool_describe_about)),
        });
        all_tool.push(Tool {
            name: lang_get!(language, tool_name_system_info),
            file_name: "system_info",
            msg: Tools::SystemInfo,
            describe: Some(lang_get!(language, tool_describe_system_info)),
        });
        all_tool.push(Tool {
            name: "輕鬆更新",
            file_name: "eazy_updater",
            msg: Tools::EazyUpdater,
            describe: Some("(開發中) 系統更新工具的GUI包裝(wrap)"),
        });
        let mut tools_ordered: HashMap<usize, Tool> = HashMap::new();
        let mut tool_count: usize = 0;
        for tool in all_tool.clone() {
            tools_ordered.insert(tool_count, tool);
            tool_count += 1;
        }
        return Self {
            tools_ordered: tools_ordered,
            language: language,
            text_size_system: text_size_system,
        };
    }

    pub fn update(&mut self, msg: HomePageMsg) {
        match msg {
            HomePageMsg::OpenTool(tool) => match env::current_exe() {
                Ok(exec_path) => {
                    let _ = process::Command::new(exec_path)
                        .arg(format!("{:?}", tool))
                        .spawn();
                }
                Err(e) => {
                    log::error!("HomePage: Fail to get exec_path! detail: {}", e);
                }
            },
        }
    }

    pub fn view(&self) -> iced::widget::Column<'_, ToolBoxMsg> {
        let mut layout = Column::new().padding(5);
        let mut layout_tools = Column::new().spacing(20).padding(40).align_x(iced::Left);
        //
        for count in 0..self.tools_ordered.len() {
            let mut layout_tool = Column::new().spacing(100);
            let tool = self.tools_ordered.get(&count).unwrap();
            let tool_name = text("啟動")
                .size(34)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center);
            layout_tool = layout_tool.push(tool_name).spacing(40);
            let describe_text = text(
                tool.describe
                    .unwrap_or(lang_get!(self.language, main_ui_no_describe)),
            )
            .size(iced::Pixels::from(18));
            layout_tool = layout_tool.push(describe_text);
            let tool_btn = button(layout_tool)
                .on_press(ToolBoxMsg::HomePageMsg(HomePageMsg::OpenTool(
                    tool.msg.clone(),
                )))
                .style(|theme: &iced::Theme, status: button::Status| {
                    let ex_palette = theme.extended_palette();
                    let mut style = button::Style::default();
                    match status {
                        button::Status::Active => {
                            style = style.with_background(iced::Color::TRANSPARENT);
                        }
                        button::Status::Hovered => {
                            style = style.with_background(ex_palette.secondary.weak.color);
                        }
                        button::Status::Pressed => {
                            style = style.with_background(ex_palette.secondary.strong.color);
                        }
                        button::Status::Disabled => {
                            style = style.with_background(ex_palette.background.weaker.color);
                            style.text_color = ex_palette.background.weaker.text;
                        }
                    }
                    style.border =
                        iced::border::rounded(iced::border::radius(iced::Pixels::from(12)))
                            .color(ex_palette.primary.strong.color)
                            .width(iced::Pixels::from(7));
                    style
                });
            layout_tools = layout_tools.push(tool_btn).spacing(30);
        }
        //
        let container_tools = container(layout_tools)
            .style(|theme: &iced::Theme| {
                let ex_palette = theme.extended_palette();
                let mut style = container::background(ex_palette.background.weak.color);
                style = style.border(iced::border::rounded(iced::border::Radius::from(12)));
                return style;
            })
            .width(iced::Length::Fill);
        let scrollable_tools = scrollable(container_tools);
        layout = layout.push(scrollable_tools);
        return layout;
    }
}
