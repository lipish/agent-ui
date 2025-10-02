// Message Input Component

use gpui::{prelude::*, *};
use crate::assets::icons;
use crate::theme::Theme;

/// Render message input
pub fn render_message_input<F>(
    input_text: &str,
    theme: &Theme,
    on_send: F,
) -> impl IntoElement
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    div()
        .border_t_1()
        .border_color(theme.border)
        .px(px(24.))
        .py(px(16.))
        .bg(theme.background)
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .child(render_input(input_text, theme))
                .child(render_send_button(theme, on_send)),
        )
}

fn render_input(input_text: &str, theme: &Theme) -> impl IntoElement {
    div()
        .flex_1()
        .px_4()
        .py_3()
        .rounded_full()
        .bg(theme.card)
        .child(
            div()
                .text_sm()
                .text_color(if input_text.is_empty() {
                    theme.muted_foreground
                } else {
                    theme.foreground
                })
                .child(if input_text.is_empty() {
                    "Message AI Assistant...".to_string()
                } else {
                    input_text.to_string()
                }),
        )
}

fn render_send_button<F>(theme: &Theme, on_send: F) -> impl IntoElement
where
    F: Fn(&ClickEvent, &mut Window, &mut App) + 'static,
{
    div()
        .id("send-button")
        .size(px(36.))
        .rounded(px(18.))
        .bg(theme.primary)
        .cursor_pointer()
        .hover(|this| this.opacity(0.8))
        .flex()
        .items_center()
        .justify_center()
        .on_click(on_send)
        .child(
            svg()
                .path(icons::PAPER_AIRPLANE)
                .size(px(18.))
                .text_color(theme.primary_foreground),
        )
}

// Re-export for compatibility
pub use render_message_input as MessageInput;

