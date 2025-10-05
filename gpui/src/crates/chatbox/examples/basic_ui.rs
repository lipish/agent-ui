use gpui::*;

pub struct BasicChatDemo {
    input_text: String,
    messages: Vec<String>,
}

impl BasicChatDemo {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            input_text: String::new(),
            messages: vec![
                "Welcome to the Chatbox UI Demo!".to_string(),
                "This shows the layout and visual design.".to_string(),
            ],
        }
    }
}

impl Render for BasicChatDemo {
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
                    .bg(rgb(0x6366f1))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .w_6()
                                    .h_6()
                                    .bg(rgb(0xffffff))
                                    .rounded_full()
                                    .border_1()
                                    .border_color(rgb(0xe5e7eb))
                            )
                            .child(
                                div()
                                    .text_color(rgb(0xffffff))
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::Normal)
                                    .child("Chatbox UI Demo")
                            )
                    )
            )
            .child(
                // Messages area
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .p_4()
                    .gap_3()
                    .child(
                        self.messages.iter().enumerate().map(|(i, message)| {
                            div()
                                .flex()
                                .gap_2()
                                .mb_2()
                                .child(
                                    div()
                                        .text_color(rgb(0x6366f1))
                                        .text_size(px(14.0))
                                        .child(format!("Message {}:", i + 1))
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .bg(rgb(0xffffff))
                                        .px_3()
                                        .py_2()
                                        .rounded_md()
                                        .border_1()
                                        .border_color(rgb(0xe5e7eb))
                                        .text_color(rgb(0x374151))
                                        .text_size(px(14.0))
                                        .child(message.clone())
                                )
                        })
                        .collect::<Vec<_>>()
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
                                            .child(if self.input_text.is_empty() {
                                                "Type your message here..."
                                            } else {
                                                &self.input_text
                                            })
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
                                            .font_weight(FontWeight::Normal)
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
                                    .w_3()
                                    .h_3()
                                    .bg(rgb(0x9ca3af))
                                    .rounded_full()
                            )
                            .child(
                                div()
                                    .text_color(rgb(0x6b7280))
                                    .text_size(px(12.0))
                                    .child("Static UI preview - demonstrates layout and design")
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);

        // Try to open the window with error handling
        match cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Chatbox UI Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(300.0), px(200.0)),
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
                let demo = cx.new(|cx| BasicChatDemo::new(cx));
                demo.into()
            },
        ) {
            Ok(_) => println!("Window opened successfully!"),
            Err(e) => eprintln!("Failed to open window: {:?}", e),
        }
    });
}