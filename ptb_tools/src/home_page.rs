use std::{collections::HashMap, env, process, sync::mpsc, thread, time};

use iced::{
    self, Length,
    widget::{self, Column, button, container, scrollable, text},
};

use ptb_shared::{
    self, lang_get,
    shared::{self, HomePageMsg, ToolBoxMsg, Tools},
};

#[derive(Debug)]
pub struct HomePage {
    tools_ordered: HashMap<usize, Tool>,
    language: ptb_shared::languages::base::PTBLanguages,
    text_size_system: shared::TextSizeControler,
    threads: Vec<(thread::JoinHandle<()>, mpsc::Receiver<ThreadProcessMsg>)>,
}

#[derive(Debug)]
pub enum ThreadProcessMsg {
    Done,
    UnknownError,
    IoError(std::io::Error),
    ErrorWithStringDetail(String),
}

#[derive(Debug, Clone)]
pub struct Tool {
    name: String,
    msg: Tools,
    describe: Option<String>,
}

impl HomePage {
    pub fn new(
        language: ptb_shared::languages::base::PTBLanguages,
        text_size_system: shared::TextSizeControler,
    ) -> Self {
        let mut all_tool: Vec<Tool> = Vec::new();
        all_tool.push(Tool {
            name: lang_get!(language, tool_info, code_indenter_name),
            msg: Tools::CodeIndenter,
            describe: Some(lang_get!(language, tool_info, code_indenter_describe)),
        });
        all_tool.push(Tool {
            name: lang_get!(language, tool_info, about_name),
            msg: Tools::About,
            describe: Some(lang_get!(language, tool_info, about_describe)),
        });
        all_tool.push(Tool {
            name: lang_get!(language, tool_info, system_info_name),
            msg: Tools::SystemInfo,
            describe: Some(lang_get!(language, tool_info, system_info_describe)),
        });
        all_tool.push(Tool {
            name: String::from("輕鬆更新"),
            msg: Tools::EazyUpdater,
            describe: Some(String::from("(開發中) 系統更新工具的GUI包裝(wrap)")),
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
            threads: Vec::new(),
        };
    }

    pub fn update(&mut self, msg: HomePageMsg) -> iced::Task<ToolBoxMsg> {
        match msg {
            HomePageMsg::OpenTool(tool) => {
                let (tx, rx) = mpsc::channel::<ThreadProcessMsg>();
                let handle = thread::spawn(move || match env::current_exe() {
                    Ok(exec_path) => {
                        match process::Command::new(exec_path)
                            .arg(format!("{:?}", tool))
                            .spawn()
                        {
                            Ok(mut child) => {
                                match child.wait() {
                                    Ok(exit_status) => {
                                        if exit_status.success() {
                                            log::info!("HomePage:child exit success.");
                                            let _result = tx.send(ThreadProcessMsg::Done);
                                        } else {
                                            log::error!("HomePage: child exit with non-zero code!");
                                            let _result = tx.send(ThreadProcessMsg::ErrorWithStringDetail(String::from("HomePage: child exit with non-zero code!")));
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("HomePage: child wait error!");
                                        let _result = tx.send(ThreadProcessMsg::IoError(e));
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("HomePage: Fail to spawn child!");
                                let _result = tx.send(ThreadProcessMsg::IoError(e));
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("HomePage: Fail to get exec_path! detail: {}", e);
                        let _result = tx.send(ThreadProcessMsg::ErrorWithStringDetail(format!(
                            "HomePage: Fail to get exec_path! detail: {}",
                            e
                        )));
                    }
                });
                self.threads.push((handle, rx));
                return iced::task::Task::done(ToolBoxMsg::HomePageMsg(
                    HomePageMsg::CheckThreadInfo,
                ));
            }
            HomePageMsg::CheckThreadInfo => {
                let mut index = 0;
                for (handle, rx) in self.threads.iter() {
                    if handle.is_finished() {
                        match rx.recv_timeout(time::Duration::from_secs(2)) {
                            Ok(thread_msg) => {
                                match thread_msg {
                                    ThreadProcessMsg::Done => {}
                                    ThreadProcessMsg::ErrorWithStringDetail(detail) => {
                                        // TODO: 通知系統
                                        log::error!(
                                            "ThreadProcessMsg::ErrorWithStringDetail: {}",
                                            detail
                                        );
                                    }
                                    ThreadProcessMsg::IoError(err_io) => {
                                        // TODO: 通知系統
                                        log::error!("ThreadProcessMsg::IoError: {}", err_io);
                                    }
                                    ThreadProcessMsg::UnknownError => {
                                        // TODO: 通知系統
                                        log::error!("ThreadProcessMsg::UnknownError!");
                                    }
                                }
                                self.threads.remove(index);
                                break;
                            }
                            Err(e) => {
                                log::error!("Fail to recv: {}", e);
                                index += 1;
                            }
                        }
                    } else {
                        index += 1;
                    }
                }
                if self.threads.len() > 0 {
                    return iced::task::Task::done(ToolBoxMsg::HomePageMsg(
                        HomePageMsg::CheckThreadInfo,
                    ));
                }
            }
        }
        return iced::Task::none();
    }

    pub fn view(&self) -> iced::widget::Column<'_, ToolBoxMsg> {
        let mut layout = Column::new().push(widget::space().height(5));
        layout = layout
            .push(text("選擇一個你要開啟的工具...").size(20))
            .push(widget::space().height(10));
        let mut layout_tools = Column::new().align_x(iced::Left);
        for count in 0..self.tools_ordered.len() {
            let mut layout_tool = Column::new();
            let tool = self.tools_ordered.get(&count).unwrap();
            let tool_name = text(tool.name.clone())
                .size(28)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center);
            layout_tool = layout_tool.push(tool_name);
            layout_tool = layout_tool.push(widget::space().height(10));
            let describe_text = text(tool.describe.clone().unwrap_or(lang_get!(
                self.language,
                home_page,
                tool_no_describe
            )))
            .size(18);
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
                            style = style.with_background(ex_palette.secondary.base.color);
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
                        iced::border::rounded(iced::border::radius(iced::Pixels::from(8)))
                            .color(ex_palette.primary.strong.color)
                            .width(iced::Pixels::from(3));
                    style
                })
                .width(Length::Fill);
            layout_tools = layout_tools.push(tool_btn).push(widget::space().height(18));
        }
        //
        let container_tools = container(layout_tools)
            .style(|theme: &iced::Theme| {
                let ex_palette = theme.extended_palette();
                let mut style =
                    container::Style::default().background(ex_palette.background.weak.color);
                style =
                    style.border(iced::border::rounded(iced::border::Radius::from(12)).width(2));
                return style;
            })
            .width(iced::Length::Fill);
        let scrollable_tools = scrollable(container_tools);
        layout = layout.push(scrollable_tools);
        return layout;
    }
}
