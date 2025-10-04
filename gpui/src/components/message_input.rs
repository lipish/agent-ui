// MessageInput Component - Specialized input for sending messages
// Combines TextInput with attachment and send buttons

use crate::components::text_input::TextInput;
use crate::theme::Theme;
use gpui::prelude::*;
use gpui::{
    div, px, svg, App, Context, Entity, FocusHandle, Focusable, IntoElement, KeyDownEvent, Render,
    Window,
};

/// Callback type for when a message is sent
pub type OnSendCallback = Box<dyn Fn(String, &mut Window, &mut Context<MessageInput>) + 'static>;

/// Callback type for when attachment button is clicked
pub type OnAttachCallback = Box<dyn Fn(&mut Window, &mut Context<MessageInput>) + 'static>;

/// MessageInput - A specialized input component for chat messages
/// 
/// Features:
/// - Text input with all editing capabilities
/// - Attachment button (left side)
/// - Send button (right side)
/// - Keyboard shortcut (⌘↵ to send)
/// - Auto-clear after sending
pub struct MessageInput {
    text_input: Entity<TextInput>,
    on_send: Option<OnSendCallback>,
    on_attach: Option<OnAttachCallback>,
}

impl MessageInput {
    /// Create a new MessageInput with the given TextInput entity
    pub fn new(text_input: Entity<TextInput>, _cx: &mut Context<Self>) -> Self {
        Self {
            text_input,
            on_send: None,
            on_attach: None,
        }
    }

    /// Set the callback for when a message is sent
    pub fn on_send<F>(mut self, callback: F) -> Self
    where
        F: Fn(String, &mut Window, &mut Context<MessageInput>) + 'static,
    {
        self.on_send = Some(Box::new(callback));
        self
    }

    /// Set the callback for when attachment button is clicked
    pub fn on_attach<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<MessageInput>) + 'static,
    {
        self.on_attach = Some(Box::new(callback));
        self
    }

    /// Get the current text content
    pub fn text(&self, cx: &App) -> String {
        self.text_input.read(cx).text()
    }

    /// Clear the input
    pub fn clear(&self, cx: &mut Context<Self>) {
        self.text_input.update(cx, |input, cx| {
            input.clear(cx);
        });
    }

    /// Set placeholder text
    pub fn set_placeholder(&self, placeholder: impl Into<gpui::SharedString>, cx: &mut Context<Self>) {
        self.text_input.update(cx, |input, _| {
            input.set_placeholder(placeholder);
        });
    }

    /// Handle send action
    fn send_message(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.text_input.update(cx, |input, cx| {
            let text = input.text();
            input.clear(cx);
            text
        });

        if !text.trim().is_empty() {
            if let Some(callback) = self.on_send.as_ref() {
                callback(text, window, cx);
            }
        }
    }





    /// Render the send button - simplified version
    fn render_send_button(&self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let has_text = !self.text_input.read(cx).text().trim().is_empty();

        div()
            .id("send-button")
            .w(px(40.))
            .h(px(40.))
            .rounded(px(8.))
            .bg(if has_text {
                theme.primary
            } else {
                theme.muted
            })
            .cursor(if has_text {
                gpui::CursorStyle::PointingHand
            } else {
                gpui::CursorStyle::Arrow
            })
            .when(has_text, |this| {
                this.hover(|this| this.opacity(0.9))
            })
            .flex()
            .items_center()
            .justify_center()
            .on_click(cx.listener(|this, _, window, cx| {
                this.send_message(window, cx);
            }))
            .child(
                svg()
                    .path(crate::assets::icons::PAPER_AIRPLANE)
                    .size(px(20.))
                    .text_color(if has_text {
                        theme.primary_foreground
                    } else {
                        theme.muted_foreground
                    }),
            )
    }
}

impl Render for MessageInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>().clone();

        div()
            .flex()
            .items_center()
            .gap_3()
            .w_full()
            .child(
                // Text input - takes most space
                div()
                    .flex_1()
                    .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                        // Enter to send (without modifiers)
                        if event.keystroke.key == "enter"
                            && !event.keystroke.modifiers.shift
                            && !event.keystroke.modifiers.alt
                        {
                            this.send_message(window, cx);
                        }
                    }))
                    .child(self.text_input.clone())
            )
            .child(
                // Send button
                self.render_send_button(&theme, cx)
            )
    }
}

impl Focusable for MessageInput {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.text_input.read(cx).focus_handle(cx)
    }
}

