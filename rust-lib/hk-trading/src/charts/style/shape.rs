use crate::charts::{BackendColor, BackendStyle};

use super::color::{Color, RGBAColor};

/// Style for any shape
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShapeStyle {
    /// Specification of the color.
    pub color: RGBAColor,
    /// Whether the style is filled with color.
    pub filled: bool,
    /// Stroke width.
    pub stroke_width: u32,
}

impl ShapeStyle {
    pub fn filled(&self) -> Self {
        Self {
            color: self.color.to_rgba(),
            filled: true,
            stroke_width: self.stroke_width,
        }
    }
}

impl<T: Color> From<T> for ShapeStyle {
    fn from(f: T) -> Self {
        ShapeStyle {
            color: f.to_rgba(),
            filled: false,
            stroke_width: 1,
        }
    }
}

impl BackendStyle for ShapeStyle {
    /// Returns the color as interpreted by the backend.
    fn color(&self) -> BackendColor {
        self.color.to_backend_color()
    }
    /// Returns the stroke width.
    fn stroke_width(&self) -> u32 {
        self.stroke_width
    }
}
