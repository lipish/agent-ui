// Final Interactive Chatbox Component
// A complete, working text input component with blinking cursor functionality
// Based on GPUI's official input example and Zed's cursor blinking implementation

use std::ops::Range;
use std::time::Duration;

use gpui::{
    actions, div, px, rgb, size, point, App, Application, Bounds, ClipboardItem, Context,
    CursorStyle, ElementId, ElementInputHandler, Entity, EntityInputHandler, FocusHandle,
    Focusable, GlobalElementId, KeyBinding, LayoutId, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, PaintQuad, Pixels, Point, ShapedLine, SharedString,
    Style, TextRun, UTF16Selection, UnderlineStyle, Window, WindowBounds, WindowOptions,
    WindowKind, WindowBackgroundAppearance, WindowDecorations,
    prelude::*, fill, hsla, relative, blue, FontWeight, Task, Timer,
};

// Actions for text input
actions!(
    chat_input,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        ShowCharacterPalette,
        Paste,
        Cut,
        Copy,
    ]
);

// Cursor blinking manager - based on Zed's BlinkManager
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
            blink_interval: Duration::from_millis(530), // Standard cursor blink rate
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

    pub fn pause_blinking(&mut self, cx: &mut Context<Self>) {
        self.show_cursor(cx);

        if self.enabled {
            let epoch = self.next_blink_epoch();
            let interval = self.blink_interval;
            let task = cx.spawn(async move |this, cx| {
                cx.background_spawn(async move {
                    Timer::after(interval).await;
                }).await;
                this.update(cx, |this, cx| this.resume_cursor_blinking(epoch, cx))
                    .ok();
            });
            self._blink_task = Some(task);
        }
    }

    fn resume_cursor_blinking(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if epoch == self.blink_epoch && self.enabled {
            self.start_blinking(epoch, cx);
        }
    }

    fn start_blinking(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if epoch == self.blink_epoch && self.enabled {
            self.visible = !self.visible;
            cx.notify();

            let epoch = self.next_blink_epoch();
            let interval = self.blink_interval;
            let task = cx.spawn(async move |this, cx| {
                cx.background_spawn(async move {
                    Timer::after(interval).await;
                }).await;
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

// Main chat input component
pub struct InteractiveChatInput {
    focus_handle: FocusHandle,
    content: SharedString,
    placeholder: SharedString,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    last_layout: Option<ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    is_selecting: bool,
    cursor_blinker: Entity<CursorBlinker>,
}

impl InteractiveChatInput {
    pub fn new(cx: &mut App) -> Self {
        let cursor_blinker = cx.new(|cx| CursorBlinker::new(cx));

        Self {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: "Type your message...".into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
            cursor_blinker,
        }
    }

    // Navigation methods
    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx)
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.content.len(), cx);
    }

    // Editing methods
    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    // Mouse methods
    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_selecting = true;

        if event.modifiers.shift {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        } else {
            self.move_to(self.index_for_mouse_position(event.position), cx)
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    // Clipboard methods
    fn show_character_palette(
        &mut self,
        _: &ShowCharacterPalette,
        window: &mut Window,
        _: &mut Context<Self>,
    ) {
        window.show_character_palette();
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.replace_text_in_range(None, &text.replace("\n", " "), window, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx)
        }
    }

    // Helper methods
    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        self.cursor_blinker.update(cx, |blinker, cx| blinker.pause_blinking(cx));
        cx.notify()
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }

        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return 0;
        };
        if position.y < bounds.top() {
            return 0;
        }
        if position.y > bounds.bottom() {
            return self.content.len();
        }
        line.closest_index_for_x(position.x - bounds.left())
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        self.cursor_blinker.update(cx, |blinker, cx| blinker.pause_blinking(cx));
        cx.notify()
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;

        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }

        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;

        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }

        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        let mut prev_offset = 0;
        for (i, _) in self.content.char_indices() {
            if i >= offset {
                break;
            }
            prev_offset = i;
        }
        prev_offset
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content
            .char_indices()
            .find(|(i, _)| *i > offset)
            .map(|(i, _)| i)
            .unwrap_or(self.content.len())
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.content = "".into();
        self.selected_range = 0..0;
        self.selection_reversed = false;
        self.marked_range = None;
        self.cursor_blinker.update(cx, |blinker, cx| blinker.pause_blinking(cx));
        cx.notify();
    }

    fn get_text(&self) -> String {
        self.content.to_string()
    }

    pub fn set_text(&mut self, text: String, cx: &mut Context<Self>) {
        let len = text.len();
        self.content = text.into();
        self.selected_range = len..len;
        self.cursor_blinker.update(cx, |blinker, cx| blinker.pause_blinking(cx));
        cx.notify();
    }
}

