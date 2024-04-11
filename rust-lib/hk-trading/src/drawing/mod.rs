// use plotters::{backend::DrawingBackend, coord::CoordTranslate, chart::ChartContext};

use crate::charts::{coord::CoordTranslate, DrawingBackend};

pub trait SubChartState {
    fn is_state_changed(self) -> bool;
}

pub trait SubChart {
    fn get_state(&self) -> dyn SubChartState;
    // fn draw<'a, DB: DrawingBackend, CT: CoordTranslate>(&self, chartContext: &mut ChartContext<'a, DB, CT>);
}
