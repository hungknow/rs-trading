use std::borrow::Borrow;

use crate::charts::{BackendCoord, DrawingBackend, DrawingErrorKind};

use super::{Drawable, PointCollection};

trait DynDrawable<DB: DrawingBackend> {
    fn draw_dyn(
        &self,
        points: &mut dyn Iterator<Item = BackendCoord>,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>>;
}

impl<DB: DrawingBackend, T: Drawable<DB>> DynDrawable<DB> for T {
    fn draw_dyn(
        &self,
        points: &mut dyn Iterator<Item = BackendCoord>,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        T::draw(self, points, backend, parent_dim)
    }
}

/// The container for a dynamically dispatched element
pub struct DynElement<'a, DB, Coord>
where
    DB: DrawingBackend,
    Coord: Clone,
{
    points: Vec<Coord>,
    drawable: Box<dyn DynDrawable<DB> + 'a>,
}

/// The trait that makes the conversion from the statically dispatched element
/// to the dynamically dispatched element
pub trait IntoDynElement<'a, DB: DrawingBackend, Coord: Clone>
where
    Self: 'a,
{
    /// Make the conversion
    fn into_dyn(self) -> DynElement<'a, DB, Coord>;
}

impl<'b, T, DB, Coord> IntoDynElement<'b, DB, Coord> for T
where
    T: Drawable<DB> + 'b,
    for<'a> &'a T: PointCollection<'a, Coord>,
    Coord: Clone,
    DB: DrawingBackend,
{
    fn into_dyn(self) -> DynElement<'b, DB, Coord> {
        DynElement {
            points: self
                .point_iter()
                .into_iter()
                .map(|x| x.borrow().clone())
                .collect(),
            drawable: Box::new(self),
        }
    }
}