// Implement EntityInputHandler for proper text input integration
impl EntityInputHandler for InteractiveChatInput {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        self.marked_range.take();
        self.cursor_blinker.update(cx, |blinker, cx| blinker.pause_blinking(cx));
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        if !new_text.is_empty() {
            self.marked_range = Some(range.start..range.start + new_text.len());
        } else {
            self.marked_range = None;
        }
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());

        self.cursor_blinker.update(cx, |blinker, cx| blinker.pause_blinking(cx));
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let last_layout = self.last_layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);
        Some(Bounds::from_corners(
            point(
                bounds.left() + last_layout.x_for_index(range.start),
                bounds.top(),
            ),
            point(
                bounds.left() + last_layout.x_for_index(range.end),
                bounds.bottom(),
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let line_point = self.last_bounds?.localize(&point)?;
        let last_layout = self.last_layout.as_ref()?;

        let utf8_index = last_layout.index_for_x(point.x - line_point.x)?;
        Some(self.offset_to_utf16(utf8_index))
    }
}

// Text element for rendering the input
struct ChatInputTextElement {
    input: Entity<InteractiveChatInput>,
}

struct PrepaintState {
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
}

impl IntoElement for ChatInputTextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ChatInputTextElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.input.read(cx);
        let content = input.content.clone();
        let selected_range = input.selected_range.clone();
        let cursor = input.cursor_offset();
        let style = window.text_style();

        let (display_text, text_color) = if content.is_empty() {
            (input.placeholder.clone(), hsla(0., 0., 0., 0.3))
        } else {
            (content, style.color)
        };

        let run = TextRun {
            len: display_text.len(),
            font: style.font(),
            color: text_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let runs = if let Some(marked_range) = input.marked_range.as_ref() {
            vec![
                TextRun {
                    len: marked_range.start,
                    ..run.clone()
                },
                TextRun {
                    len: marked_range.end - marked_range.start,
                    underline: Some(UnderlineStyle {
                        color: Some(run.color),
                        thickness: px(1.0),
                        wavy: false,
                    }),
                    ..run.clone()
                },
                TextRun {
                    len: display_text.len() - marked_range.end,
                    ..run
                },
            ]
            .into_iter()
            .filter(|run| run.len > 0)
            .collect()
        } else {
            vec![run]
        };

        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs, None);

        let cursor_pos = line.x_for_index(cursor);
        let (selection, cursor) = if selected_range.is_empty() {
            (
                None,
                Some(fill(
                    Bounds::new(
                        point(bounds.left() + cursor_pos, bounds.top()),
                        size(px(2.), bounds.bottom() - bounds.top()),
                    ),
                    blue(),
                )),
            )
        } else {
            (
                Some(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + line.x_for_index(selected_range.start),
                            bounds.top(),
                        ),
                        point(
                            bounds.left() + line.x_for_index(selected_range.end),
                            bounds.bottom(),
                        ),
                    ),
                    blue().opacity(0.3),
                )),
                None,
            )
        };
        PrepaintState {
            line: Some(line),
            cursor,
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );
        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection)
        }
        let line = prepaint.line.take().unwrap();
        line.paint(bounds.origin, window.line_height(), window, cx)
            .unwrap();

        if focus_handle.is_focused(window) {
            let cursor_visible = self.input.read(cx).cursor_blinker.read(cx).is_visible();
            if let Some(cursor) = prepaint.cursor.take() {
                if cursor_visible {
                    window.paint_quad(cursor);
                }
            }
        }

        self.input.update(cx, |input, _cx| {
            input.last_layout = Some(line);
            input.last_bounds = Some(bounds);
        });
    }
}

impl Render for InteractiveChatInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Enable cursor blinking when focused
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
            .key_context("InteractiveChatInput")
            .track_focus(&self.focus_handle(cx))
            .cursor(CursorStyle::IBeam)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::home))
            .on_action(cx.listener(Self::end))
            .on_action(cx.listener(Self::show_character_palette))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::copy))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .bg(rgb(0xf8fafc))
            .border_1()
            .border_color(rgb(0xd1d5db))
            .rounded_md()
            .line_height(px(28.))
            .text_size(px(16.))
            .px_3()
            .py_2()
            .child(ChatInputTextElement { input: cx.entity() })
    }
}

