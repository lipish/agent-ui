use gpui::*;

pub struct ChatDemo {
    message_count: usize,
}

impl ChatDemo {
    pub fn new(_cx: &mut App) -> Self {
        Self { message_count: 2 }
    }
}

impl Render for ChatDemo {
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
                    .px_4()
                    .py_3()
                    .bg(rgb(0x6366f1))
                    .child(
                        div()
                            .text_color(rgb(0xffffff))
                            .text_size(px(18.0))
                            .child("💬 Chatbox UI Demo")
                    )
            )
            .child(
                // Main content area
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .p_4()
                    .child(
                        // Message display
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .child(
                                div()
                                    .bg(rgb(0xe3f2fd))
                                    .px_4()
                                    .py_3()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(rgb(0xbbdefb))
                                    .child(
                                        div()
                                            .text_color(rgb(0x1565c0))
                                            .text_size(px(14.0))
                                            .child("Message 1: Welcome to the chatbox!")
                                    )
                            )
                            .child(
                                div()
                                    .bg(rgb(0xf3e5f5))
                                    .px_4()
                                    .py_3()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(rgb(0xe1bee7))
                                    .child(
                                        div()
                                            .text_color(rgb(0x6a1b9a))
                                            .text_size(px(14.0))
                                            .child("Message 2: This shows the UI layout.")
                                    )
                            )
                            .child(
                                div()
                                    .bg(rgb(0xe8f5e8))
                                    .px_4()
                                    .py_3()
                                    .rounded_md()
                                    .border_1()
                                    .border_color(rgb(0xc8e6c9))
                                    .child(
                                        div()
                                            .text_color(rgb(0x2e7d32))
                                            .text_size(px(14.0))
                                            .child(format!("Total messages: {}", self.message_count))
                                    )
                            )
                    )
            )
            .child(
                // Input area
                div()
                    .p_4()
                    .border_t_1()
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
                                    .bg(rgb(0xf9fafb))
                                    .border_1()
                                    .border_color(rgb(0xd1d5db))
                                    .rounded_md()
                                    .child(
                                        div()
                                            .text_color(rgb(0x9ca3af))
                                            .text_size(px(14.0))
                                            .child("Type your message here...")
                                    )
                            )
                            .child(
                                div()
                                    .px_6()
                                    .py_3()
                                    .bg(rgb(0x6366f1))
                                    .rounded_md()
                                    .border_1()
                                    .border_color(rgb(0x4f46e5))
                                    .child(
                                        div()
                                            .text_color(rgb(0xffffff))
                                            .text_size(px(14.0))
                                            .child("Send")
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
                    .bg(rgb(0xf8f9fa))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_color(rgb(0x6b7280))
                                    .text_size(px(12.0))
                                    .child("ℹ️")
                            )
                            .child(
                                div()
                                    .text_color(rgb(0x6b7280))
                                    .text_size(px(12.0))
                                    .child("This is a static UI preview")
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
                    title: Some("Chatbox UI Demo".into()),
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
                let demo = cx.new(|cx| ChatDemo::new(cx));
                demo.into()
            },
        ) {
            Ok(_) => println!("✅ Chatbox UI demo launched successfully!"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}