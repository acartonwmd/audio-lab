use iced::{Element, widget::text, window::Id};
use crate::gui::types::message::Message;

pub struct AudioLab {
    id: Id,
}

impl AudioLab {
    pub fn new() -> Self {
        Self {
            id: Id::unique(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        text!("Hello, bitches from id: {}", self.id).center().into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::StartApp => {},
        }
    }
}
