use editor::{
    Editor, EditorElement, EditorMode, EditorStyle,
    actions::Paste,
};
use gpui::{
    Context, Entity, EventEmitter, FocusHandle, Focusable, TextStyle, Window,
};
use language::{Buffer, Language};
use settings::Settings;
use std::sync::Arc;
use theme::ThemeSettings;
use ui::prelude::*;

pub struct MessageEditor {
    editor: Entity<Editor>,
}

#[derive(Clone, Copy, Debug)]
pub enum MessageEditorEvent {
    Send,
    Cancel,
    Focus,
    LostFocus,
}

impl EventEmitter<MessageEditorEvent> for MessageEditor {}

impl MessageEditor {
    pub fn new(
        placeholder: &str,
        mode: EditorMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let language = Language::new(
            language::LanguageConfig {
                completion_query_characters: collections::HashSet::from_iter(['.', '-', '_', '@']),
                ..Default::default()
            },
            None,
        );

        let buffer = cx.new(|cx| Buffer::local("", cx).with_language(Arc::new(language), cx));
        let buffer = cx.new(|cx| editor::MultiBuffer::singleton(buffer, cx));

        let editor = cx.new(|cx| {
            let mut editor = Editor::new(mode, buffer, None, window, cx);
            editor.set_placeholder_text(placeholder, window, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_soft_wrap();
            editor.set_use_modal_editing(true);
            editor
        });

        // Set up focus event listeners
        cx.on_focus_in(&editor.focus_handle(cx), window, |_, _, cx| {
            cx.emit(MessageEditorEvent::Focus)
        })
        .detach();
        cx.on_focus_out(&editor.focus_handle(cx), window, |_, _, _, cx| {
            cx.emit(MessageEditorEvent::LostFocus)
        })
        .detach();

        Self { editor }
    }

    pub fn is_empty(&self, cx: &App) -> bool {
        self.editor.read(cx).is_empty(cx)
    }

    pub fn clear(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
        });
    }

    pub fn text(&self, cx: &App) -> String {
        self.editor.read(cx).text(cx)
    }

    pub fn set_text(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.set_text(text, window, cx);
        });
    }

    pub fn send(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.is_empty(cx) {
            cx.emit(MessageEditorEvent::Send);
        }
    }

    fn cancel(&mut self, _: &editor::actions::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(MessageEditorEvent::Cancel);
    }

    fn paste(&mut self, _: &Paste, _: &mut Window, _cx: &mut Context<Self>) {
        // Simple paste handling - could be extended for images etc.
    }

    pub fn set_read_only(&mut self, read_only: bool, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.set_read_only(read_only);
            cx.notify()
        })
    }

    pub fn set_mode(&mut self, mode: EditorMode, cx: &mut Context<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.set_mode(mode);
            cx.notify()
        });
    }
}

impl Focusable for MessageEditor {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Render for MessageEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("MessageEditor")
            .on_action(cx.listener(Self::cancel))
            .capture_action(cx.listener(Self::paste))
            .flex_1()
            .child({
                let settings = ThemeSettings::get_global(cx);

                let text_style = TextStyle {
                    color: cx.theme().colors().text,
                    font_family: settings.buffer_font.family.clone(),
                    font_fallbacks: settings.buffer_font.fallbacks.clone(),
                    font_features: settings.buffer_font.features.clone(),
                    font_size: settings.buffer_font_size(cx).into(),
                    line_height: relative(settings.buffer_line_height.value()),
                    ..Default::default()
                };

                EditorElement::new(
                    &self.editor,
                    EditorStyle {
                        background: cx.theme().colors().editor_background,
                        local_player: cx.theme().players().local(),
                        text: text_style,
                        syntax: cx.theme().syntax().clone(),
                        inlay_hints_style: editor::make_inlay_hints_style(cx),
                        ..Default::default()
                    },
                )
            })
    }
}