use gpui::{Context, Render, Window};
use ui::prelude::*;

pub struct CopilotChat {
    message: SharedString,
}

impl CopilotChat {
    pub fn new() -> Self {
        Self {
            message: "".into(),
        }
    }

    pub fn set_message(&mut self, message: SharedString) {
        self.message = message;
    }

    pub fn clear(&mut self) {
        self.message = "".into();
    }
}

impl Render for CopilotChat {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .child(
                div()
                    .p_4()
                    .child(
                        if self.message.is_empty() {
                            Label::new("Chat interface ready - no messages yet")
                                .color(Color::Muted)
                        } else {
                            Label::new(self.message.clone())
                        }
                    )
            )
    }
}