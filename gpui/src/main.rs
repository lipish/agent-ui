use gpui::*;
use agent_ui::components::AssistantPanel;
use agent_ui::keybindings;
use agent_ui::theme::Theme;

fn main() {
    Application::new().run(|cx: &mut App| {
        // 初始化主题
        cx.set_global(Theme::default());

        // 注册键盘绑定
        keybindings::register_keybindings(cx);

        // 获取屏幕尺寸并计算右侧位置
        let display = cx.displays().first().cloned().unwrap();
        let screen_width = display.bounds().size.width;
        let window_width = px(450.0);
        let window_height = px(800.0);

        // 窗口放在屏幕最右侧，留一点边距
        let x_position = screen_width - window_width - px(20.0);
        let y_position = px(100.0);

        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("Agent".into()),
                    appears_transparent: false,
                    ..Default::default()
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point::new(x_position, y_position),
                    size: Size {
                        width: window_width,
                        height: window_height,
                    },
                })),
                ..Default::default()
            },
            |window, cx| {
                let panel = cx.new(|cx| AssistantPanel::new(cx));
                // 设置初始焦点到输入框
                let focus_handle = panel.read(cx).focus_handle(cx);
                window.focus(&focus_handle);
                panel
            },
        ).unwrap();
    });
}

