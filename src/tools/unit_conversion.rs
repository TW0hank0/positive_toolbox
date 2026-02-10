//use std::collections::HashMap;

//use serde;
use iced;
use iced::widget::{Column, Row, button, combo_box, text, text_editor};

use serde_json;

use image;

const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");
const TOOL_NAME: &str = "unit_conversion";

#[derive(Default)]
pub struct CodeIndenter {
    orig_code_text_editor_content: iced::widget::text_editor::Content,
    indented_code: String,
    indented_code_text_editor_content: iced::widget::text_editor::Content,
    selected_program_lang: Option<ProgramLanguages>,
    combo_box_langs: combo_box::State<ProgramLanguages>,
}

fn main() -> iced::Result {
    const ICON_PNG: &[u8] = include_bytes!("../../icon.png");
    let img = image::load_from_memory_with_format(ICON_PNG, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (img_width, img_height) = img.dimensions();
    let mut window_settings = iced::window::Settings::default();
    window_settings.maximized = true;
    window_settings.icon =
        iced::window::icon::from_rgba(img.into_raw(), img_width, img_height).ok();
    //
    iced::application(CodeIndenter::new, CodeIndenter::update, CodeIndenter::view)
        .theme(CodeIndenter::theme)
        .title(CodeIndenter::title)
        .font(include_bytes!(
            "../../assets/fonts/Noto_Sans_TC/static/NotoSansTC-Regular.ttf"
        ))
        .window(window_settings)
        .run()
}

#[derive(Debug, Clone)]
pub enum CodeIndenterMsg {
    OrigCodeChange(iced::widget::text_editor::Action),
    UnitConversion,
    CodeIndenter,
    IndentCodeNow,
    IndentedCodeChange(iced::widget::text_editor::Action),
    LangSelected(ProgramLanguages),
}

impl CodeIndenter {
    pub fn new() -> Self {
        let combo_box_langs = combo_box::State::with_selection(
            vec![ProgramLanguages::Json],
            Some(&ProgramLanguages::Json),
        );
        return Self {
            selected_program_lang: None,
            combo_box_langs: combo_box_langs,
            orig_code_text_editor_content: iced::widget::text_editor::Content::new(),
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
            CodeIndenterMsg::OrigCodeChange(act) => {
                self.orig_code_text_editor_content.perform(act);
            }
            CodeIndenterMsg::IndentCodeNow => {
                println!(
                    "IndentCode -> orig_code:{}",
                    &self.orig_code_text_editor_content.text()
                );
                self.indented_code = code_indenter(
                    self.orig_code_text_editor_content.text(),
                    ProgramLanguages::Json,
                );
                println!("IndentCode -> indented_code:{}", &self.indented_code);
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
        }
    }

    pub fn view(&self) -> Column<'_, CodeIndenterMsg> {
        let mut layout = Column::new().padding(30);
        let mut row_layout = Row::new().padding(10);
        row_layout = row_layout.push(
            text(TOOL_NAME)
                .size(70)
                .align_x(iced::alignment::Horizontal::Left)
                .align_y(iced::alignment::Vertical::Bottom),
        );
        row_layout = row_layout.spacing(50);
        row_layout = row_layout.push(
            text(PROJECT_NAME)
                .size(50)
                .align_x(iced::alignment::Horizontal::Left)
                .align_y(iced::alignment::Vertical::Bottom),
        );
        layout = layout.push(row_layout);
        //
        layout = layout.push(text("選擇一種語言").size(24));
        layout = layout.push(
            combo_box(
                &self.combo_box_langs,
                "選擇一種語言",
                self.selected_program_lang.as_ref(),
                CodeIndenterMsg::LangSelected,
            )
            .width(200),
        );
        //layout = layout.push(text("請輸入程式碼"));
        layout = layout.push(
            text_editor(&self.orig_code_text_editor_content)
                .on_action(CodeIndenterMsg::OrigCodeChange)
                .placeholder("code here...")
                .size(20),
        );
        layout = layout.spacing(30);
        let submit_btn = button(
            text("縮排")
                .size(20)
                .align_y(iced::alignment::Vertical::Center)
                .align_x(iced::alignment::Horizontal::Center),
        )
        .on_press(CodeIndenterMsg::IndentCodeNow)
        .width(160)
        .height(60);
        layout = layout.push(submit_btn);
        layout = layout.spacing(30);
        layout = layout.push(
            text_editor(&self.indented_code_text_editor_content)
                .on_action(CodeIndenterMsg::IndentedCodeChange)
                .placeholder("縮排後的程式碼輸出...")
                .size(20),
        );
        return layout;
    }

    pub fn title(&self) -> String {
        return String::from("code_indenter — positive_toolbox");
    }

    pub fn theme(&self) -> Option<iced::Theme> {
        Some(iced::Theme::Dark)
    }
}

#[derive(Clone, Debug)]
pub enum ProgramLanguages {
    Json,
}

impl std::fmt::Display for ProgramLanguages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Json => "json",
        })
    }
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
