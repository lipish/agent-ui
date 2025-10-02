// Agent UI - Modular Design
// 模块化设计，遵循系统设计文档

use gpui::{prelude::*, *};
use agent_ui::{assets::Assets, AssistantPanel, utils};

fn main() {
    let app = Application::new();

    app.with_assets(Assets::from_project_root())
        .run(|cx| {
            cx.activate(true);

            // Window configuration
            let window_size = Size {
                width: px(600.),
                height: px(1067.),
            };

            let margin = px(20.);

            let bounds = if let Some(display) = cx.primary_display() {
                let display_bounds = display.bounds();
                let display_size = display_bounds.size;

                utils::calculate_window_bounds(display_size, window_size, margin)
            } else {
                // Fallback bounds
                Bounds {
                    origin: Point::new(px(100.), margin),
                    size: window_size,
                }
            };

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("AI Assistant".into()),
                        appears_transparent: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |_window, cx| cx.new(AssistantPanel::new),
            )
            .unwrap();
        });
}

