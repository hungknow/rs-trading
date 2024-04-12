mod area;
pub use area::{Rect, DrawingArea, DrawingAreaErrorKind};

#[cfg(test)]
mod backend_impl;
#[cfg(test)]
pub use backend_impl::*;