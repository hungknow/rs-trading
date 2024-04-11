use std::borrow::Borrow;

use super::{coord::CoordTranslate, drawing::{DrawingArea, DrawingAreaErrorKind}, elements::{CoordMapper, Drawable, PointCollection}, overlays::SeriesAnno, DrawingBackend};

pub struct ChartContext<'a, DB: DrawingBackend, CT: CoordTranslate> {
    pub(crate) drawing_area: DrawingArea<DB, CT>,
    pub(crate) series_anno: Vec<SeriesAnno<'a, DB>>,
}

impl<'a, DB: DrawingBackend, CT: CoordTranslate> ChartContext<'a, DB, CT> {
    pub fn draw_series<B, E, R, S>(
        &mut self,
        series: S,
    ) -> Result<&mut SeriesAnno<'a, DB>, DrawingAreaErrorKind<DB::ErrorType>>
    where 
        B: CoordMapper,
        for<'b> &'b E: PointCollection<'b, CT::From, B>,
        E: Drawable<DB, B>,
        R: Borrow<E>,
        S: IntoIterator<Item = R>,
    {
        for element in series {
            self.drawing_area.draw(element.borrow())?;
        }
        Ok(self.alloc_series_anno())
    }

    pub(crate) fn alloc_series_anno(&mut self) -> &mut SeriesAnno<'a, DB> {
        let idx = self.series_anno.len();
        self.series_anno.push(SeriesAnno::new());
        &mut self.series_anno[idx]
    }

}