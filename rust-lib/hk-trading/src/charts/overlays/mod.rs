mod series;
use std::sync::Arc;

pub use series::SeriesAnno;

use super::{
    context::ChartContext, coord::CoordTranslate, drawing::DrawingAreaErrorKind, DrawingBackend,
};

mod empty;
pub use empty::EmptyOverlay;

mod ohlcs;
pub use ohlcs::OhlcOverlay;

pub trait OverlayData<DataType, Setting> {
    fn overlay_name(&self) -> &str;
    fn overlay_type(&self) -> &str;
    fn overlay_data<'a>(&self) -> Option<Arc<DataType>>;
    // fn overlay_data_mut(&mut self) -> Option<Arc<T>>;

    // The higher value, the topper it is drawed on other overlays
    fn priority(&self) -> u32;
    //TODO: Support gettings later
    fn get_settings(&self) -> &Setting;
}

pub trait OverlayDrawing<DB: DrawingBackend, CT: CoordTranslate> {
    fn draw(
        &mut self,
        chart_context: &mut ChartContext<DB, CT>,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>;
}

pub trait Overlay<DB: DrawingBackend, CT: CoordTranslate> {
    fn overlay_name(&self) -> &str;
    fn overlay_type(&self) -> &str;
    fn priority(&self) -> u32;

    fn draw(
        &mut self,
        chart_context: &mut ChartContext<DB, CT>,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>;
}
