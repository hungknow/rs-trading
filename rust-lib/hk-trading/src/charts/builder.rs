use super::{
    context::ChartContext,
    coord::{cartesian::Cartesian2d, ranged1d::AsRangedCoord, Shift},
    drawing::{DrawingArea, DrawingAreaErrorKind},
    DrawingBackend,
};

pub struct ChartBuilder<'a, DB: DrawingBackend> {
    root_area: &'a DrawingArea<DB, Shift>,
}

impl<'a, DB: DrawingBackend> ChartBuilder<'a, DB> {
    pub fn on(root: &'a DrawingArea<DB, Shift>) -> Self {
        Self {
            // label_area_size: [0; 4],
            root_area: root,
            // title: None,
            // margin: [0; 4],
            // overlap_plotting_area: [false; 4],
        }
    }

    pub fn build_cartesian_2d<X: AsRangedCoord, Y: AsRangedCoord>(
        &mut self,
        x_spec: X,
        y_spec: Y,
    ) -> Result<
        ChartContext<'a, DB, Cartesian2d<X::CoordDescType, Y::CoordDescType>>,
        DrawingAreaErrorKind<DB::ErrorType>,
    > {
        let mut drawing_area = DrawingArea::clone(self.root_area);
        let mut pixel_range = drawing_area.get_pixel_range();

        Ok(ChartContext {
            drawing_area: drawing_area.apply_coord_spec(Cartesian2d::new(
                x_spec,
                y_spec,
                pixel_range,
            )),
            series_anno: vec![],
        })
    }
}
