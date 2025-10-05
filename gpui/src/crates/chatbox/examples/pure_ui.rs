// 纯 UI 演示 - 不依赖任何 Zed 设置系统
// 只使用 GPUI 的原生功能

use gpui::*;

pub struct PureChatDemo {
    messages: Vec<String>,
    click_count: usize,
    show_input_hint: bool,
}

impl PureChatDemo {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            messages: vec![
                "Welcome to the Pure Chatbox UI!".to_string(),
                "This demo uses zero Zed dependencies.".to_string(),
                "No settings system, no theme system, no webrtc!".to_string(),
            ],
            click_count: 0,
            show_input_hint: true,
        }
    }

    fn add_message(&mut self, cx: &mut Context<Self>) {
        self.click_count += 1;
        let messages = [
            "Hello from the chatbox! 👋",
            "This is a clean, minimal UI.",
            "No complex dependencies required!",
            "Pure GPUI + Rust = Beautiful UI",
            "Click again for more messages!",
        ];

        let message = messages[self.click_count % messages.len()];
        self.messages.push(format!("Message {}: {}", self.click_count, message));
        cx.notify();
    }

    fn clear_all(&mut self, cx: &mut Context<Self>) {
        self.messages.clear();
        self.messages.push("Chat cleared! Ready for new messages.".to_string());
        self.click_count = 0;
        cx.notify();
    }

    fn toggle_input_hint(&mut self, cx: &mut Context<Self>) {
        self.show_input_hint = !self.show_input_hint;
        if self.show_input_hint {
            self.messages.push("Input hints enabled! ✨".to_string());
        } else {
            self.messages.push("Input hints disabled.".to_string());
        }
        cx.notify();
    }
}

impl Render for PureChatDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf1f5f9))
            .child(
                // Modern header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_5()
                    .py_4()
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
                                    .child("Pure Chatbox UI")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0x1e293b))
                                    .rounded()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0x94a3b8))
                                    .child("Clear")
                                    .on_click(cx.listener(Self::clear_all))
                            )
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0x475569))
                                    .rounded()
                                    .text_size(px(11.0))
                                    .text_color(rgb(0xe2e8f0))
                                    .child("Toggle")
                                    .on_click(cx.listener(Self::toggle_input_hint))
                            )
                    )
            )
            .child(
                // Messages area with gradient background
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
                            .gap_2()
                            .children(
                                self.messages.iter().enumerate().map(|(i, message)| {
                                    let color_schemes = [
                                        (rgb(0xdbf4ff), rgb(0x0c4a6e)),  // Blue
                                        (rgb(0xf3e8ff), rgb(0x581c87)),  // Purple
                                        (rgb(0xf0fdf4), rgb(0x14532d)),  // Green
                                        (rgb(0xfff7ed), rgb(0x7c2d12)),  // Orange
                                        (rgb(0xfdf4ff), rgb(0x581c87)),  // Pink
                                        (rgb(0xf0fdfa), rgb(0x166534)),  // Emerald
                                    ];

                                    let (bg_color, text_color) = color_schemes[i % color_schemes.len()];

                                    div()
                                        .bg(bg_color)
                                        .px_4()
                                        .py_3()
                                        .rounded_lg()
                                        .shadow_sm()
                                        .border_1()
                                        .border_color(bg_color)
                                        .child(
                                            div()
                                                .flex()
                                                .items_start()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .text_color(text_color)
                                                        .text_size(px(12.0))
                                                        .child(format!("#{:02}", i + 1))
                                                )
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .text_color(text_color)
                                                        .text_size(px(14.0))
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
                    .shadow_inner()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                if self.show_input_hint {
                                    div()
                                        .px_4()
                                        .py_2()
                                        .bg(rgb(0xf0f9ff))
                                        .border_1()
                                        .border_color(rgb(0x7dd3fc))
                                        .rounded()
                                        .text_color(rgb(0x075985))
                                        .text_size(px(12.0))
                                        .child("💡 Click any button below to interact")
                                } else {
                                    div()
                                        .px_4()
                                        .py_2()
                                        .bg(rgb(0xf8fafc))
                                        .border_1()
                                        .border_color(rgb(0xe2e8f0))
                                        .rounded()
                                        .text_color(rgb(0x64748b))
                                        .text_size(px(12.0))
                                        .child("Input hints are disabled")
                                }
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
                                            .bg(rgb(0xf8fafc))
                                            .border_2()
                                            .border_dashed()
                                            .border_color(rgb(0cbd5e))
                                            .rounded_lg()
                                            .text_color(rgb(0x166534))
                                            .text_size(px(14.0))
                                            .child("📝 Click to simulate typing")
                                            .on_click(cx.listener(Self::add_message))
                                    )
                                    .child(
                                        div()
                                            .px_6()
                                            .py_3()
                                            .bg(rgb(0x10b981))
                                            .rounded_lg()
                                            .shadow_md()
                                            .text_size(px(14.0))
                                            .text_color(rgb(0xffffff))
                                            .child("Send")
                                            .on_click(cx.listener(Self::add_message))
                                    )
                                    .child(
                                        div()
                                            .px_6()
                                            .py_3()
                                            .bg(rgb(0x3b82f6))
                                            .rounded_lg()
                                            .shadow_md()
                                            .text_size(px(14.0))
                                    .text_color(rgb(0xffffff))
                                            .child("Quick")
                                    .on_click(cx.listener(Self::add_message))
                                    )
                            )
                    )
            )
            .child(
                // Footer with stats
                div()
                    .px_5()
                    .py_3()
                    .border_t_1()
                    .border_color(rgb(0xe2e8f0))
                    .bg(rgb(0xf8fafc))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify-between()
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
                                            .text_color(rgb(0x475569))
                                            .text_size(px(12.0))
                                            .child("✨ Pure GPUI Demo")
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
                                            .text_size(px(11.0))
                                            .child(format!("{} messages", self.messages.len()))
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x64748b))
                                            .text_size(px(11.0))
                                            .child(format!("{} clicks", self.click_count))
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
                    title: Some("Pure Chatbox - Zero Dependencies".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(350.0), px(200.0)),
                    size: size(px(700.0), px(600.0)),
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
                let demo = cx.new(|cx| PureChatDemo::new(cx));
                demo.into()
            },
        ) {
            Ok(_) => println!("✅ Pure Chatbox UI demo launched successfully!"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}