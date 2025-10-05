use gpui::*;

struct MinimalDemo {
    input_text: String,
}

impl MinimalDemo {
    fn new(_cx: &mut AppContext) -> Self {
        Self {
            input_text: "Hello, this is a minimal chatbox demo!".to_string(),
        }
    }
}

impl Render for MinimalDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf8f9fa))
            .child(
                // Header
                div()
                    .w_full()
                    .p_4()
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
                                gpui::Label::new("Minimal Chatbox Demo")
                                    .color(rgb(0xffffff))
                                    .size(gpui::LabelSize::Large)
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
                                gpui::Label::new(&self.input_text)
                                    .color(rgb(0x1565c0))
                                    .size(gpui::LabelSize::Small)
                            )
                    )
                    .child(
                        div()
                            .p_3()
                            .bg(rgb(0xf3f4f6))
                            .rounded_md()
                            .child(
                                gpui::Label::new("Chat interface ready - no messages yet")
                                    .color(rgb(0x6b7280))
                                    .size(gpui::LabelSize::Small)
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
                                        gpui::Label::new("Type your message here...")
                                            .color(rgb(0x9ca3af))
                                            .size(gpui::LabelSize::Small)
                                    )
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(rgb(0x6366f1))
                                    .rounded_md()
                                    .child(
                                        gpui::Label::new("Send")
                                            .color(rgb(0xffffff))
                                            .size(gpui::LabelSize::Small)
                                    )
                            )
                    )
            )
    }
}

fn main() {
    gpui::App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Minimal Chatbox UI".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(300.0), px(300.0)),
                    size: size(px(500.0), px(400.0)),
                })),
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                decorations: Some(Decorations::Server),
                is_movable: true,
                display_id: None,
                fullscreen: false,
            },
            |_window, cx| {
                let demo = cx.new(|cx| MinimalDemo::new(cx));
                demo.into()
            },
        );
    });
}