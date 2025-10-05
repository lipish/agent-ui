use gpui::*;
use chatbox::ChatView;

struct SimpleDemo {
    chat_view: Entity<ChatView>,
}

impl SimpleDemo {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let chat_view = cx.new(|cx| ChatView::new(window, cx));
        Self { chat_view }
    }
}

impl Render for SimpleDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .flex()
            .bg(gpui::white())
            .child(self.chat_view.clone())
    }
}

fn main() {
    gpui::App::new().run(|cx: &mut AppContext| {
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
                decorations: Some(Decorations::Server),
                is_movable: true,
                display_id: None,
                fullscreen: false,
            },
            |window, cx| {
                let demo = cx.new(|cx| SimpleDemo::new(window, cx));
                demo.into()
            },
        );
    });
}