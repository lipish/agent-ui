use gpui::{div, prelude::*, rgb, IntoElement, ParentElement, Render, Styled};
use crate::models::message::Message;

pub struct Conversation {
    messages: Vec<Message>,
}

impl Conversation {
    pub fn new(messages: Vec<Message>) -> Self {
        Self { messages }
    }
}

impl Render for Conversation {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        div().children(self.messages.iter().map(|message| {
            let (bg_color, text_color) = if message.is_user() {
                (rgb(0x1e88e5), rgb(0xffffff))
            } else {
                (rgb(0x424242), rgb(0xffffff))
            };

            div()
                .p_2()
                .m_2()
                .rounded_lg()
                .bg(bg_color)
                .child(div().text_color(text_color).child(message.content.clone()))
        }))
    }
}