use gpui::*;

struct StandaloneDemo {
    message: String,
}

impl StandaloneDemo {
    fn new(_cx: &mut App) -> Self {
        Self {
            message: "Chat interface ready - no messages yet".to_string(),
        }
    }
}

impl Render for StandaloneDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .flex_col()
            .bg(rgb(0xf8f9fa))
            .child(
                // Header with title
                div()
                    .w_full()
                    .px_4()
                    .py_3()
                    .bg(rgb(0x6366f1))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w_4()
                                    .h_4()
                                    .bg(rgb(0xffffff))
                                    .rounded_full()
                            )
                            .child(
                                div()
                                    .text(rgb(0xffffff))
                                    .text_size(px(16.0))
                                    .child("Simple Chatbox Demo")
                            )
                    )
            )
            .child(
                // Chat area
                div()
                    .flex_1()
                    .p_4()
                    .child(
                        div()
                            .mb_4()
                            .p_3()
                            .bg(rgb(0xe3f2fd))
                            .rounded_md()
                            .child(
                                div()
                                    .text(rgb(0x1565c0))
                                    .text_size(px(14.0))
                                    .child(&self.message)
                            )
                    )
                    .child(
                        div()
                            .p_3()
                            .bg(rgb(0xf3f4f6))
                            .rounded_md()
                            .child(
                                div()
                                    .text(rgb(0x6b7280))
                                    .text_size(px(14.0))
                                    .child("This shows the chatbox UI layout")
                            )
                    )
            )
            .child(
                // Input area
                div()
                    .w_full()
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
                                    .px_3()
                                    .py_2()
                                    .bg(rgb(0xf9fafb))
                                    .border_1()
                                    .border_color(rgb(0xd1d5db))
                                    .rounded_md()
                                    .child(
                                        div()
                                            .text(rgb(0x9ca3af))
                                            .text_size(px(14.0))
                                            .child("Type your message here...")
                                    )
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(rgb(0x6366f1))
                                    .rounded_md()
                                    .child(
                                        div()
                                            .text(rgb(0xffffff))
                                            .text_size(px(14.0))
                                            .child("Send")
                                    )
                            )
                    )
            )
            .child(
                // Footer
                div()
                    .w_full()
                    .p_3()
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
                                    .w_3()
                                    .h_3()
                                    .bg(rgb(0x6c757d))
                                    .rounded_full()
                            )
                            .child(
                                div()
                                    .text(rgb(0x6c757d))
                                    .text_size(px(12.0))
                                    .child("Static UI preview - no interaction")
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        let _ = cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Chatbox UI Preview".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(400.0), px(300.0)),
                    size: size(px(500.0), px(400.0)),
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
                let demo = cx.new(|_cx| StandaloneDemo::new(_cx));
                demo.into()
            },
        );
    });
}