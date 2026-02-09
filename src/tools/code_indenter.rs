//use std::collections::HashMap;

//use serde;
use iced;
use iced::widget::{Column, button, rich_text, row, text, text_editor};
use serde_json;

#[derive(Default)]
pub struct CodeIndenter {
    orig_code: String,
    indented_code: String,
    indented_code_text_editor_content: iced::widget::text_editor::Content,
}

fn main() -> iced::Result {
    iced::application(CodeIndenter::new, CodeIndenter::update, CodeIndenter::view)
        .theme(CodeIndenter::theme)
        .title(CodeIndenter::title)
        .font(include_bytes!(
            "../../assets/fonts/Noto_Sans_TC/static/NotoSansTC-Regular.ttf"
        ))
        .run()
}

#[derive(Debug, Clone)]
pub enum CodeIndenterMsg {
    OrigCodeChange(String),
    UnitConversion,
    CodeIndenter,
    IndentCodeNow,
}

impl CodeIndenter {
    pub fn new() -> Self {
        return Self {
            orig_code: String::new(),
            indented_code: String::new(),
            indented_code_text_editor_content: iced::widget::text_editor::Content::new(),
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
            CodeIndenterMsg::OrigCodeChange(orig_code) => self.orig_code = orig_code,
            CodeIndenterMsg::IndentCodeNow => {
                println!("IndentCode -> orig_code:{}", &self.orig_code);
                self.indented_code = code_indenter(self.orig_code.clone(), ProgramLanguages::Json);
                println!("IndentCode -> indented_code:{}", &self.indented_code);
            }
        }
    }

    pub fn view(&self) -> Column<'_, CodeIndenterMsg> {
        let mut layout = Column::new().padding(30);
        layout = layout.push(
            row![text("code indenter").size(60),]
                .spacing(50)
                .push(text("positive toolbox").size(40))
                .align_y(iced::Bottom),
        );
        layout = layout.push(text("請輸入程式碼"));
        layout = layout.push(
            iced::widget::text_input("輸入程式碼...", &self.orig_code)
                .on_input(CodeIndenterMsg::OrigCodeChange)
                .size(20),
        );
        let submit_btn = button("縮排")
            .on_press(CodeIndenterMsg::IndentCodeNow)
            .width(200)
            .height(80);
        layout = layout.push(submit_btn);
        layout = layout.push(text_editor(&self.indented_code_text_editor_content));
        return layout;
    }

    pub fn title(&self) -> String {
        return String::from("code_indenter — positive_toolbox");
    }

    pub fn theme(&self) -> Option<iced::Theme> {
        Some(iced::Theme::Dark)
    }
}

enum ProgramLanguages {
    Json,
}

fn code_indenter(orig_code: String, lang: ProgramLanguages) -> String {
    match lang {
        ProgramLanguages::Json => {
            let result_i: serde_json::Result<serde_json::Value> = serde_json::from_str(&orig_code);
            match result_i {
                Ok(i) => {
                    match serde_json::to_string_pretty(&i) {
                        Ok(i2) => {
                            return i2;
                        }
                        Err(e) => {
                            return format!("錯誤！err-msg:{}", e);
                        }
                    };
                    //return indented_code;
                }
                Err(e) => {
                    return format!("錯誤！err-msg:{}", e);
                }
            }
        }
    }
}
