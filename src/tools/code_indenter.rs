//use std::collections::HashMap;

//use serde;

use iced::widget::{Column, text, text_input};

#[derive(Default)]
pub struct CodeIndenter {
    orig_code: String,
    indented_code: String,
}

fn main() -> iced::Result {
    iced::run(CodeIndenter::update, CodeIndenter::view)
}

#[derive(Debug, Clone)]
pub enum CodeIndenterMsg {
    OrigCodeChange(String),
    UnitConversion,
    CodeIndenter,
}

impl CodeIndenter {
    pub fn new() -> Self {
        return Self {
            orig_code: String::new(),
            indented_code: String::new(),
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
        }
    }

    pub fn view(&self) -> Column<'_, CodeIndenterMsg> {
        let mut layout = Column::new().padding(20);
        layout = layout.push(text("請輸入程式碼"));
        layout = layout.push(
            text_input("輸入程式碼...", &self.orig_code).on_input(CodeIndenterMsg::OrigCodeChange),
        );
        return layout;
    }
}
