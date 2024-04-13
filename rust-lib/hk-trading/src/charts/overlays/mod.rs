mod series;
pub use series::SeriesAnno;

use super::{
    context::ChartContext,
    coord::CoordTranslate,
    drawing::DrawingAreaErrorKind,
    DrawingBackend,
};

mod ohlcs;
pub use ohlcs::Ohlcs;

pub trait OverlayDrawing<DB: DrawingBackend, CT: CoordTranslate> {
    fn draw<'a>(
        &mut self,
        chart_context: &mut ChartContext<'a, DB, CT>,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>;
}
