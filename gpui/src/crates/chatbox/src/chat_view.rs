use gpui::*;
use ui::prelude::*;
use editor::EditorMode;
use crate::Chatbox;
use crate::message_editor::MessageEditorEvent;

pub struct ChatView {
    chatbox: Entity<Chatbox>,
}

impl ChatView {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let chatbox = cx.new(|cx| {
            Chatbox::new(
                "Type your message here...",
                EditorMode::AutoHeight {
                    min_lines: 1,
                    max_lines: Some(5),
                },
                _window,
                cx,
            )
        });

        // For this demo, we'll keep it simple and not handle events
        // The chatbox is mainly for UI demonstration

        Self { chatbox }
    }
}

impl Render for ChatView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .flex_col()
            .bg(cx.theme().colors().panel_background)
            .child(
                div()
                    .p_4()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new(IconName::Chat)
                                    .size(IconSize::Medium)
                                    .color(Color::Accent)
                            )
                            .child(
                                Label::new("Simple Chatbox Demo")
                                    .size(LabelSize::Large)
                            )
                    )
                    .child(
                        div()
                            .mt_2()
                            .child(
                                Label::new("This is a simplified chatbox UI component.")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                    )
            )
            .child(
                div()
                    .flex_1()
                    .child(self.chatbox.clone())
            )
            .child(
                div()
                    .p_3()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new(IconName::Info)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted)
                            )
                            .child(
                                Label::new("Press Enter to send, Escape to cancel")
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted)
                            )
                    )
            )
    }
}