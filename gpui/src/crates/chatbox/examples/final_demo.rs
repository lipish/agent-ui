// 最终演示版本 - 完全避免 Zed 的设置系统依赖
// 使用纯 GPUI + 简单的颜色

use gpui::*;

pub struct FinalChatDemo {
    messages: Vec<String>,
    click_count: usize,
}

impl FinalChatDemo {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            messages: vec![
                "🎉 Welcome to the Final Chatbox Demo!".to_string(),
                "✅ No settings system required".to_string(),
                "🚀 No webrtc, no complex dependencies".to_string(),
                "💻 Pure GPUI + Rust power".to_string(),
            ],
            click_count: 0,
        }
    }

    fn add_message(&mut self, _event: &(), _window: &mut Window, cx: &mut Context<Self>) {
        self.click_count += 1;
        let demo_messages = [
            "Hello from the interactive chatbox! 👋",
            "This UI is built with pure GPUI components",
            "No Zed settings system needed!",
            "Clean, fast, and dependency-free",
            "Click again for more demo messages",
        ];

        let message = demo_messages[self.click_count % demo_messages.len()];
        self.messages.push(format!("Message {}: {}", self.click_count, message));
        cx.notify();
    }

    fn clear_messages(&mut self, _event: &(), _window: &mut Window, cx: &mut Context<Self>) {
        self.messages.clear();
        self.messages.push("🧹 Chat cleared! Ready for new messages.".to_string());
        self.click_count = 0;
        cx.notify();
    }
}

impl Render for FinalChatDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf8fafc))
            .child(
                // Beautiful header
                div()
                    .flex()
                    .items_center()
                    .px_4()
                    .py_3()
                    .bg(rgb(0x1e293b))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w_5()
                                    .h_5()
                                    .bg(rgb(0x10b981))
                                    .rounded_full()
                            )
                            .child(
                                div()
                                    .text_color(rgb(0xf1f5f9))
                                    .text_size(px(18.0))
                                    .child("Final Chatbox UI")
                            )
                    )
            )
            .child(
                // Messages display area
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_2()
                    .overflow_y_scroll()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .children(
                                self.messages.iter().enumerate().map(|(i, message)| {
                                    let colors = [
                                        rgb(0xe0f2fe), // Light blue
                                        rgb(0xf3e8ff), // Light purple
                                        rgb(0xf0fdf4), // Light green
                                        rgb(0fef3c7), // Light orange
                                        rgb(0xfce7f3), // Light pink
                                    ];
                                    let text_colors = [
                                        rgb(0x0c4a6e), // Dark blue
                                        rgb(0x581c87), // Dark purple
                                        rgb(0x14532d), // Dark green
                                        rgb(0x7c2d12), // Dark orange
                                        rgb(0x831843), // Dark pink
                                    ];

                                    let bg_color = colors[i % colors.len()];
                                    let text_color = text_colors[i % text_colors.len()];

                                    div()
                                        .bg(bg_color)
                                        .px_4()
                                        .py_3()
                                        .rounded_md()
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
                // Interactive input area
                div()
                    .p_4()
                    .border_t_2()
                    .border_color(rgb(0xe5e7eb))
                    .bg(rgb(0xffffff))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(rgb(0xf0f9ff))
                                    .rounded_md()
                                    .text_color(rgb(0x0c4a6e))
                                    .text_size(px(12.0))
                                    .child("💡 Click any button below to interact with the chatbox")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .flex_1()
                                            .px_4()
                                            .py_3()
                                            .bg(rgb(0xf1f5f9))
                                            .border_2()
                                            .border_dashed()
                                            .border_color(rgb(0x94a3b8))
                                            .rounded_md()
                                            .text_color(rgb(0x475569))
                                            .text_size(px(14.0))
                                            .child("📝 Click here to send message")
                                            .on_click(cx.listener(Self::add_message))
                                    )
                                    .child(
                                        div()
                                            .px_6()
                                            .py_3()
                                            .bg(rgb(0x10b981))
                                            .rounded_md()
                                            .text_size(px(14.0))
                                            .text_color(rgb(0xffffff))
                                            .child("Send")
                                            .on_click(cx.listener(Self::add_message))
                                    )
                                    .child(
                                        div()
                                            .px_6()
                                            .py_3()
                                            .bg(rgb(0xef4444))
                                            .rounded_md()
                                            .text_size(px(14.0))
                                            .text_color(rgb(0xffffff))
                                            .child("Clear")
                                            .on_click(cx.listener(Self::clear_messages))
                                    )
                            )
                    )
            )
            .child(
                // Footer with statistics
                div()
                    .px_4()
                    .py_3()
                    .border_t_1()
                    .border_color(rgb(0xe5e7eb))
                    .bg(rgb(0xf8fafc))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .w_3()
                                    .h_3()
                                    .bg(rgb(0x10b981))
                                    .rounded_full()
                            )
                            .child(
                                div()
                                    .text_color(rgb(0x64748b))
                                    .text_size(px(12.0))
                                    .child("✅ Zero Dependencies - Pure GPUI Demo")
                            )
                            .child(
                                div()
                                    .text_color(rgb(0x9ca3af))
                                    .text_size(px(11.0))
                                    .child(format!("Messages: {} | Clicks: {}", self.messages.len(), self.click_count))
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
                    title: Some("Final Chatbox - No Dependencies".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(400.0), px(250.0)),
                    size: size(px(650.0), px(500.0)),
                })),
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                is_resizable: true,
                window_min_size: Some(size(px(500.0), px(400.0))),
                is_minimizable: true,
                window_background: WindowBackgroundAppearance::Transparent,
                app_id: None,
                display_id: None,
                tabbing_identifier: None,
                window_decorations: Some(WindowDecorations::Server),
            },
            |_window, cx| {
                let demo = cx.new(|cx| FinalChatDemo::new(cx));
                demo.into()
            },
        ) {
            Ok(_) => println!("✅ Final Chatbox demo launched successfully!"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}