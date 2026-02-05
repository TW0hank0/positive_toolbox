use std::collections::HashMap;

use iced::Center;
use iced::widget::{Column, button, column, row, scrollable, text};

pub fn main() -> iced::Result {
    iced::run(Toolbox::update, Toolbox::view)
}

#[derive(Default)]
struct Toolbox {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Tools {
    UnitConversion,
    CodeIndenter,
}

impl Toolbox {
    fn update(&mut self, message: Tools) {
        match message {
            Tools::UnitConversion => {
                println!("UintConversion");
            }
            Tools::CodeIndenter => {
                println!("CodeIndenter")
            }
        }
    }

    fn view(&self) -> Column<'_, Tools> {
        let mut tools = HashMap::new();
        tools.insert("單位轉換器", Tools::UnitConversion);
        tools.insert("程式碼縮排", Tools::CodeIndenter);
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
            button("加").on_press(Message::Increment),
            text(self.value).size(50),
            button("減").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_x(Center) */
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
