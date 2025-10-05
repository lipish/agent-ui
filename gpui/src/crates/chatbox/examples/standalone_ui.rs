// 这是一个完全独立的演示，不依赖 Zed 的复杂模块
// 只使用 GPUI 的基本功能

use gpui::*;

pub struct StandaloneChatDemo {
    input_text: String,
    messages: Vec<String>,
    input_active: bool,
}

impl StandaloneChatDemo {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            input_text: String::new(),
            messages: vec![
                "Welcome to the Chatbox UI Demo!".to_string(),
                "Click the input area to 'type', then click Send.".to_string(),
                "This is a visual demonstration only.".to_string(),
            ],
            input_active: false,
        }
    }

    fn simulate_input(&mut self, cx: &mut Context<Self>) {
        if !self.input_active {
            self.input_text = "Hello, this is a simulated message!".to_string();
            self.input_active = true;
        } else {
            self.input_text = "Another test message...".to_string();
        }
        cx.notify();
    }

    fn send_message(&mut self, cx: &mut Context<Self>) {
        if !self.input_text.trim().is_empty() {
            self.messages.push(format!("You: {}", self.input_text.clone()));
            self.input_text.clear();
            self.input_active = false;
            cx.notify();
        }
    }

    fn clear_chat(&mut self, cx: &mut Context<Self>) {
        self.messages.clear();
        self.messages.push("Chat cleared. Start a new conversation!".to_string());
        cx.notify();
    }
}

impl Render for StandaloneChatDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf8f9fa))
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_3()
                    .bg(rgb(0x2563eb))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w_5()
                                    .h_5()
                                    .bg(rgb(0xffffff))
                                    .rounded_full()
                            )
                            .child(
                                div()
                                    .text_color(rgb(0xffffff))
                                    .text_size(px(18.0))
                                    .child("Standalone Chatbox UI")
                            )
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .bg(rgb(0x1d4ed8))
                            .rounded_md()
                            .text_size(px(12.0))
                            .text_color(rgb(0xffffff))
                            .child("Clear")
                            .on_click(cx.listener(Self::clear_chat))
                    )
            )
            .child(
                // Messages area
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .children(
                                self.messages.iter().enumerate().map(|(i, message)| {
                                    let colors = [
                                        (rgb(0xeff6ff), rgb(0x1e40af)),  // Blue
                                        (rgb(0xf3e8ff), rgb(0x6b21a8)),  // Purple
                                        (rgb(0xf0fdf4), rgb(0x166534)),  // Green
                                        (rgb(0xfff7ed), rgb(0x9a3412)),  // Orange
                                    ];
                                    let (bg_color, text_color) = colors[i % colors.len()];

                                    div()
                                        .bg(bg_color)
                                        .px_4()
                                        .py_3()
                                        .rounded_lg()
                                        .border_1()
                                        .border_color(bg_color)
                                        .shadow_lg()
                                        .child(
                                            div()
                                                .text_color(text_color)
                                                .text_size(px(14.0))
                                                .child(message.clone())
                                        )
                                })
                            )
                    )
            )
            .child(
                // Input area
                div()
                    .p_4()
                    .border_t_2()
                    .border_color(rgb(0xe5e7eb))
                    .bg(rgb(0xffffff))
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .flex_1()
                                    .px_4()
                                    .py_3()
                                    .bg(if self.input_active { rgb(0xf0f9ff) } else { rgb(0xf9fafb) })
                                    .border_2()
                                    .border_color(if self.input_active { rgb(0x3b82f6) } else { rgb(0xd1d5db) })
                                    .rounded_lg()
                                    .cursor_pointer()
                                    .text_color(rgb(0x374151))
                                    .text_size(px(14.0))
                                    .child(if self.input_text.is_empty() {
                                        "Click here to simulate typing..."
                                    } else {
                                        &self.input_text
                                    })
                                    .on_click(cx.listener(Self::simulate_input))
                            )
                            .child(
                                div()
                                    .px_6()
                                    .py_3()
                                    .bg(rgb(0x2563eb))
                                    .rounded_lg()
                                    .border_2()
                                    .border_color(rgb(0x1d4ed8))
                                    .shadow_md()
                                    .text_size(px(14.0))
                                    .text_color(rgb(0xffffff))
                                    .child("Send")
                                    .on_click(cx.listener(Self::send_message))
                            )
                    )
            )
            .child(
                // Footer with info
                div()
                    .px_4()
                    .py_3()
                    .border_t_1()
                    .border_color(rgb(0xe5e7eb))
                    .bg(rgb(0xf8f9fa))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .w_4()
                                            .h_4()
                                            .bg(rgb(0x10b981))
                                            .rounded_full()
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x6b7280))
                                            .text_size(px(12.0))
                                            .child("Interactive UI Demo")
                                    )
                            )
                            .child(
                                div()
                                    .text_color(rgb(0x9ca3af))
                                    .text_size(px(11.0))
                                    .child(format!("{} messages", self.messages.len()))
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
                    title: Some("Chatbox UI - Standalone Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(300.0), px(200.0)),
                    size: size(px(700.0), px(550.0)),
                })),
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                is_resizable: true,
                window_min_size: Some(size(px(400.0), px(300.0))),
                is_minimizable: true,
                window_background: WindowBackgroundAppearance::Transparent,
                app_id: None,
                display_id: None,
                tabbing_identifier: None,
                window_decorations: Some(WindowDecorations::Server),
            },
            |_window, cx| {
                let demo = cx.new(|cx| StandaloneChatDemo::new(cx));
                demo.into()
            },
        ) {
            Ok(_) => println!("✅ Standalone Chatbox UI demo launched successfully!"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}