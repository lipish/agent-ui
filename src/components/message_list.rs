// Message List Component

use gpui::{prelude::*, *};
use crate::models::Message;
use crate::theme::Theme;

/// Render message list
pub fn render_message_list(messages: &[Message], theme: &Theme) -> impl IntoElement {
    div()
        .id("messages")
        .flex_1()
        .overflow_scroll()
        .px(px(24.))
        .py(px(16.))
        .flex()
        .flex_col()
        .gap_3()
        .children(messages.iter().map(|msg| {
            render_message(msg, theme)
        }))
}

fn render_message(msg: &Message, theme: &Theme) -> impl IntoElement {
    let is_user = msg.is_user();

    div()
        .w_full()
        .flex()
        .flex_col()
        .gap_1()
        .when(is_user, |this| this.items_end())
        .child(
            // Message bubble
            div()
                .max_w(px(480.))
                .px_4()
                .py_3()
                .rounded_xl()
                .bg(theme.card)
                .text_color(theme.foreground)
                .child(
                    div()
                        .text_sm()
                        .line_height(rems(1.6))
                        .child(msg.content.clone())
                ),
        )
}

// Re-export for compatibility
pub use render_message_list as MessageList;

