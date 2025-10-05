// 最小化的可交互 Chatbox
// 使用 GPUI 的基本功能，避免复杂的 API

use gpui::*;
use std::time::Duration;

pub struct MinimalInteractiveChatbox {
    messages: Vec<String>,
    input_text: String,
    input_active: bool,
}

impl MinimalInteractiveChatbox {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            messages: vec![
                "🎉 Welcome to the Interactive Chatbox!".to_string(),
                "✨ Click the input area to start typing!".to_string(),
                "🚀 Pure GPUI - no Zed dependencies!".to_string(),
            ],
            input_text: String::new(),
            input_active: false,
        }
    }

    fn activate_input(&mut self, _event: &(), _window: &mut Window, cx: &mut Context<Self>) {
        self.input_active = true;
        cx.notify();
    }

    fn add_sample_text(&mut self, _event: &(), _window: &mut Window, cx: &mut Context<Self>) {
        let sample_texts = [
            "Hello, this is a test message!",
            "How are you doing today?",
            "This chatbox is working great!",
            "I love this minimal design!",
            "Let's try another message...",
        ];

        let current_index = self.messages.len() - 3; // Subtract the welcome messages
        let text = sample_texts[current_index % sample_texts.len()];
        self.input_text = text.to_string();
        cx.notify();
    }

    fn send_message(&mut self, _event: &(), _window: &mut Window, cx: &mut Context<Self>) {
        if !self.input_text.trim().is_empty() {
            self.messages.push(format!("You: {}", self.input_text));
            self.input_text.clear();
            self.input_active = false;
            cx.notify();
        }
    }

    fn clear_chat(&mut self, _event: &(), _window: &mut Window, cx: &mut Context<Self>) {
        self.messages.clear();
        self.messages.push("🧹 Chat cleared! Ready for new messages.".to_string());
        self.input_text.clear();
        self.input_active = false;
        cx.notify();
    }
}

