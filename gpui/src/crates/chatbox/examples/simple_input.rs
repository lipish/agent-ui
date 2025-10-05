// 极简版本 - 只有一个 text input 测试输入功能
// 使用纯 GPUI，专注于输入测试

use gpui::*;

pub struct SimpleInputTest {
    input_text: String,
    messages: Vec<String>,
}

impl SimpleInputTest {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            input_text: String::new(),
            messages: vec!["Type something below!".to_string()],
        }
    }

    fn handle_key(&mut self, event: &gpui::KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        match event.keystroke.key.as_str() {
            "enter" => {
                if !self.input_text.trim().is_empty() {
                    self.messages.push(format!("You typed: {}", self.input_text));
                    self.input_text.clear();
                    cx.notify();
                }
            }
            "backspace" => {
                if !self.input_text.is_empty() {
                    self.input_text.pop();
                    cx.notify();
                }
            }
            "escape" => {
                self.input_text.clear();
                cx.notify();
            }
            _ => {
                // Handle character input
                if let Some(ch) = event.keystroke.key.chars().next() {
                    if ch.is_ascii() && !ch.is_control() {
                        self.input_text.push(ch);
                        cx.notify();
                    }
                }
            }
        }
    }
}

impl Render for SimpleInputTest {
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
                    .bg(rgb(0x0f172a))
                    .child(
                        div()
                            .text_color(rgb(0xffffff))
                            .text_size(px(16.0))
                            .child("Simple Input Test")
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
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .children(
                                self.messages.iter().map(|message| {
                                    div()
                                        .bg(rgb(0xe3f2fd))
                                        .px_3()
                                        .py_2()
                                        .round_md()
                                        .text_color(rgb(0x1e40af))
                                        .text_size(px(14.0))
                                        .child(message.clone())
                                })
                            )
                    )
            )
            .child(
                // Simple input area
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
                                    .px_3()
                                    .py_2()
                                    .bg(rgb(0xf8fafc))
                                    .border_1()
                                    .border_color(rgb(0xd1d5db))
                                    .round_md()
                                    .text_color(rgb(0x374151))
                                    .text_size(px(14.0))
                                    .child(if self.input_text.is_empty() {
                                        "Type here..."
                                    } else {
                                        &self.input_text
                                    })
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(rgb(0x3b82f6))
                                    .round_md()
                                    .text_color(rgb(0xffffff))
                                    .text_size(px(14.0))
                                    .child("Send")
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(rgb(0xef4444))
                                    .round_md()
                                    .text_color(rgb(0xffffff))
                                    .text_size(px(14.0))
                                    .child("Clear")
                            )
                    )
                    .child(
                        div()
                            .text_color(rgb(0x6b7280))
                            .text_size(px(12.0))
                            .child("Press Enter to send, Backspace to delete, Escape to clear")
                    )
            )
            .child(
                // Footer
                div()
                    .px_4()
                    .py_2()
                    .border_t_1()
                    .border_color(rgb(0xe5e7eb))
                    .bg(rgb(0xf8fafc))
                    .child(
                        div()
                            .text_color(rgb(0x64748b))
                            .text_size(px(12.0))
                            .child(format!("Current input: '{}' ({} chars)", self.input_text, self.input_text.len()))
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
                    title: Some("Simple Input Test".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(500.0), px(300.0)),
                    size: size(px(400.0), px(300.0)),
                })),
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                is_resizable: true,
                window_min_size: Some(size(px(300.0), px(200.0))),
                is_minimizable: true,
                window_background: WindowBackgroundAppearance::Transparent,
                app_id: None,
                display_id: None,
                tabbing_identifier: None,
                window_decorations: Some(WindowDecorations::Server),
            },
            |_window, cx| {
                let input_test = cx.new(|cx| SimpleInputTest::new(cx));
                input_test.into()
            },
        ) {
            Ok(_) => println!("✅ Simple Input Test launched!"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}