mod series;
use std::sync::Arc;

pub use series::SeriesAnno;

use super::{
    context::ChartContext, coord::CoordTranslate, drawing::DrawingAreaErrorKind, DrawingBackend,
};

mod ohlcs;
pub use ohlcs::OhlcOverlay;

pub trait OverlayData<T, S> {
    fn overlay_name(&self) -> &str;
    fn overlay_type(&self) -> &str;
    fn overlay_data<'a>(&self) -> Option<Arc<T>>;
    // fn overlay_data_mut(&mut self) -> Option<Arc<T>>;

    // The higher value, the topper it is drawed on other overlays
    fn priority(&self) -> u32;
    //TODO: Support gettings later
    fn get_settings(&self) -> &S;
}

pub trait OverlayDrawing<DB: DrawingBackend, CT: CoordTranslate> {
    fn draw<'a>(
        &mut self,
        chart_context: &mut ChartContext<'a, DB, CT>,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>;
}
