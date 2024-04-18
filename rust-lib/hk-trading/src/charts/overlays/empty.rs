use chrono::{DateTime, Utc};

use crate::charts::{
    coord::{
        cartesian::Cartesian2d,
        types::{RangedCoordf64, RangedDateTime},
    },
    drawing::Rect,
    elements::Rectangle,
    style::{Color, BLUE},
    DrawingBackend,
};

use super::Overlay;

pub struct EmptyOverlay {}

impl EmptyOverlay {
    pub fn new() -> Self {
        Self {}
    }
}

impl<DB: DrawingBackend> Overlay<DB, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf64>>
    for EmptyOverlay
{
    fn overlay_name(&self) -> &str {
        "EmptyOverlay"
    }

    fn overlay_type(&self) -> &str {
        "EMPTY"
    }

    fn priority(&self) -> u32 {
        1
    }

    fn draw(
        &mut self,
        chart_context: &mut crate::charts::context::ChartContext<
            DB,
            Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf64>,
        >,
    ) -> Result<(), crate::charts::drawing::DrawingAreaErrorKind<DB::ErrorType>> {
        // let x_range = chart_context.drawing_area.get_x_range();
        // let y_range = chart_context.drawing_area.get_y_range();
        // chart_context.drawing_area.draw(&Rectangle::new(
        //     [
        //         (x_range.start, y_range.start),
        //         (x_range.end, y_range.end),
        //     ],
        //     Color::stroke_width(&BLUE, 2),
        // ));
        Ok(())
    }
}
