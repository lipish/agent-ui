// Keyboard bindings for the application

use gpui::KeyBinding;

// Import actions from components
use crate::components::text_input;

/// Register all keyboard bindings
pub fn register_keybindings(cx: &mut gpui::App) {
    // TextInput bindings
    cx.bind_keys([
        // Navigation
        KeyBinding::new("left", text_input::Left, Some("TextInput")),
        KeyBinding::new("right", text_input::Right, Some("TextInput")),
        KeyBinding::new("home", text_input::Home, Some("TextInput")),
        KeyBinding::new("end", text_input::End, Some("TextInput")),
        KeyBinding::new("cmd-left", text_input::Home, Some("TextInput")),
        KeyBinding::new("cmd-right", text_input::End, Some("TextInput")),
        
        // Selection
        KeyBinding::new("shift-left", text_input::SelectLeft, Some("TextInput")),
        KeyBinding::new("shift-right", text_input::SelectRight, Some("TextInput")),
        KeyBinding::new("cmd-a", text_input::SelectAll, Some("TextInput")),
        
        // Editing
        KeyBinding::new("backspace", text_input::Backspace, Some("TextInput")),
        KeyBinding::new("delete", text_input::Delete, Some("TextInput")),
        
        // Clipboard
        KeyBinding::new("cmd-c", text_input::Copy, Some("TextInput")),
        KeyBinding::new("cmd-v", text_input::Paste, Some("TextInput")),
        KeyBinding::new("cmd-x", text_input::Cut, Some("TextInput")),
    ]);
}

