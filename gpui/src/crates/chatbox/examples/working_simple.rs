// 最简单的可工作版本 - 避免所有复杂的 GPUI 功能
// 只使用最基本的功能来展示 UI 布局

use gpui::*;

pub struct SimpleWorkingDemo {
    messages: Vec<String>,
    counter: usize,
}

impl SimpleWorkingDemo {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            messages: vec![
                "✅ Welcome to the Working Chatbox UI!".to_string(),
                "🚀 No settings system, no webrtc needed".to_string(),
                "💻 Pure GPUI + Rust components".to_string(),
            ],
            counter: 0,
        }
    }

    fn handle_click(&mut self, _event: &(), _window: &mut Window, cx: &mut Context<Self>) {
        self.counter += 1;
        let messages = [
            "Hello from the chatbox! 👋",
            "This is a working UI demo",
            "No complex dependencies",
            "Clean and minimal design",
            "Click again for more messages",
        ];

        let msg = messages[self.counter % messages.len()];
        self.messages.push(format!("Message {}: {}", self.counter, msg));
        cx.notify();
    }
}

impl Render for SimpleWorkingDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf8fafc))
            .child(
                // Simple header
                div()
                    .px_4()
                    .py_3()
                    .bg(rgb(0x1e293b))
                    .child(
                        div()
                            .text_color(rgb(0xffffff))
                            .text_size(px(16.0))
                            .child("🎯 Working Chatbox UI")
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
                    .children(
                        self.messages.iter().enumerate().map(|(i, message)| {
                            let colors = [
                                rgb(0xe0f2fe), // Light blue
                                rgb(0xf3e8ff), // Light purple
                                rgb(0xf0fdf4), // Light green
                            ];
                            let text_colors = [
                                rgb(0x0c4a6e), // Dark blue
                                rgb(0x581c87), // Dark purple
                                rgb(0x14532d), // Dark green
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
            .child(
                // Interactive area
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
                                    .child("💡 Click the button below to add messages")
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
                                            .child("📝 Click to send message")
                                            .on_click(cx.listener(Self::handle_click))
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
                                            .on_click(cx.listener(Self::handle_click))
                                    )
                            )
                    )
            )
            .child(
                // Footer
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
                                    .child("✅ Working Demo - Zero Dependencies")
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

        let result = cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Working Chatbox - No Settings".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(450.0), px(300.0)),
                    size: size(px(600.0), px(450.0)),
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
                let demo = cx.new(|cx| SimpleWorkingDemo::new(cx));
                demo.into()
            },
        );

        match result {
            Ok(_) => println!("✅ Working Chatbox demo launched successfully!"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}