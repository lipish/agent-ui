pub mod message_editor;
pub mod copilot_chat;
pub mod chat_view;
pub mod interactive_chatbox;

pub use message_editor::MessageEditor;
pub use copilot_chat::CopilotChat;
pub use chat_view::ChatView;
pub use interactive_chatbox::{InteractiveChatbox, InteractiveChatInput};

use gpui::{Context, Entity, Render, Window};
use ui::prelude::*;

pub struct Chatbox {
    message_editor: Entity<MessageEditor>,
    copilot_chat: Entity<CopilotChat>,
}

impl Chatbox {
    pub fn new(
        placeholder: &str,
        mode: editor::EditorMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let message_editor = cx.new(|cx| MessageEditor::new(placeholder, mode, window, cx));
        let copilot_chat = cx.new(|_| CopilotChat::new());

        Self {
            message_editor,
            copilot_chat,
        }
    }

    pub fn message_editor(&self) -> &Entity<MessageEditor> {
        &self.message_editor
    }

    pub fn copilot_chat(&self) -> &Entity<CopilotChat> {
        &self.copilot_chat
    }

    pub fn is_empty(&self, cx: &App) -> bool {
        self.message_editor.read(cx).is_empty(cx)
    }

    pub fn clear(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.message_editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
        });
        self.copilot_chat.update(cx, |chat, _| {
            chat.clear();
        });
    }

    pub fn text(&self, cx: &App) -> String {
        self.message_editor.read(cx).text(cx)
    }

    fn handle_send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.text(cx);
        if !text.is_empty() {
            self.copilot_chat.update(cx, |chat, _| {
                chat.set_message(format!("Message sent: {}", text).into());
            });
            self.clear(window, cx);
        }
    }

    fn handle_cancel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.clear(window, cx);
        self.copilot_chat.update(cx, |chat, _| {
            chat.set_message("Message cancelled".into());
        });
    }
}

impl Render for Chatbox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .flex_col()
            .child(
                div()
                    .flex_1()
                    .child(self.copilot_chat.clone())
            )
            .child(
                div()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .p_2()
                    .child(self.message_editor.clone())
            )
    }
}