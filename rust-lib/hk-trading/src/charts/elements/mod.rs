use std::borrow::Borrow;

use super::{coord::CoordTranslate, drawing::Rect, BackendCoord, DrawingBackend, DrawingErrorKind};

/// Used for 2d coordinate transformations.
pub struct BackendCoordOnly;

impl CoordMapper for BackendCoordOnly {
    type Output = BackendCoord;
    fn map<CT: CoordTranslate>(coord_trans: &CT, from: &CT::From, rect: &Rect) -> BackendCoord {
        rect.truncate(coord_trans.translate(from))
    }
}

/// Useful to translate from guest coordinates to backend coordinates
pub trait CoordMapper {
    /// Specifies the output data from the translation
    type Output;
    /// Performs the translation from guest coordinates to backend coordinates
    fn map<CT: CoordTranslate>(coord_trans: &CT, from: &CT::From, rect: &Rect) -> Self::Output;
}

pub trait Drawable<DB: DrawingBackend, CM: CoordMapper = BackendCoordOnly> {
    /// Actually draws the element. The key points is already translated into the
    /// image coordinate and can be used by DC directly
    fn draw<I: Iterator<Item = CM::Output>>(
        &self,
        pos: I,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>>;
}

pub trait PointCollection<'a, Coord, CM = BackendCoordOnly> {
    /// The item in point iterator
    type Point: Borrow<Coord> + 'a;

    /// The point iterator
    type IntoIter: IntoIterator<Item = Self::Point>;

    /// framework to do the coordinate mapping
    fn point_iter(self) -> Self::IntoIter;
}

mod candlestick;
pub use candlestick::*;

mod dynelem;
pub use dynelem::*;

#[cfg(test)]
mod mocked_element;
#[cfg(test)]
pub use mocked_element::*;