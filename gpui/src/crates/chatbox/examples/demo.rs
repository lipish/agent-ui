use gpui::*;
use ui::prelude::*;
use theme::ActiveTheme;

struct DemoApp {
    input_text: String,
    message: String,
}

impl DemoApp {
    fn new(_cx: &mut App) -> Self {
        Self {
            input_text: "".to_string(),
            message: "Chat interface ready - no messages yet".to_string(),
        }
    }
}

impl Render for DemoApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .flex_col()
            .bg(cx.theme().colors().panel_background)
            .child(
                // Header
                div()
                    .p_4()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new(IconName::Chat)
                                    .size(IconSize::Medium)
                                    .color(Color::Accent)
                            )
                            .child(
                                Label::new("Simple Chatbox Demo")
                                    .size(LabelSize::Large)
                            )
                    )
                    .child(
                        div()
                            .mt_2()
                            .child(
                                Label::new("This is a simplified chatbox UI component.")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                    )
            )
            .child(
                // Chat area
                div()
                    .flex_1()
                    .p_4()
                    .child(
                        if self.message.is_empty() {
                            Label::new("Chat interface ready - no messages yet")
                                .color(Color::Muted)
                        } else {
                            Label::new(&self.message)
                        }
                    )
            )
            .child(
                // Input area
                div()
                    .p_3()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        div()
                            .flex()
                            .gap_2()
                            .child(
                                div()
                                    .flex_1()
                                    .px_3()
                                    .py_2()
                                    .bg(cx.theme().colors().editor_background)
                                    .border_1()
                                    .border_color(cx.theme().colors().border)
                                    .rounded_md()
                                    .child(
                                        if self.input_text.is_empty() {
                                            Label::new("Type your message here...")
                                                .color(Color::Muted)
                                                .size(LabelSize::Small)
                                        } else {
                                            Label::new(&self.input_text)
                                        }
                                    )
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_2()
                                    .bg(gpui::blue())
                                    .rounded_md()
                                    .child(
                                        Label::new("Send")
                                            .color(Color::Default)
                                            .size(LabelSize::Small)
                                    )
                            )
                    )
            )
            .child(
                // Footer
                div()
                    .p_2()
                    .border_t_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new(IconName::Info)
                                    .size(IconSize::XSmall)
                                    .color(Color::Muted)
                            )
                            .child(
                                Label::new("This is a static demo - no interactive functionality")
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted)
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Chatbox UI Demo".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(200.0), px(200.0)),
                    size: size(px(600.0), px(500.0)),
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
                let demo = cx.new(|_cx| DemoApp::new(_cx));
                demo.into()
            },
        );
    });
}