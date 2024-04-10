use crate::charts::{BackendCoord, DrawingBackend};

use super::{Drawable, PointCollection};

pub struct CandleStick<X, Y: PartialOrd> {
    points: [(X, Y); 4],
}

impl<'a, X: 'a, Y: PartialOrd + 'a> PointCollection<'a, (X, Y)> for &'a CandleStick<X, Y> {
    type Point = &'a (X, Y);
    type IntoIter = &'a [(X, Y)];

    fn point_iter(self) -> Self::IntoIter {
        &self.points
    }
}

impl<X, Y: PartialOrd, DB: DrawingBackend> Drawable<DB> for CandleStick<X, Y> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        pos: I,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), crate::charts::DrawingErrorKind<<DB as DrawingBackend>::ErrorType>> {
        // backend.draw_line(from, to, style)
        todo!();
        // width of candle
        // pixel_of_one_candle * 0.6
        
        Ok(())
    }
}
