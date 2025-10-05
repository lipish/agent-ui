use gpui::*;

pub struct InteractiveChatDemo {
    input_text: String,
    messages: Vec<String>,
    editor: Entity<Editor>,
}

impl InteractiveChatDemo {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        // Create a simple editor for input
        let buffer = cx.new(|cx| Buffer::local("", cx));
        let editor = cx.new(|cx| Editor::new(editor::EditorMode::SingleLine, buffer, None, window, cx));

        Self {
            input_text: String::new(),
            messages: vec![
                "Welcome to the Interactive Chatbox!".to_string(),
                "Try typing in the input field below and click Send.".to_string(),
            ],
            editor,
        }
    }

    fn send_message(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.editor.read(cx).text(cx);
        if !text.trim().is_empty() {
            self.messages.push(format!("You: {}", text));
            self.editor.update(cx, |editor, cx| {
                editor.clear(window, cx);
            });
            cx.notify();
        }
    }

    fn clear_chat(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.messages.clear();
        self.messages.push("Chat cleared.".to_string());
        cx.notify();
    }
}

impl Render for InteractiveChatDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                            .text_color(rgb(0xffffff))
                            .text_size(px(18.0))
                            .child("💬 Interactive Chatbox")
                    )
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0x4f46e5))
                                    .rounded_md()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0xffffff))
                                    .child("Clear")
                                    .when(cx.background_executor().read(cx).is_some(), |this| {
                                        this.on_click(cx.listener(Self::clear_chat))
                                    })
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
                    .gap_2()
                    .overflow_y_scroll()
                    .child(
                        if self.messages.is_empty() {
                            div()
                                .flex_1()
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .text_color(rgb(0x9ca3af))
                                        .text_size(px(14.0))
                                        .child("No messages yet. Start typing below!")
                                )
                        } else {
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .children(
                                    self.messages.iter().enumerate().map(|(i, message)| {
                                        let bg_color = if i % 2 == 0 { rgb(0xe3f2fd) } else { rgb(0xf3e5f5) };
                                        let text_color = if i % 2 == 0 { rgb(0x1565c0) } else { rgb(0x6a1b9a) };

                                        div()
                                            .bg(bg_color)
                                            .px_4()
                                            .py_3()
                                            .rounded_md()
                                            .border_1()
                                            .border_color(bg_color)
                                            .child(
                                                div()
                                                    .text_color(text_color)
                                                    .text_size(px(14.0))
                                                    .child(message.clone())
                                            )
                                    })
                                )
                        }
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
                                    .child(self.editor.clone())
                            )
                            .child(
                                div()
                                    .px_6()
                                    .py_3()
                                    .bg(rgb(0x6366f1))
                                    .rounded_md()
                                    .border_1()
                                    .border_color(rgb(0x4f46e5))
                                    .text_size(px(14.0))
                                    .text_color(rgb(0xffffff))
                                    .child("Send")
                                    .on_click(cx.listener(Self::send_message))
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
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_color(rgb(0x6b7280))
                                            .text_size(px(12.0))
                                            .child("💡")
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x6b7280))
                                            .text_size(px(12.0))
                                            .child("Type a message and click Send or press Enter")
                                    )
                            )
                            .child(
                                div()
                                    .text_color(rgb(0x9ca3af))
                                    .text_size(px(11.0))
                                    .child(format!("Messages: {}", self.messages.len()))
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
                    title: Some("Interactive Chatbox Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(350.0), px(200.0)),
                    size: size(px(650.0), px(500.0)),
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
            |window, cx| {
                let demo = cx.new(|cx| InteractiveChatDemo::new(window, cx));
                demo.into()
            },
        ) {
            Ok(_) => println!("✅ Interactive Chatbox demo launched successfully!"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}