impl Focusable for InteractiveChatInput {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// Main chatbox component with messages
pub struct InteractiveChatbox {
    messages: Vec<String>,
    chat_input: Entity<InteractiveChatInput>,
    focus_handle: FocusHandle,
}

impl InteractiveChatbox {
    pub fn new(cx: &mut App) -> Self {
        let chat_input = cx.new(|cx| InteractiveChatInput::new(cx));

        Self {
            messages: vec![
                "🎉 Welcome to the Interactive Chatbox!".to_string(),
                "✨ This input field has full text editing functionality!".to_string(),
                "💪 Try typing, selecting, copying, and pasting!".to_string(),
                "⌨️ Use arrow keys, Home/End, Ctrl+A to select all!".to_string(),
                "✨ The cursor blinks naturally when focused!".to_string(),
            ],
            chat_input,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn add_message(&mut self, message: &str, cx: &mut Context<Self>) {
        self.messages.push(message.to_string());
        cx.notify();
    }

    pub fn get_input_text(&self, cx: &App) -> String {
        self.chat_input.read(cx).get_text()
    }

    pub fn clear_input(&mut self, cx: &mut Context<Self>) {
        self.chat_input.update(cx, |input, cx| {
            input.clear(cx);
        });
    }

    pub fn set_input_text(&mut self, text: String, cx: &mut Context<Self>) {
        self.chat_input.update(cx, |input, cx| {
            input.set_text(text, cx);
        });
    }

    pub fn clear_messages(&mut self, cx: &mut Context<Self>) {
        self.messages.clear();
        cx.notify();
    }

    pub fn focus_input(&self, window: &mut Window, cx: &App) {
        let focus_handle = self.chat_input.read(cx).focus_handle.clone();
        window.focus(&focus_handle);
    }
}

impl Render for InteractiveChatbox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xf8fafc))
            .track_focus(&self.focus_handle(cx))
            .child(
                // Header
                div()
                    .px_4()
                    .py_3()
                    .bg(rgb(0x1f2937))
                    .child(
                        div()
                            .text_color(rgb(0xffffff))
                            .text_size(px(18.0))
                            .font_weight(FontWeight::BOLD)
                            .child("✅ Interactive Chatbox - Complete Text Input")
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
                                self.messages.iter().enumerate().map(|(i, message)| {
                                    div()
                                        .bg(rgb(0xe3f2fd))
                                        .px_4()
                                        .py_3()
                                        .rounded_md()
                                        .border_1()
                                        .border_color(rgb(0x2196f3))
                                        .child(
                                            div()
                                                .flex()
                                                .items_start()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .text_color(rgb(0x1976d2))
                                                        .text_size(px(14.0))
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .child(format!("{}:", i + 1))
                                                )
                                                .child(
                                                    div()
                                                        .flex_1()
                                                        .text_color(rgb(0x0d47a1))
                                                        .text_size(px(15.0))
                                                        .child(message.clone())
                                                )
                                        )
                                })
                            )
                    )
            )
            .child(
                // Input area
                div()
                    .p_4()
                    .border_t_2()
                    .border_color(rgb(0xe5e7eb))
                    .bg(rgb(0xffffff))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
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
                                    .child("💡 Full-featured text input with blinking cursor!")
                            )
                            .child(self.chat_input.clone())
                            .child(
                                div()
                                    .text_color(rgb(0x6b7280))
                                    .text_size(px(12.0))
                                    .child("🎯 Features: Arrow keys • Home/End • Ctrl+A (select all) • Copy/Paste • Blinking cursor • Text selection")
                            )
                    )
            )
            .child(
                // Footer
                div()
                    .px_4()
                    .py_3()
                    .border_t_1()
                    .border_color(rgb(0xe5e7eb))
                    .bg(rgb(0xf9fafb))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_between()
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        div()
                                            .w_3()
                                            .h_3()
                                            .bg(rgb(0x10b981))
                                            .rounded_full()
                                    )
                                    .child(
                                        div()
                                            .text_color(rgb(0x374151))
                                            .text_size(px(13.0))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("✅ Complete Working Text Input")
                                    )
                            )
                            .child(
                                div()
                                    .text_color(rgb(0x6b7280))
                                    .text_size(px(11.0))
                                    .child(format!("{} messages", self.messages.len()))
                            )
                    )
            )
    }
}

impl Focusable for InteractiveChatbox {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// Function to set up key bindings and launch the chatbox
pub fn launch_interactive_chatbox() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);

        // Set up key bindings for text input
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),
            KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),
            KeyBinding::new("right", Right, None),
            KeyBinding::new("shift-left", SelectLeft, None),
            KeyBinding::new("shift-right", SelectRight, None),
            KeyBinding::new("cmd-a", SelectAll, None),
            KeyBinding::new("cmd-v", Paste, None),
            KeyBinding::new("cmd-c", Copy, None),
            KeyBinding::new("cmd-x", Cut, None),
            KeyBinding::new("home", Home, None),
            KeyBinding::new("end", End, None),
            KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, None),
        ]);

        match cx.open_window(
            WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("Interactive Chatbox - Complete Text Input".into()),
                    appears_transparent: false,
                    traffic_light_position: Some(point(px(12.0), px(20.0))),
                }),
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(400.0), px(200.0)),
                    size: size(px(700.0), px(650.0)),
                })),
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                is_resizable: true,
                window_min_size: Some(size(px(500.0), px(400.0))),
                is_minimizable: true,
                window_background: WindowBackgroundAppearance::Transparent,
                app_id: None,
                display_id: None,
                tabbing_identifier: None,
                window_decorations: Some(WindowDecorations::Server),
            },
            |_window, cx| {
                let chatbox = cx.new(|cx| InteractiveChatbox::new(cx));
                chatbox.into()
            },
        ) {
            Ok(_) => println!("✅✅✅ INTERACTIVE CHATBOX WITH BLINKING CURSOR LAUNCHED! ✅✅✅"),
            Err(e) => eprintln!("❌ Failed to open window: {:?}", e),
        }
    });
}