impl Render for MinimalInteractiveChatbox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf8fafc))
            .child(
                // Header
                div()
                    .flex()
                    .px_4()
                    .py_3()
                    .bg(rgb(0x0f172a))
                    .shadow_md()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .w_6()
                                    .h_6()
                                    .bg(rgb(0x10b981))
                                    .rounded_full()
                                    .shadow_lg()
                            )
                            .child(
                                div()
                                    .text_color(rgb(0xf1f5f9))
                                    .text_size(px(20.0))
                                    .child("Interactive Chatbox")
                            )
                    )
            )
            .child(
                // Messages area
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .p_5()
                    .gap_3()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .children(
                                self.messages.iter().enumerate().map(|(i, message)| {
                                    let color_schemes = [
                                        (rgb(0xdbf4ff), rgb(0x0c4a6e)),  // Blue
                                        (rgb(0xf3e8ff), rgb(0x581c87)),  // Purple
                                        (rgb(0xf0fdf4), rgb(0x14532d)),  // Green
                                        (rgb(0xfff7ed), rgb(0x7c2d12)),  // Orange
                                        (rgb(0xfdf4ff), rgb(0x581c87)),  // Pink
                                    ];

                                    let (bg_color, text_color) = color_schemes[i % color_schemes.len()];

                                    div()
                                        .bg(bg_color)
                                        .px_5()
                                        .py_4()
                                        .rounded_lg()
                                        .shadow_sm()
                                        .border_1()
                                        .border_color(bg_color)
                                        .child(
                                            div()
                                                .flex()
                                                .items_start()
                                                .gap_3()
                                                .child(
                                                    div()
                                                        .w_8()
                                                        .h_8()
                                                        .min_w_8()
                                                        .min_h_8()
                                                        .bg(text_color)
                                                        .rounded_full()
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .child(
                                                            div()
                                                                .text_color(bg_color)
                                                                .text_size(px(12.0))
                                                                .child(format!("{}", i + 1))
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .text_color(text_color)
                                                        .text_size(px(15.0))
                                                        .child(message.clone())
                                                )
                                        )
                                })
                            )
                    )
            )
            .child(
                // Interactive input area
                div()
                    .p_5()
                    .border_t_2()
                    .border_color(rgb(0xe2e8f0))
                    .bg(rgb(0xffffff))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(rgb(0xf0f9ff))
                                    .border_1()
                                    .border_color(rgb(0x7dd3fc))
                                    .rounded_md()
                                    .text_color(rgb(0x075985))
                                    .text_size(px(13.0))
                                    .child(if self.input_active {
                                        "✏️  Input active! Click Send or Clear buttons below"
                                    } else {
                                        "💡 Click the input area to simulate typing!"
                                    })
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .child(
                                        div()
                                            .flex_1()
                                            .px_4()
                                            .py_3()
                                            .bg(if self.input_active { rgb(0xf0f9ff) } else { rgb(0xf8fafc) })
                                            .border_2()
                                            .border_color(if self.input_active { rgb(0x10b981) } else { rgb(0xd1d5db) })
                                            .rounded_lg()
                                            .text_color(rgb(0x374151))
                                            .text_size(px(15.0))
                                            .child(if self.input_text.is_empty() {
                                                "📝 Click here to simulate typing..."
                                            } else {
                                                &self.input_text
                                            })
                                            .on_click(cx.listener(Self::add_sample_text))
                                    )
                                    .child(
                                        div()
                                            .px_8()
                                            .py_3()
                                            .bg(if !self.input_text.is_empty() { rgb(0x10b981) } else { rgb(0x9ca3af) })
                                            .rounded_lg()
                                            .shadow_md()
                                            .text_size(px(15.0))
                                            .text_color(rgb(0xffffff))
                                            .child("Send")
                                            .on_click(cx.listener(Self::send_message))
                                    )
                                    .child(
                                        div()
                                            .px_8()
                                            .py_3()
                                            .bg(rgb(0xef4444))
                                            .rounded_lg()
                                            .shadow_md()
                                            .text_size(px(15.0))
                                            .text_color(rgb(0xffffff))
                                            .child("Clear")
                                            .on_click(cx.listener(Self::clear_chat))
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_color(rgb(0x64748b))
                                            .text_size(px(12.0))
                                            .child("📋 How to use:")
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x64748b))
                                            .text_size(px(12.0))
                                            .child("1. Click input area to add sample text")
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x64748b))
                                            .text_size(px(12.0))
                                            .child("2. Click Send to add message to chat")
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x64748b))
                                            .text_size(px(12.0))
                                            .child("3. Click Clear to reset everything")
                                    )
                            )
                    )
            )
            .child(
                // Footer
                div()
                    .px_5()
                    .py_4()
                    .border_t_1()
                    .border_color(rgb(0xe2e8f0))
                    .bg(rgb(0xf8fafc))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        div()
                                            .w_4()
                                            .h_4()
                                            .bg(rgb(0x10b981))
                                            .rounded_full()
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x475569))
                                            .text_size(px(13.0))
                                            .child("✅ Pure GPUI - Interactive Demo")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_4()
                                    .child(
                                        div()
                                            .text_color(rgb(0x64748b))
                                            .text_size(px(12.0))
                                            .child(format!("{} messages", self.messages.len()))
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x9ca3af))
                                            .text_size(px(11.0))
                                            .child(if self.input_active { "active" } else { "inactive" })
                                    )
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);

        match cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Interactive Chatbox - Pure GPUI".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(400.0), px(200.0)),
                    size: size(px(750.0), px(650.0)),
                })),
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                is_resizable: true,
                window_min_size: Some(size(px(600.0), px(500.0))),
                is_minimizable: true,
                window_background: WindowBackgroundAppearance::Transparent,
                app_id: None,
                display_id: None,
                tabbing_identifier: None,
                window_decorations: Some(WindowDecorations::Server),
            },
            |_window, cx| {
                let chatbox = cx.new(|cx| MinimalInteractiveChatbox::new(cx));
                chatbox.into()
            },
        ) {
            Ok(_) => println!("✅ Interactive Chatbox launched successfully!"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}