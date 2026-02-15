use iced;
use iced::widget::{Column, Row, button, combo_box, scrollable, text, text_editor};

use serde_json;

use log;

use positive_toolbox::shared;
use positive_toolbox::shared::FONT_NOTO_SANS_REG;

const TOOL_NAME: &str = "unit_conversion";

fn main() -> iced::Result {
    log::info!("正在啟動 {} - {}", TOOL_NAME, shared::PROJECT_NAME);
    let (icon,) = shared::init();
    let mut window_settings = iced::window::Settings::default();
    window_settings.maximized = true;
    window_settings.icon = icon;
    window_settings.min_size = Some(iced::Size::new(540.0, 360.0));
    //
    let mut app_settings = iced::Settings::default();
    app_settings.id = Some(format!("{} - {}", TOOL_NAME, shared::PROJECT_NAME));
    app_settings.default_text_size = iced::Pixels::from(26);
    app_settings.default_font = FONT_NOTO_SANS_REG;
    //
    log::info!("啟動iced");
    iced::application(CodeIndenter::new, CodeIndenter::update, CodeIndenter::view)
        .theme(CodeIndenter::theme)
        .title(CodeIndenter::title)
        //.font(FONT_NOTO_SANS_REGULAR_BYTES)
        .window(window_settings)
        .default_font(FONT_NOTO_SANS_REG)
        .settings(app_settings)
        .run()
}

#[derive(Default)]
pub struct CodeIndenter {
    orig_code_text_editor_content: iced::widget::text_editor::Content,
    indented_code: String,
    indented_code_text_editor_content: iced::widget::text_editor::Content,
    selected_program_lang: Option<ProgramLanguages>,
    combo_box_langs: combo_box::State<ProgramLanguages>,
    window_width: u32,
    window_height: u32,
}

#[derive(Debug, Clone)]
pub enum CodeIndenterMsg {
    OrigCodeChange(iced::widget::text_editor::Action),
    UnitConversion,
    CodeIndenter,
    IndentCodeNow,
    IndentedCodeChange(iced::widget::text_editor::Action),
    LangSelected(ProgramLanguages),
    WindowResized { width: u32, height: u32 },
}

impl CodeIndenter {
    pub fn new() -> Self {
        let combo_box_langs = combo_box::State::with_selection(ProgramLanguages::all(), None);
        return Self {
            selected_program_lang: None,
            combo_box_langs: combo_box_langs,
            orig_code_text_editor_content: iced::widget::text_editor::Content::new(),
            indented_code: String::new(),
            indented_code_text_editor_content: iced::widget::text_editor::Content::new(),
            window_width: 1080,
            window_height: 720,
        };
    }

    pub fn update(&mut self, message: CodeIndenterMsg) {
        match message {
            CodeIndenterMsg::UnitConversion => {
                println!("UintConversion");
            }
            CodeIndenterMsg::CodeIndenter => {
                println!("CodeIndenter")
            }
            CodeIndenterMsg::OrigCodeChange(act) => {
                self.orig_code_text_editor_content.perform(act.clone());
                match act {
                    iced::widget::text_editor::Action::Edit(_) => {
                        if self.selected_program_lang.is_some() {
                            let code_result: bool;
                            let indented_code: String;
                            (code_result, indented_code) = code_indenter(
                                self.orig_code_text_editor_content.text(),
                                self.selected_program_lang.clone().unwrap(),
                            );
                            if code_result {
                                self.indented_code = indented_code;
                                println!("IndentCode -> indented_code: {}", &self.indented_code);
                                self.indented_code_text_editor_content =
                                    iced::widget::text_editor::Content::with_text(
                                        &self.indented_code,
                                    );
                            }
                        }
                    }
                    _ => {}
                }
            }
            CodeIndenterMsg::IndentCodeNow => {
                println!(
                    "IndentCode -> orig_code: {}",
                    &self.orig_code_text_editor_content.text()
                );
                if self.orig_code_text_editor_content.text().trim() == "" {
                    self.indented_code = String::new();
                } else {
                    if self.selected_program_lang.is_some() {
                        let _code_result: bool;
                        (_code_result, self.indented_code) = code_indenter(
                            self.orig_code_text_editor_content.text(),
                            self.selected_program_lang.clone().unwrap(),
                        );
                        println!("IndentCode -> indented_code: {}", &self.indented_code);
                    } else {
                        self.indented_code = String::from("未選擇語言！");
                    }
                }
                self.indented_code_text_editor_content =
                    iced::widget::text_editor::Content::with_text(&self.indented_code);
            }
            CodeIndenterMsg::IndentedCodeChange(act) => {
                match act {
                    text_editor::Action::Edit(_) => {}
                    _ => {
                        self.indented_code_text_editor_content.perform(act);
                    }
                };
            }
            CodeIndenterMsg::LangSelected(lang) => self.selected_program_lang = Some(lang),
            CodeIndenterMsg::WindowResized { width, height } => {
                self.window_width = width;
                self.window_height = height;
            }
        }
    }

