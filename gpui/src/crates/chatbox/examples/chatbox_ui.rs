use gpui::*;
use chatbox::Chatbox;

struct ChatboxExample {
    chatbox: Entity<Chatbox>,
}

impl ChatboxExample {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let chatbox = cx.new(|cx| {
            Chatbox::new(
                "Type your message here...",
                editor::EditorMode::AutoHeight {
                    min_lines: 1,
                    max_lines: Some(5),
                },
                window,
                cx,
            )
        });

        Self { chatbox }
    }
}

impl Render for ChatboxExample {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .flex_col()
            .bg(gpui::white())
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .child(
                        gpui::Label::new("Simple Chatbox Example")
                            .size(gpui::LabelSize::Large)
                            .color(gpui::Color::Default)
                    )
                    .child(
                        gpui::Label::new("This is a simplified chatbox UI component.")
                            .size(gpui::LabelSize::Small)
                            .color(gpui::Color::Muted)
                    )
            )
            .child(self.chatbox.clone())
    }
}

fn main() {
    gpui::App::new().run(|cx: &mut AppContext| {
        cx.activate(true);
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Chatbox Example".into()),
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(100.0), px(100.0)),
                    size: size(px(800.0), px(600.0)),
                })),
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                decorations: Some(Decorations::Server),
                is_movable: true,
                display_id: None,
                fullscreen: false,
            },
            |window, cx| {
                let example = cx.new(|cx| ChatboxExample::new(window, cx));
                example.into()
            },
        );
    });
}