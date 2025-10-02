// Theme configuration - Yuanbao style

use gpui::Hsla;

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
}

impl Theme {
    /// Yuanbao style theme - minimalist gray and white
    pub fn yuanbao() -> Self {
        Self {
            background: hsla(0.0, 0.0, 1.0, 1.0),       // #ffffff - pure white
            card: hsla(0.0, 0.0, 0.96, 1.0),            // #f5f5f5 - light gray
            
            foreground: hsla(0.0, 0.0, 0.10, 1.0),      // #1a1a1a - dark text
            muted_foreground: hsla(0.0, 0.0, 0.60, 1.0), // #999999 - muted text
            
            border: hsla(0.0, 0.0, 0.90, 1.0),          // #e5e5e5 - light border
            
            primary: hsla(0.0, 0.0, 0.10, 1.0),         // #1a1a1a - dark primary
            primary_foreground: hsla(0.0, 0.0, 1.0, 1.0), // #ffffff - white text
        }
    }
}

// Helper function to create HSLA colors
fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
    Hsla { h, s, l, a }
}

