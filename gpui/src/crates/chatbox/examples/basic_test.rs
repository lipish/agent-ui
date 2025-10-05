// 基础测试版本 - 确定能工作的简单界面
// 专注于展示 UI，不处理输入

use gpui::*;

pub struct BasicTest {
    messages: Vec<String>,
    display_text: String,
}

impl BasicTest {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            messages: vec![
                "Welcome to Basic Test!".to_string(),
                "This should work!".to_string(),
            ],
            display_text: "Type something here...".to_string(),
        }
    }
}

impl Render for BasicTest {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf8fafc))
            .child(
                // Header
                div()
                    .px_4()
                    .py_3()
                    .bg(rgb(0x0f172a))
                    .child(
                        div()
                            .text_color(rgb(0xffffff))
                            .text_size(px(16.0))
                            .child("Basic Test")
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
                                        .text_color(rgb(0x1e40af))
                                        .text_size(px(14.0))
                                        .child(message.clone())
                                })
                            )
                    )
            )
            .child(
                // Input area (static display)
                div()
                    .p_4()
                    .border_t_1()
                    .border_color(rgb(0xe5e7eb))
                    .bg(rgb(0xffffff))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
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
                                            .text_color(rgb(0x374151))
                                            .text_size(px(14.0))
                                            .child(self.display_text.clone())
                                    )
                                    .child(
                                        div()
                                            .px_4()
                                            .py_2()
                                            .bg(rgb(0x3b82f6))
                                            .text_color(rgb(0xffffff))
                                            .text_size(px(14.0))
                                            .child("Send")
                                    )
                                    .child(
                                        div()
                                            .px_4()
                                            .py_2()
                                            .bg(rgb(0xef4444))
                                            .text_color(rgb(0xffffff))
                                            .text_size(px(14.0))
                                            .child("Clear")
                                    )
                            )
                            .child(
                                div()
                                    .text_color(rgb(0x6b7280))
                                    .text_size(px(12.0))
                                    .child("This is a static display - input field shows below")
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
                    .bg(rgb(0xf8fafc))
                    .child(
                        div()
                            .text_color(rgb(0x64748b))
                            .text_size(px(12.0))
                            .child("✅ Basic UI Working")
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
                    title: Some("Basic Test".into()),
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
                let basic_test = cx.new(|cx| BasicTest::new(cx));
                basic_test.into()
            },
        ) {
            Ok(_) => println!("✅ Basic Test launched!"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}