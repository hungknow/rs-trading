//! Basic predefined colors.
use super::{RGBAColor, RGBColor};

/// Defines and names a color based on its R, G, B, A values.
#[macro_export]
macro_rules! define_color {
    ($name:ident, $r:expr, $g:expr, $b:expr, $doc:expr) => {
        pub const $name: RGBColor = RGBColor($r, $g, $b);

    };

    ($name:ident, $r:expr, $g:expr, $b:expr, $a: expr, $doc:expr) => {
        pub const $name: RGBAColor = RGBAColor($r, $g, $b, $a);
    };
}

define_color!(RED, 255, 0, 0, "Red");
define_color!(GREEN, 0, 255, 0, "Green");

// pub mod full_palette;