    pub fn view(&self) -> Column<'_, CodeIndenterMsg> {
        let mut layout = Column::new()
            .padding(5)
            .align_x(iced::alignment::Horizontal::Left)
            .width(iced::Length::Fill);
        let layout_title = shared::view_title(String::from(TOOL_NAME));
        layout = layout.push(layout_title);
        layout = layout.spacing(60);
        //
        let mut layout_lang = Row::new().padding(5);
        layout_lang = layout_lang.push(text("選擇一種語言：").size(28));
        layout_lang = layout_lang.push(
            combo_box(
                &self.combo_box_langs,
                "選擇一種語言",
                self.selected_program_lang.as_ref(),
                CodeIndenterMsg::LangSelected,
            )
            .width(180)
            .line_height(iced::widget::text::LineHeight::Absolute(
                iced::Pixels::from(30),
            ))
            .size(iced::Pixels::from(24))
            .padding(10),
        );
        layout = layout.push(layout_lang);
        //
        layout = layout.spacing(10);
        //
        let mut layout_input_tip = Row::new()
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink);
        layout_input_tip = layout_input_tip.push(
            text("請輸入程式碼")
                .size(28)
                .width(iced::Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Bottom),
        );
        layout_input_tip = layout_input_tip.push(
            text("縮排的程式碼")
                .size(28)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Bottom)
                .width(iced::Length::Fill),
        );
        layout = layout.push(layout_input_tip);
        //
        let mut layout_code_blocks = Row::new();
        layout_code_blocks = layout_code_blocks.push(
            text_editor(&self.orig_code_text_editor_content)
                .on_action(CodeIndenterMsg::OrigCodeChange)
                .placeholder("code here...")
                .size(26),
        );
        layout_code_blocks = layout_code_blocks.spacing(30);
        layout_code_blocks = layout_code_blocks.push(
            text_editor(&self.indented_code_text_editor_content)
                .on_action(CodeIndenterMsg::IndentedCodeChange)
                .placeholder("縮排後的程式碼輸出...")
                .size(26),
        );
        let scrollable_code_blocks = scrollable(layout_code_blocks).height(iced::Length::from(
            (self.window_height / 2) + (self.window_height / 7),
        ));
        layout = layout.push(scrollable_code_blocks).spacing(10);
        let submit_btn = button(
            text("縮排")
                .size(24)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center),
        )
        .on_press(CodeIndenterMsg::IndentCodeNow)
        .width(150)
        .height(50);
        layout = layout.push(submit_btn).spacing(10);
        return layout;
    }

    pub fn title(&self) -> String {
        return String::from(format!("{} — {}", TOOL_NAME, shared::PROJECT_NAME));
    }

    pub fn theme(&self) -> Option<iced::Theme> {
        Some(iced::Theme::Dark)
    }

    pub fn subscription(&self) -> iced::Subscription<CodeIndenterMsg> {
        return iced::event::listen_with(|event, _status, _id| match event {
            iced::Event::Window(wevent) => match wevent {
                iced::window::Event::Resized(size) => Some(CodeIndenterMsg::WindowResized {
                    width: size.width as u32,
                    height: size.height as u32,
                }),
                _ => {
                    return None;
                }
            },
            _ => {
                return None;
            }
        });
    }
}

#[derive(Clone, Debug)]
pub enum UnitTypes {
    Long,
}

#[derive(Debug, Clone)]
pub enum TypeLong {
    Centimeter,
}

impl UnitTypes {
    pub fn all() -> Vec<UnitTypes> {
        vec![UnitTypes::Long]
    }
}

impl std::fmt::Display for UnitTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Long => "距離",
        })
    }
}

impl std::fmt::Display for TypeLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Centimeter => "公分",
        })
    }
}

/* fn code_indenter(orig_code: String, lang: ProgramLanguages) -> (bool, String) {
    match lang {
        ProgramLanguages::Json => {
            let result_i: serde_json::Result<serde_json::Value> = serde_json::from_str(&orig_code);
            match result_i {
                Ok(i) => {
                    match serde_json::to_string_pretty(&i) {
                        Ok(i2) => {
                            return (true, i2);
                        }
                        Err(e) => {
                            return (false, format!("錯誤！錯誤訊息：{}", e));
                        }
                    };
                }
                Err(e) => {
                    return (false, format!("錯誤！錯誤訊息：{}", e));
                }
            }
        }
        ProgramLanguages::Xml => {
            let result_i: Result<serde_json::Value, quick_xml::DeError> =
                quick_xml::de::from_str(&orig_code);
            match result_i {
                Ok(i) => {
                    match quick_xml::se::to_string_with_root("root", &i) {
                        Ok(i2) => {
                            return (true, i2);
                        }
                        Err(e) => {
                            return (false, format!("錯誤！錯誤訊息：{}", e));
                        }
                    };
                }
                Err(e) => {
                    return (false, format!("錯誤！錯誤訊息：{}", e));
                }
            }
        }
    }
} */
