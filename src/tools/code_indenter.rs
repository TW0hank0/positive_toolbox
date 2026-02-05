//use std::collections::HashMap;

//use serde;

// 此模組由頂層應用程式呼叫，不提供獨立的 `main`
use iced::widget::{Column, text, text_input};

#[derive(Default)]
pub struct CodeIndenter {
    orig_code: String,
    indented_code: String,
}

fn main() -> iced::Result {
    iced::run(CodeIndenter::update, CodeIndenter::view)
}

#[derive(Debug, Clone, Copy)]
pub enum CodeIndenterMsg {
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
        }
    }

    pub fn view(&self) -> Column<'_, CodeIndenterMsg> {
        let mut layout = Column::new().padding(20);
        layout = layout.push(text("請輸入程式碼"));
        layout = layout.push(text_input("輸入程式碼...", &self.orig_code));
        return layout;
    }
}
