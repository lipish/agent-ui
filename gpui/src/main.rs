use gpui::{
    actions, div, px, rgb, size, App, Application, Bounds, Context, FocusHandle,
    Focusable, InteractiveElement, IntoElement, KeyDownEvent, Entity,
    ParentElement, Render, Styled, Task, Timer, Window, WindowBounds, WindowOptions,
    prelude::*, KeyBinding, black, white,
};
use std::time::Duration;

// 光标闪烁器
pub struct CursorBlinker {
    blink_interval: Duration,
    blink_epoch: usize,
    visible: bool,
    enabled: bool,
    _blink_task: Option<Task<()>>,
}

impl CursorBlinker {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            blink_interval: Duration::from_millis(530), // 标准光标闪烁频率
            blink_epoch: 0,
            visible: true,
            enabled: false,
            _blink_task: None,
        }
    }

    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    pub fn enable(&mut self, cx: &mut Context<Self>) {
        if self.enabled {
            return;
        }

        self.enabled = true;
        self.visible = true;
        self.start_blinking(self.blink_epoch, cx);
    }

    pub fn disable(&mut self, cx: &mut Context<Self>) {
        self.visible = false;
        self.enabled = false;
        if let Some(task) = self._blink_task.take() {
            task.detach();
        }
        cx.notify();
    }

    pub fn show_cursor(&mut self, cx: &mut Context<Self>) {
        if !self.visible {
            self.visible = true;
            cx.notify();
        }
    }

    fn start_blinking(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if epoch == self.blink_epoch && self.enabled {
            self.visible = !self.visible;
            cx.notify();

            let epoch = self.next_blink_epoch();
            let interval = self.blink_interval;
            let task = cx.spawn(async move |this, cx| {
                Timer::after(interval).await;
                if let Some(this) = this.upgrade() {
                    this.update(cx, |this, cx| this.start_blinking(epoch, cx))
                        .ok();
                }
            });
            self._blink_task = Some(task);
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

// 定义对话框组件
struct Dialog {
    title: String,
    message: String,
    input_text: String,
    focus_handle: FocusHandle,
    cursor_position: usize,
    cursor_blinker: Entity<CursorBlinker>,
}

// 定义操作
actions!(dialog, [Confirm, Cancel]);

impl Dialog {
    fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        let cursor_blinker = cx.new(|cx| CursorBlinker::new(cx));
        
        Self {
            title: "对话框标题".into(),
            message: "这是一个消息对话框。您可以在这里放置任何您想要的内容。".into(),
            input_text: String::new(),
            focus_handle,
            cursor_blinker,
            cursor_position: 0,
        }
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut Context<Self>) {
        println!("用户点击了确认按钮");
        println!("输入内容: {}", self.input_text);
        cx.quit();
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut Context<Self>) {
        println!("用户点击了取消按钮");
        cx.quit();
    }


    
    fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        if event.keystroke.key == "backspace" {
            // 处理退格键
            if self.cursor_position > 0 && !self.input_text.is_empty() {
                self.input_text.remove(self.cursor_position - 1);
                self.cursor_position -= 1;
            }
        } else if event.keystroke.key == "left" {
            // 处理左箭头键
            if self.cursor_position > 0 {
                self.cursor_position -= 1;
            }
        } else if event.keystroke.key == "right" {
            // 处理右箭头键
            if self.cursor_position < self.input_text.len() {
                self.cursor_position += 1;
            }
        } else if event.keystroke.key.len() == 1 {
            // 处理字符输入
            self.input_text.insert_str(self.cursor_position, &event.keystroke.key);
            self.cursor_position += 1;
        }
        
        // 重置光标可见性
        self.cursor_blinker.update(cx, |blinker, cx| {
            blinker.show_cursor(cx);
        });
        cx.notify();
    }
}

impl Focusable for Dialog {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Dialog {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 启用或禁用光标闪烁
        if self.focus_handle.is_focused(window) {
            self.cursor_blinker.update(cx, |blinker, cx| {
                blinker.enable(cx);
            });
        } else {
            self.cursor_blinker.update(cx, |blinker, cx| {
                blinker.disable(cx);
            });
        }
        
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0xf5f5f5)) // 浅灰色背景
            .on_key_down(cx.listener(|this, event, _, cx| this.handle_key_down(event, cx)))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(400.)) // 固定宽度
                    .bg(white())
                    .border_1()
                    .border_color(rgb(0xdddddd))
                    .rounded_lg()
                    .shadow_md()
                    .child(
                        // 标题栏
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .px_4()
                            .py_3()
                            .bg(rgb(0x2196f3)) // 蓝色标题栏
                            .rounded_t_lg()
                            .child(
                                div() // 使用div代替h3
                                    .text_base()
                                    .text_color(white())
                                    .child(self.title.clone())
                            )
                    )
                    .child(
                        // 内容区域
                        div()
                            .flex()
                            .flex_col()
                            .px_4()
                            .py_3()
                            .gap_3()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(black())
                                    .child(self.message.clone())
                            )
                            .child(
                                // 输入框容器
                                div()
                                    .w_full()
                                    .relative()
                                    .child(
                                        div()
                                            .w_full()
                                            .px_2()
                                            .py_1()
                                            .border_1()
                                            .border_color(rgb(0xcccccc))
                                            .rounded_sm()
                                            .track_focus(&self.focus_handle)
                                            .child(self.input_text.clone())
                                    )
                                    .child(
                                        // 光标显示
                                        div()
                                            .absolute()
                                            .top(px(9.))
                                            .h(px(14.))
                                            .left(px(12.05 + (self.cursor_position as f32) * 7.5)) // 调整光标位置计算，微调与字符的间距
                                            .w(px(1.5)) // 保持光标宽度为1.5px
                                            .bg(rgb(0x999999)) // 将光标颜色从黑色改为浅灰色
                                            .when(self.focus_handle.is_focused(window) && self.cursor_blinker.read(cx).is_visible(), |this| {
                                                this.visible()
                                            })
                                            .when(!self.focus_handle.is_focused(window) || !self.cursor_blinker.read(cx).is_visible(), |this| {
                                                this.invisible()
                                            })
                                    )
                            )
                    )
                    .child(
                        // 按钮区域
                        div()
                            .flex()
                            .justify_end()
                            .px_4()
                            .py_3()
                            .gap_2()
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0x6c757d)) // 灰色取消按钮
                                    .text_color(white())
                                    .rounded_sm()
                                    .text_sm()
                                    .on_mouse_up(gpui::MouseButton::Left, cx.listener(|this, _, _, cx| this.cancel(&Cancel, cx)))
                                    .child("取消")
                            )
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .bg(rgb(0x2196f3)) // 蓝色确认按钮
                                    .text_color(white())
                                    .rounded_sm()
                                    .text_sm()
                                    .on_mouse_up(gpui::MouseButton::Left, cx.listener(|this, _, _, cx| this.confirm(&Confirm, cx)))
                                    .child("确认")
                            )
                    )
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        // 绑定键盘快捷键
        cx.bind_keys([
            KeyBinding::new("enter", Confirm, None),
            KeyBinding::new("escape", Cancel, None),
        ]);

        // 创建窗口
        let bounds = Bounds::centered(None, size(px(500.), px(500.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| Dialog::new(cx)).into()
            },
        )
        .unwrap();
        cx.activate(true);
    });
}