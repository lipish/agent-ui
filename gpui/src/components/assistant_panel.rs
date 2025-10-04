use crate::components::message_input::MessageInput;
use crate::components::text_input::TextInput;
use crate::state::ConversationState;
use crate::theme::Theme;
use gpui::{prelude::*, *};

pub struct AssistantPanel {
    state: ConversationState,
    message_input: Entity<MessageInput>,
}

impl AssistantPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        // Create TextInput
        let text_input = cx.new(|cx| {
            let mut input = TextInput::new(cx);
            input.set_placeholder("Message...");
            input
        });

        // Create MessageInput with callbacks
        let panel_entity = cx.entity();
        let message_input = cx.new(|cx| {
            MessageInput::new(text_input, cx).on_send(move |text, _window, cx| {
                panel_entity.update(cx, |panel, cx| {
                    panel.handle_message_sent(text, cx);
                });
            })
        });

        Self {
            state: ConversationState::new(),
            message_input,
        }
    }

    fn handle_message_sent(&mut self, text: String, cx: &mut Context<Self>) {
        if !text.trim().is_empty() {
            self.state.add_user_message(text);
            let responses = [
                "我理解了你的问题。",
                "这是一个很好的想法！",
                "让我帮你分析一下。",
                "我会尽力帮助你。",
            ];
            let response = responses[self.state.messages().len() % responses.len()];
            self.state.add_assistant_message(response.to_string());
            cx.notify();
        }
    }

    fn render_input_area(&self, theme: &Theme, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px(px(16.))
            .py(px(12.))
            .bg(theme.background)
            .child(self.message_input.clone())
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>().clone();

        div()
            .size_full()
            .bg(theme.background)
            .flex()
            .flex_col()
            .child(
                div()
                    .child(crate::components::message_list::render_message_list(
                        self.state.messages(),
                        &theme,
                    ))
                    .flex_grow(),
            )
            .child(self.render_input_area(&theme, cx))
    }
}

impl Focusable for AssistantPanel {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.message_input.read(cx).focus_handle(cx)
    }
}
