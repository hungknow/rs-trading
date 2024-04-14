mod shape;
mod size;
pub use shape::ShapeStyle;

mod color;
pub use color::{Color, RGBAColor, RGBColor};

pub mod colors;
pub use colors::{
    BLACK, BLACK_1, BLUE, CYAN, GREEN, GREEN_1, MAGENTA, RED, RED_1, TRANSPARENT, WHITE, YELLOW,
};
pub use size::SizeDesc;
