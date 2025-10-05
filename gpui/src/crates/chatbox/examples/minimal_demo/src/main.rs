// 最小化的 Chatbox UI 演示
// 只依赖 GPUI，没有其他复杂的依赖

use gpui::*;

pub struct MinimalChatDemo {
    messages: Vec<String>,
    click_count: usize,
}

impl MinimalChatDemo {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            messages: vec![
                "Welcome to the Minimal Chatbox!".to_string(),
                "This demo uses only GPUI core functionality.".to_string(),
                "No webrtc, no complex dependencies!".to_string(),
            ],
            click_count: 0,
        }
    }

    fn add_message(&mut self, cx: &mut Context<Self>) {
        self.click_count += 1;
        self.messages.push(format!("Click #{}: Hello from the chatbox!", self.click_count));
        cx.notify();
    }

    fn clear_messages(&mut self, cx: &mut Context<Self>) {
        self.messages.clear();
        self.messages.push("All messages cleared! Click below to add new ones.".to_string());
        self.click_count = 0;
        cx.notify();
    }
}

impl Render for MinimalChatDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf3f4f6))
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_3()
                    .bg(rgb(0x1f2937))
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
                                    .text_color(rgb(0xf9fafb))
                                    .text_size(px(16.0))
                                    .child("Minimal Chatbox UI")
                            )
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .bg(rgb(0x374151))
                            .rounded()
                            .text_size(px(11.0))
                            .text_color(rgb(0xf9fafb))
                            .child("Clear")
                            .on_click(cx.listener(Self::clear_messages))
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
                    .overflow_y_scroll()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .children(
                                self.messages.iter().enumerate().map(|(i, message)| {
                                    let bg_colors = [
                                        rgb(0xffffff), rgb(0xf9fafb), rgb(0xf3f4f6), rgb(0xfeefc3)
                                    ];
                                    let text_colors = [
                                        rgb(0x111827), rgb(0x374151), rgb(0x6b7280), rgb(0x854d0e)
                                    ];

                                    div()
                                        .bg(bg_colors[i % bg_colors.len()])
                                        .px_3()
                                        .py_2()
                                        .rounded()
                                        .border_1()
                                        .border_color(rgb(0xe5e7eb))
                                        .child(
                                            div()
                                                .text_color(text_colors[i % text_colors.len()])
                                                .text_size(px(13.0))
                                                .child(format!("{}: {}", i + 1, message))
                                        )
                                })
                            )
                    )
            )
            .child(
                // Input simulation area
                div()
                    .p_4()
                    .border_t_2()
                    .border_color(rgb(0xd1d5db))
                    .bg(rgb(0xffffff))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .px_4()
                                    .py_3()
                                    .bg(rgb(0xf9fafb))
                                    .border_2()
                                    .border_dashed()
                                    .border_color(rgb(0x9ca3af))
                                    .rounded()
                                    .text_color(rgb(0x6b7280))
                                    .text_size(px(13.0))
                                    .child("📝 Simulated Input Area")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .flex_1()
                                            .px_4()
                                            .py_2()
                                            .bg(rgb(0x1f2937))
                                            .rounded()
                                            .text_color(rgb(0xf9fafb))
                                            .text_size(px(13.0))
                                            .child("Click to 'Send Message'")
                                            .on_click(cx.listener(Self::add_message))
                                    )
                                    .child(
                                        div()
                                            .px_4()
                                            .py_2()
                                            .bg(rgb(0x10b981))
                                            .rounded()
                                            .text_color(rgb(0xffffff))
                                            .text_size(px(13.0))
                                            .child("Send")
                                            .on_click(cx.listener(Self::add_message))
                                    )
                            )
                    )
            )
            .child(
                // Footer
                div()
                    .px_4()
                    .py_2()
                    .border_t_1()
                    .border_color(rgb(0xe5e7eb))
                    .bg(rgb(0xf9fafb))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .text_color(rgb(0x6b7280))
                                    .text_size(px(11.0))
                                    .child("✨ Minimal dependency demo - No webrtc!")
                            )
                            .child(
                                div()
                                    .text_color(rgb(0x9ca3af))
                                    .text_size(px(10.0))
                                    .child(format!("{} messages | {} clicks", self.messages.len(), self.click_count))
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
                    title: Some("Minimal Chatbox - No WebRTC".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(400.0), px(250.0)),
                    size: size(px(600.0), px(450.0)),
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
                let demo = cx.new(|cx| MinimalChatDemo::new(cx));
                demo.into()
            },
        ) {
            Ok(_) => println!("✅ Minimal Chatbox demo launched successfully!"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}