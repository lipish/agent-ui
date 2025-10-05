# Chatbox Component

A simplified UI chatbox component for Zed, inspired by the copilot interface but focused purely on input functionality.

## Features

- **Simple Input Box**: Clean text input with placeholder support
- **Event System**: Emits events for send, cancel, focus, and blur
- **Minimal Dependencies**: Uses only core UI components without backend complexity
- **Configurable**: Supports different editor modes and styling

## Usage

```rust
use gpui::*;
use chatbox::Chatbox;

struct MyChatView {
    chatbox: Entity<Chatbox>,
}

impl MyChatView {
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

impl Render for MyChatView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .flex_col()
            .child(self.chatbox.clone())
    }
}
```

## API

### Chatbox

The main component that combines a message editor and display area.

#### Methods

- `new(placeholder, mode, window, cx)` - Create a new chatbox instance
- `message_editor()` - Get access to the message editor
- `copilot_chat()` - Get access to the chat display
- `is_empty(cx)` - Check if the input is empty
- `clear(window, cx)` - Clear the input and display
- `text(cx)` - Get the current text content

### MessageEditor

The input component for typing messages.

#### Events

- `Send` - Emitted when a message should be sent
- `Cancel` - Emitted when input is cancelled
- `Focus` - Emitted when editor gains focus
- `LostFocus` - Emitted when editor loses focus

#### Methods

- `new(placeholder, mode, window, cx)` - Create a new message editor
- `is_empty(cx)` - Check if editor is empty
- `clear(window, cx)` - Clear editor content
- `text(cx)` - Get current text
- `set_text(text, window, cx)` - Set editor text
- `send(window, cx)` - Manually trigger send event
- `set_read_only(read_only, cx)` - Set read-only state
- `set_mode(mode, cx)` - Change editor mode

### CopilotChat

Simple display component for showing messages or status.

#### Methods

- `new()` - Create a new chat display
- `set_message(message)` - Set display message
- `clear()` - Clear display content

## Dependencies

This component intentionally has minimal dependencies:

- `gpui` - Core UI framework
- `ui` - UI components and styling
- `editor` - Text editing functionality
- `theme` - Theme support
- `settings` - Configuration
- `util` - Utility functions
- `language` - Language support
- `collections` - Basic data structures
- `serde` - Serialization support

## Example

Run the example to see the chatbox in action:

```bash
cargo run --example chatbox_ui
```

## Notes

This is a simplified version of a chat interface that focuses on providing a clean input box without backend functionality. It can be extended with:

- Message history
- Rich text formatting
- File/image attachments
- Real-time communication
- Custom styling
- Keyboard shortcuts
- Auto-completion

The component is designed to be lightweight and easily integrable into any Zed application.