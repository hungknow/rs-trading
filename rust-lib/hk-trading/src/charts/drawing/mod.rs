mod area;
pub use area::{DrawingArea, DrawingAreaErrorKind, IntoDrawingArea, Rect};

#[cfg(test)]
mod backend_impl;
#[cfg(test)]
pub use backend_impl::*;
