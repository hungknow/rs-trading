use crate::charts::{BackendCoord, DrawingBackend};

use super::{Drawable, PointCollection};

pub struct MockedElement<X, Y> {
    pub(crate) points: [(X, Y); 4],
}

impl<X, Y, DB: DrawingBackend> Drawable<DB> for MockedElement<X, Y> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        pos: I,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), crate::charts::DrawingErrorKind<<DB as DrawingBackend>::ErrorType>>
    {
        Ok(())
    }
}

impl<'a, X: 'a, Y: PartialOrd + 'a> PointCollection<'a, (X, Y)> for &'a MockedElement<X, Y> {
    type Point = &'a (X, Y);
    type IntoIter = &'a [(X, Y)];

    fn point_iter(self) -> Self::IntoIter {
        &self.points
    }
}