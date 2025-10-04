// Utility functions

use gpui::{Bounds, Pixels, Point, Size};

/// Calculate window bounds for right-aligned positioning
pub fn calculate_window_bounds(
    display_size: Size<Pixels>,
    window_size: Size<Pixels>,
    margin: Pixels,
) -> Bounds<Pixels> {
    // Right-aligned position
    let x = display_size.width - window_size.width - margin;
    let y = margin;

    // Limit height to fit screen
    let max_height = display_size.height - margin * 2.0;
    let actual_height = window_size.height.min(max_height);

    Bounds {
        origin: Point::new(x, y),
        size: Size {
            width: window_size.width,
            height: actual_height,
        },
    }
}

