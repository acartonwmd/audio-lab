use iced::{Element, Theme, widget::{button, column, text}, window::Id};
use crate::gui::types::message::Message;

pub struct Beatnik {
    id: Id,
    counter: i32,
}

impl Beatnik {
    pub fn new() -> Self {
        Self {
            id: Id::unique(),
            counter: 0,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.counter += 1,
            Message::Decrement => self.counter -= 1,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        column![
            text!("Hello, bitches!"),
            button("+").on_press(Message::Increment),
            text!("{}", self.counter),
            button("+").on_press(Message::Decrement),
        ].into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dracula
    }
}
