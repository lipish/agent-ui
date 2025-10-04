// Theme configuration - Yuanbao style

use gpui::{hsla, px, Hsla, TextStyle};

/// Theme colors for the UI
#[derive(Debug, Clone)]
pub struct Theme {
    // Background colors
    pub background: Hsla,
    pub card: Hsla,

    // Text colors
    pub foreground: Hsla,
    pub muted_foreground: Hsla,

    // Border colors
    pub border: Hsla,

    // Primary colors (for buttons, etc.)
    pub primary: Hsla,
    pub primary_foreground: Hsla,

    // Muted color (for disabled states)
    pub muted: Hsla,

    // TextInput style
    pub text_input: TextInputTheme,
}

#[derive(Debug, Clone)]
pub struct TextInputTheme {
    pub background: Hsla,
    pub border_color: Hsla,
    pub border_color_focused: Hsla,
    pub text: TextStyle,
    pub placeholder_color: Hsla,
    pub selection_color: Hsla,
    pub cursor_color: Hsla,
}

impl gpui::Global for Theme {}

impl Default for Theme {
    fn default() -> Self {
        Self::yuanbao()
    }
}

impl Theme {
    /// Yuanbao style theme - minimalist gray and white
    pub fn yuanbao() -> Self {
        Self {
            background: hsla(0.0, 0.0, 0.98, 1.0),      // #fafafa - off-white
            card: hsla(0.0, 0.0, 0.96, 1.0),            // #f5f5f5 - light gray

            foreground: hsla(0.0, 0.0, 0.10, 1.0),      // #1a1a1a - dark text
            muted_foreground: hsla(0.0, 0.0, 0.55, 1.0), // #8c8c8c - muted text

            border: hsla(0.0, 0.0, 0.92, 1.0),          // #ebebeb - light border

            primary: hsla(0.0, 0.0, 0.10, 1.0),         // #1a1a1a - dark primary
            primary_foreground: hsla(0.0, 0.0, 1.0, 1.0), // #ffffff - white text

            muted: hsla(0.0, 0.0, 0.88, 1.0),           // #e0e0e0 - muted background

            text_input: TextInputTheme {
                background: hsla(0.0, 0.0, 1.0, 1.0),   // #ffffff - pure white
                border_color: hsla(0.0, 0.0, 0.88, 1.0), // #e0e0e0 - light border
                border_color_focused: hsla(0.0, 0.0, 0.20, 1.0), // #333333 - dark border when focused
                text: TextStyle {
                    color: hsla(0.0, 0.0, 0.10, 1.0),
                    font_family: "Inter".into(),
                    font_size: px(15.).into(),
                    ..Default::default()
                },
                placeholder_color: hsla(0.0, 0.0, 0.65, 1.0), // #a6a6a6 - lighter placeholder
                selection_color: hsla(0.58, 0.8, 0.6, 0.3),
                cursor_color: hsla(0.0, 0.0, 0.10, 1.0),
            }
        }
    }
}

