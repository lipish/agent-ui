// 可交互的 Chatbox UI 演示 - 添加真正的输入功能
// 基于成功的 showcase 版本，增加交互能力

use gpui::*;

pub struct InteractiveShowcase {
    messages: Vec<String>,
    input_text: String,
    editor: Entity<Editor>,
}

impl InteractiveShowcase {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        // 创建一个真正的编辑器用于输入
        let buffer = cx.new(|cx| Buffer::local("", cx));
        let editor = cx.new(|cx| {
            let mut editor = Editor::new(editor::EditorMode::SingleLine, buffer, None, window, cx);
            editor.set_placeholder_text("Type your message here...", window, cx);
            editor
        });

        Self {
            messages: vec![
                "🎉 Welcome to the Interactive Chatbox!".to_string(),
                "✨ You can now type in the input field below!".to_string(),
                "🚀 Try typing something and clicking Send!".to_string(),
            ],
            input_text: String::new(),
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

    fn clear_all(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.messages.clear();
        self.messages.push("🧹 Chat cleared! Ready for new messages.".to_string());
        self.editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
        });
        cx.notify();
    }
}

impl Render for InteractiveShowcase {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf8fafc))
            .child(
                // Header
                div()
                    .flex()
                    .items_center()
                    .px_4()
                    .py_3()
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
                                    .child("Interactive Chatbox")
                            )
                    )
            )
            .child(
                // Messages area
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
                            .gap_3()
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
                                        .px_5()
                                        .py_4()
                                        .rounded_lg()
                                        .shadow_sm()
                                        .border_1()
                                        .border_color(bg_color)
                                        .child(
                                            div()
                                                .flex()
                                                .items_start()
                                                .gap_3()
                                                .child(
                                                    div()
                                                        .w_8()
                                                        .h_8()
                                                        .min_w_8()
                                                        .min_h_8()
                                                        .bg(text_color)
                                                        .rounded_full()
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .child(
                                                            div()
                                                                .text_color(bg_color)
                                                                .text_size(px(12.0))
                                                                .child(format!("{}", i + 1))
                                                        )
                                                )
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .text_color(text_color)
                                                        .text_size(px(15.0))
                                                        .child(message.clone())
                                                )
                                        )
                                })
                            )
                    )
            )
            .child(
                // Input area with real editor
                div()
                    .p_5()
                    .border_t_2()
                    .border_color(rgb(0xe2e8f0))
                    .bg(rgb(0xffffff))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(rgb(0xf0f9ff))
                                    .border_1()
                                    .border_color(rgb(0x7dd3fc))
                                    .rounded_md()
                                    .text_color(rgb(0x075985))
                                    .text_size(px(13.0))
                                    .child("💡 Type your message below and press Enter or click Send")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .child(
                                        div()
                                            .flex_1()
                                            .px_3()
                                            .py_2()
                                            .bg(rgb(0xf8fafc))
                                            .border_2()
                                            .border_color(rgb(0x10b981))
                                            .rounded_lg()
                                            .child(self.editor.clone())
                                    )
                                    .child(
                                        div()
                                            .px_8()
                                            .py_3()
                                            .bg(rgb(0x10b981))
                                            .rounded_lg()
                                            .shadow_md()
                                            .text_size(px(15.0))
                                            .text_color(rgb(0xffffff))
                                            .child("Send")
                                            .when(cx.background_executor().read(cx).is_some(), |this| {
                                                this.on_click(cx.listener(Self::send_message))
                                            })
                                    )
                                    .child(
                                        div()
                                            .px_8()
                                            .py_3()
                                            .bg(rgb(0xef4444))
                                            .rounded_lg()
                                            .shadow_md()
                                            .text_size(px(15.0))
                                            .text_color(rgb(0xffffff))
                                            .child("Clear")
                                            .when(cx.background_executor().read(cx).is_some(), |this| {
                                                this.on_click(cx.listener(Self::clear_all))
                                            })
                                    )
                            )
                    )
            )
            .child(
                // Footer
                div()
                    .px_5()
                    .py_4()
                    .border_t_1()
                    .border_color(rgb(0xe2e8f0))
                    .bg(rgb(0xf8fafc))
                    .child(
                        div()
                            .flex()
                            .items-center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
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
                                            .text_size(px(13.0))
                                            .child("✨ Interactive Demo - Real Input Enabled")
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
                                            .text_size(px(12.0))
                                            .child(format!("{} messages", self.messages.len()))
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x9ca3af))
                                            .text_size(px(11.0))
                                            .child("v2.0 - Interactive")
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
                    title: Some("Interactive Chatbox - Real Input".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(400.0), px(200.0)),
                    size: size(px(750.0), px(600.0)),
                })),
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                is_resizable: true,
                window_min_size: Some(size(px(600.0), px(500.0))),
                is_minimizable: true,
                window_background: WindowBackgroundAppearance::Transparent,
                app_id: None,
                display_id: None,
                tabbing_identifier: None,
                window_decorations: Some(WindowDecorations::Server),
            },
            |window, cx| {
                let showcase = cx.new(|cx| InteractiveShowcase::new(window, cx));
                showcase.into()
            },
        ) {
            Ok(_) => println!("✅ Interactive Chatbox launched successfully!"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}