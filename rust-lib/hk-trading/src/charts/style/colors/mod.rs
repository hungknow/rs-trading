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

define_color!(WHITE, 255, 255, 255, "White");
define_color!(BLACK, 0, 0, 0, "Black");
define_color!(RED, 255, 0, 0, "Red");
define_color!(GREEN, 0, 255, 0, "Green");
define_color!(BLUE, 0, 0, 255, "Blue");
define_color!(YELLOW, 255, 255, 0, "Yellow");
define_color!(CYAN, 0, 255, 255, "Cyan");
define_color!(MAGENTA, 255, 0, 255, "Magenta");
define_color!(TRANSPARENT, 0, 0, 0, 0.0, "Transparent");

// pub mod full_palette;
