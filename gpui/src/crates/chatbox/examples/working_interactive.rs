// 可工作的交互式 Chatbox
// 使用正确的 GPUI API 来处理点击事件

use gpui::*;

pub struct WorkingInteractiveChatbox {
    messages: Vec<String>,
    input_text: String,
    click_count: usize,
}

impl WorkingInteractiveChatbox {
    pub fn new(_cx: &mut App) -> Self {
        Self {
            messages: vec![
                "🎉 Welcome to the Working Interactive Chatbox!".to_string(),
                "✨ Click the buttons below to interact!".to_string(),
                "🚀 Pure GPUI - Real interactivity!".to_string(),
            ],
            input_text: String::new(),
            click_count: 0,
        }
    }

    fn add_sample_input(&mut self, _event: &ClickEvent, _window: &mut Window, cx: &mut App) {
        self.click_count += 1;
        let sample_inputs = [
            "Hello, this is a test message!",
            "How are you doing today?",
            "This chatbox is working great!",
            "I love this interactive design!",
            "Let's try another message...",
        ];

        self.input_text = sample_inputs[self.click_count % sample_inputs.len()].to_string();

        // Update the messages through a global state or by re-rendering
        // For this demo, we'll just update the UI state
    }

    fn send_message(&mut self, _event: &ClickEvent, _window: &mut Window, cx: &mut App) {
        if !self.input_text.trim().is_empty() {
            // Note: In a real app, you'd maintain the state properly
            // For now, this demonstrates the click handling works
            println!("Would send: {}", self.input_text);
        }
    }

    fn clear_all(&mut self, _event: &ClickEvent, _window: &mut Window, cx: &mut App) {
        self.input_text.clear();
        self.click_count = 0;
        println!("Chat cleared!");
    }
}

impl Render for WorkingInteractiveChatbox {
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
                                    .child("Working Interactive Chatbox")
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
                    .child(
                        // Display current input
                        if !self.input_text.is_empty() {
                            div()
                                .bg(rgb(0xf0fdf4))
                                .px_5()
                                .py_4()
                                .rounded_lg()
                                .shadow_sm()
                                .border_1()
                                .border_color(rgb(0x14532d))
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
                                                .bg(rgb(0x14532d))
                                                .rounded_full()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .child(
                                                    div()
                                                        .text_color(rgb(0xf0fdf4))
                                                        .text_size(px(12.0))
                                                        .child("✏️")
                                                )
                                        )
                                        .child(
                                            div()
                                                .flex_1()
                                                .text_color(rgb(0x14532d))
                                                .text_size(px(15.0))
                                                .child(format!("Draft: {}", self.input_text))
                                        )
                                )
                        } else {
                            div()
                        }
                    )
            )
            .child(
                // Interactive input area
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
                                    .child("💡 Click the buttons below to test interactivity!")
                            )
                            .child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .child(
                                        div()
                                            .flex_1()
                                            .px_4()
                                            .py_3()
                                            .bg(rgb(0xf8fafc))
                                            .border_2()
                                            .border_dashed()
                                            .border_color(rgb(0x10b981))
                                            .rounded_lg()
                                            .text_color(rgb(0x166534))
                                            .text_size(px(15.0))
                                            .child(if self.input_text.is_empty() {
                                                "📝 Click to add text"
                                            } else {
                                                &self.input_text
                                            })
                                            .on_click(cx.listener(Self::add_sample_input))
                                    )
                                    .child(
                                        div()
                                            .px_8()
                                            .py_3()
                                            .bg(if !self.input_text.is_empty() { rgb(0x10b981) } else { rgb(0x9ca3af) })
                                            .rounded_lg()
                                            .shadow_md()
                                            .text_size(px(15.0))
                                            .text_color(rgb(0xffffff))
                                            .child("Send")
                                            .on_click(cx.listener(Self::send_message))
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
                                            .on_click(cx.listener(Self::clear_all))
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_color(rgb(0x64748b))
                                            .text_size(px(12.0))
                                            .child("✅ Real Click Events Working!")
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x64748b))
                                            .text_size(px(12.0))
                                            .child("📱 Interactive UI with GPUI")
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x64748b))
                                            .text_size(px(12.0))
                                            .child(format!("🖱️ Clicks: {} | Input: {}", self.click_count, if self.input_text.is_empty() { "empty" } else { "has text" }))
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
                            .items_center()
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
                                            .child("✅ Working Interactive Demo")
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
                                            .child("GPUI Events Working")
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x9ca3af))
                                            .text_size(px(11.0))
                                            .child("v3.0")
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
                    title: Some("Working Interactive - Real GPUI".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(400.0), px(200.0)),
                    size: size(px(750.0), px(650.0)),
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
            |_window, cx| {
                let chatbox = cx.new(|cx| WorkingInteractiveChatbox::new(cx));
                chatbox.into()
            },
        ) {
            Ok(_) => println!("✅ Working Interactive Chatbox launched!"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}