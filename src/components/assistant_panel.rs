// Assistant Panel - Main container component

use gpui::{prelude::*, *};
use crate::state::ConversationState;
use crate::theme::Theme;
use super::message_list::render_message_list;
use super::message_input::render_message_input;

/// Main assistant panel component
pub struct AssistantPanel {
    state: ConversationState,
    theme: Theme,
}

impl AssistantPanel {
    /// Create a new assistant panel
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            state: ConversationState::new(),
            theme: Theme::yuanbao(),
        }
    }

    /// Handle send message action
    fn send_message(&mut self, cx: &mut Context<Self>) {
        if self.state.send_message() {
            cx.notify();
        }
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(self.theme.background)
            .flex()
            .flex_col()
            .child(render_message_list(self.state.messages(), &self.theme))
            .child(render_message_input(
                self.state.input(),
                &self.theme,
                cx.listener(|this, _, _, cx| {
                    this.send_message(cx);
                }),
            ))
    }
}

