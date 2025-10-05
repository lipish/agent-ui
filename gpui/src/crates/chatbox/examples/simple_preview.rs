use gpui::*;

struct SimpleDemo {
    message: String,
}

impl SimpleDemo {
    fn new(_cx: &mut App) -> Self {
        Self {
            message: "Chat interface ready - no messages yet".to_string(),
        }
    }
}

impl Render for SimpleDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .flex_col()
            .bg(gpui::white())
            .child(
                // Header
                div()
                    .w_full()
                    .px_4()
                    .py_3()
                    .bg(gpui::black())
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w_4()
                                    .h_4()
                                    .bg(gpui::blue())
                                    .rounded_full()
                            )
                            .child(
                                gpui::Label::new("Simple Chatbox Demo")
                                    .color(gpui::white())
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
                            .bg(gpui::rgb(0xf0f0f0))
                            .rounded_md()
                            .child(
                                gpui::Label::new(&self.message)
                                    .color(gpui::rgb(0x333333))
                                    .size(gpui::LabelSize::Small)
                            )
                    )
                    .child(
                        div()
                            .p_3()
                            .bg(gpui::rgb(0xf5f5f5))
                            .rounded_md()
                            .child(
                                gpui::Label::new("This shows the chatbox UI layout")
                                    .color(gpui::rgb(0x666666))
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
                    .border_color(gpui::rgb(0xdddddd))
                    .bg(gpui::white())
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .flex_1()
                                    .px_3()
                                    .py_2()
                                    .bg(gpui::white())
                                    .border_1()
                                    .border_color(gpui::rgb(0xcccccc))
                                    .rounded_md()
                                    .child(
                                        gpui::Label::new("Type your message here...")
                                            .color(gpui::rgb(0x999999))
                                            .size(gpui::LabelSize::Small)
                                    )
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(gpui::blue())
                                    .rounded_md()
                                    .child(
                                        gpui::Label::new("Send")
                                            .color(gpui::white())
                                            .size(gpui::LabelSize::Small)
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
                    .border_color(gpui::rgb(0xdddddd))
                    .bg(gpui::rgb(0xf8f9fa))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w_3()
                                    .h_3()
                                    .bg(gpui::rgb(0x6c757d))
                                    .rounded_full()
                            )
                            .child(
                                gpui::Label::new("This is a static UI preview")
                                    .color(gpui::rgb(0x6c757d))
                                    .size(gpui::LabelSize::XSmall)
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
                    origin: point(px(300.0), px(300.0)),
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
                let demo = cx.new(|_cx| SimpleDemo::new(_cx));
                demo.into()
            },
        );
    });
}