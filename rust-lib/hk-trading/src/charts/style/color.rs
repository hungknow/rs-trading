use crate::charts::{BackendColor, BackendStyle};

use super::ShapeStyle;

pub trait Color {
    /// Normalize this color representation to the backend color
    fn to_backend_color(&self) -> BackendColor;

    /// Convert the RGB representation to the standard RGB tuple
    #[inline(always)]
    fn rgb(&self) -> (u8, u8, u8) {
        self.to_backend_color().rgb
    }

    /// Get the alpha channel of the color
    #[inline(always)]
    fn alpha(&self) -> f64 {
        self.to_backend_color().alpha
    }

    /// Mix the color with given opacity
    fn mix(&self, value: f64) -> RGBAColor {
        let (r, g, b) = self.rgb();
        let a = self.alpha() * value;
        RGBAColor(r, g, b, a)
    }

    /// Convert the color into the RGBA color which is internally used by Plotters
    fn to_rgba(&self) -> RGBAColor {
        let (r, g, b) = self.rgb();
        let a = self.alpha();
        RGBAColor(r, g, b, a)
    }

    /// Make a filled style form the color
    fn filled(&self) -> ShapeStyle
    where
        Self: Sized,
    {
        Into::<ShapeStyle>::into(self).filled()
    }

    /// Make a shape style with stroke width from a color
    fn stroke_width(&self, width: u32) -> ShapeStyle
    where
        Self: Sized,
    {
        Into::<ShapeStyle>::into(self).stroke_width(width)
    }
}

impl<T: Color> Color for &'_ T {
    fn to_backend_color(&self) -> BackendColor {
        <T as Color>::to_backend_color(*self)
    }
}

/// The color described by its RGB value
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct RGBColor(pub u8, pub u8, pub u8);

impl Color for RGBColor {
    #[inline(always)]
    fn to_backend_color(&self) -> BackendColor {
        BackendColor {
            rgb: (self.0, self.1, self.2),
            alpha: 1.0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct RGBAColor(pub u8, pub u8, pub u8, pub f64);

impl Color for RGBAColor {
    #[inline(always)]
    fn to_backend_color(&self) -> BackendColor {
        BackendColor {
            rgb: (self.0, self.1, self.2),
            alpha: self.3,
        }
    }
}

impl From<RGBColor> for RGBAColor {
    fn from(rgb: RGBColor) -> Self {
        Self(rgb.0, rgb.1, rgb.2, 1.0)
    }
}
