mod series;
pub use series::SeriesAnno;

use super::{
    context::ChartContext,
    coord::CoordTranslate,
    drawing::DrawingAreaErrorKind,
    DrawingBackend,
};

pub mod ohlcs;

pub trait OverlayDrawing<DB: DrawingBackend, CT: CoordTranslate> {
    fn draw<'a>(
        &mut self,
        chart_context: &mut ChartContext<'a, DB, CT>,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>;
}
