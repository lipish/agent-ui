use gpui::{div, rgb, IntoElement, Render, Styled, ParentElement, Window, Context};

pub struct Workspace;

impl Workspace {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e7d32))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xffffff))
            .child("Hello from Workspace!")
    }
}