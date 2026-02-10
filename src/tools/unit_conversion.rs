use iced;
use iced::widget::{Column, button, row, text, text_editor};

use serde_json;

use image;

#[derive(Default)]
pub struct CodeIndenter {
    orig_code: String,
    indented_code: String,
    indented_code_text_editor_content: iced::widget::text_editor::Content,
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
    OrigCodeChange(String),
    UnitConversion,
    CodeIndenter,
    IndentCodeNow,
    IndentedCodeChange(iced::widget::text_editor::Action),
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
                self.indented_code_text_editor_content =
                    iced::widget::text_editor::Content::with_text(&self.indented_code);
            }
            CodeIndenterMsg::IndentedCodeChange(act) => {
                self.indented_code_text_editor_content.perform(act);
            }
        }
    }

    pub fn view(&self) -> Column<'_, CodeIndenterMsg> {
        let mut layout = Column::new().padding(30);
        layout = layout.push(
            row![text("code indenter").size(70),]
                .spacing(50)
                .push(text("positive toolbox").size(50))
                .align_y(iced::Bottom),
        );
        layout = layout.push(text("請輸入程式碼"));
        layout = layout.push(
            iced::widget::text_input("輸入程式碼...", &self.orig_code)
                .on_input(CodeIndenterMsg::OrigCodeChange)
                .size(20),
        );
        layout = layout.spacing(30);
        let submit_btn = button(text("縮排").size(24))
            .on_press(CodeIndenterMsg::IndentCodeNow)
            .width(160)
            .height(60);
        layout = layout.push(submit_btn);
        layout = layout.spacing(30);
        layout = layout.push(
            text_editor(&self.indented_code_text_editor_content)
                .on_action(CodeIndenterMsg::IndentedCodeChange),
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
