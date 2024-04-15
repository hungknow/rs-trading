use std::borrow::Borrow;

use super::{
    coord::{CoordTranslate, Shift},
    drawing::{DrawingArea, DrawingAreaErrorKind},
    elements::{CoordMapper, Drawable, PointCollection},
    DrawingBackend,
};

pub struct ChartContext<'a, DB: DrawingBackend, CT: CoordTranslate> {
    // pub right_side_bar_area: Option<DrawingArea<DB, Shift>>,
    // The main drawing area (for Ohlcs)
    pub drawing_area: &'a DrawingArea<DB, CT>,
    // The drawing area for off chart drawings (Indicators draw their data here)
    // pub(crate) off_chart_drawings: Vec<DrawingArea<DB, CT>>,
    // pub(crate) series_anno: Vec<SeriesAnno<'a, DB>>,
}

impl<'a, DB: DrawingBackend, CT: CoordTranslate> ChartContext<'a, DB, CT> {
    pub fn new(drawing_area: &'a DrawingArea<DB, CT>) -> Self {
        Self { drawing_area }
    }

    pub fn draw_series<B, E, R, S>(
        &mut self,
        series: S,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
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
        Ok(())
        // Ok(self.alloc_series_anno())
    }

    // pub(crate) fn alloc_series_anno(&mut self) -> &mut SeriesAnno<'a, DB> {
    //     let idx = self.series_anno.len();
    //     self.series_anno.push(SeriesAnno::new());
    //     &mut self.series_anno[idx]
    // }
}
// Contains all main drawing area and off chart drawing areas
// +----------+------------------------------+------+
// |    1     |      2 (Main chart)          |   3  |
// |  Left    |      Plotting Area)          | Right|
// |  Labels  |                              | Label|
// +----------+------------------------------+------+
// |    6     |        7 (OffChart 1)        |   8  |
// +----------+------------------------------+------+
// |    9     |        10 (OffChart 2)       |  11  |
// +----------+------------------------------+------+
// |    12    |        13 (Bottom Labels)    |  14  |
// +----------+------------------------------+------+
pub struct TradingChartContext<DB: DrawingBackend, CT: CoordTranslate> {
    pub root_drawing_area: DrawingArea<DB, Shift>,
    pub main_drawing_area: DrawingArea<DB, CT>,
    pub right_side_main_drawing_area: DrawingArea<DB, Shift>,
    // The drawing area for off chart drawings (Indicators draw their data here)
    pub off_chart_drawing_areas: Vec<DrawingArea<DB, CT>>,
    pub right_side_off_chart_drawing_areas: Vec<DrawingArea<DB, Shift>>,

    // The X-axis area
    pub x_axis: DrawingArea<DB, Shift>,
    // pub(crate) y_axis: DrawingArea<DB, CT>,
    // pub(crate) drawing_area: DrawingArea<DB, super::coord::cartesian::Cartesian2d<_, _>>,
}
