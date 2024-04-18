use super::{
    context::{ChartContext, TradingChartContext},
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

    pub fn calculate_sidebar_width() -> u32 {
        //TODO: Add logic to calculate sidebar width by text
        60
    }

    pub fn build_trading_chart_context<X: AsRangedCoord, Y: AsRangedCoord>(
        &mut self,
        x_spec: X,
        y_spec: Y,
        off_chart_count: u32,
    ) -> Result<
        TradingChartContext<DB, Cartesian2d<X::CoordDescType, Y::CoordDescType>>,
        DrawingAreaErrorKind<DB::ErrorType>,
    > {
        let mut root_drawing_area = DrawingArea::clone(self.root_area);
        let mut root_pixel_range = root_drawing_area.get_pixel_range();
        let mut root_dim = root_drawing_area.dim_in_pixel();

        // bottom bar

        let bottom_bar = 28;
        let side_bar_width = Self::calculate_sidebar_width();

        let chart_height = root_dim.1 - bottom_bar;

        let mut offchart_height_scale = 2.0 * (off_chart_count as f64).sqrt() / 7.0;
        // (off_chart_count || 1);
        offchart_height_scale = if off_chart_count == 0 {
            offchart_height_scale
        } else {
            offchart_height_scale / off_chart_count as f64
        };
        let offchart_height_pixel = (offchart_height_scale * chart_height as f64) as u32;

        let mut current_y: u32 = 0;
        let mut x_breakpoints = vec![];
        let mut y_breakpoints = vec![];

        // The bottom of main drawing area
        current_y += chart_height - offchart_height_pixel * off_chart_count;
        y_breakpoints.push(current_y);

        for _ in 0..off_chart_count {
            current_y += offchart_height_pixel;
            y_breakpoints.push(current_y);
        }

        // The left edge of the right side bar
        x_breakpoints.push(root_dim.0 - side_bar_width);

        // the right edge of the root drawing area
        // x_breakpoints.push(root_dim.0);

        // The bottom bar area
        y_breakpoints.push(chart_height);
        // y_breakpoints.push(root_dim.1);

        // root_drawing_area.split_by_breakpoints(xs, ys)

        //     let (w, h) = drawing_area.dim_in_pixel();
        //     let mut actual_drawing_area_pos = [0, h as i32, 0, w as i32];

        let mut split_drawing_area: Vec<_> = root_drawing_area
            .split_by_breakpoints(x_breakpoints, y_breakpoints)
            .into_iter()
            // .map(Some)
            .collect();

        // Take out the plotting area
        // std::mem::swap(&mut drawing_area, split[4].as_mut().unwrap());

        // let mut main_area_pixel_range = root_drawing_area.get_pixel_range();
        // main_area_pixel_range.0.end -= 1;
        // main_area_pixel_range.1.end -= 1;
        // main_area_pixel_range.1 = main_area_pixel_range.1.end..main_area_pixel_range.1.start;

        let mut off_chart_drawing_areas = vec![];
        let mut right_side_off_chart_drawing_areas = vec![];
        let offchart_starting_index = 2;
        for off_chart_index in 0..off_chart_count {
            let current_index = offchart_starting_index + off_chart_index as usize;

            off_chart_drawing_areas.push(split_drawing_area[current_index].apply_coord_spec(
                Cartesian2d::new(
                    x_spec.clone(),
                    y_spec.clone(),
                    split_drawing_area[current_index].get_pixel_range(),
                ),
            ));
            right_side_off_chart_drawing_areas.push(split_drawing_area[current_index + 1].clone());
        }
        let trading_chart_context = TradingChartContext {
            root_drawing_area: root_drawing_area,
            main_drawing_area: split_drawing_area[0].apply_coord_spec(Cartesian2d::new(
                x_spec.clone(),
                y_spec.clone(),
                split_drawing_area[0].get_pixel_range(),
            )),
            right_side_main_drawing_area: split_drawing_area[1].clone(),
            off_chart_drawing_areas,
            right_side_off_chart_drawing_areas,
            x_axis: split_drawing_area[split_drawing_area.len() - 1].clone(),
        };

        // std::mem::swap(&mut trading_chart_context.main_drawing_area.right_side_bar_area, split[4].as_mut().unwrap());

        Ok(trading_chart_context)
    }

    // pub fn build_cartesian_2d<X: AsRangedCoord, Y: AsRangedCoord>(
    //     &mut self,
    //     x_spec: X,
    //     y_spec: Y,
    // ) -> Result<
    //     ChartContext<DB, Cartesian2d<X::CoordDescType, Y::CoordDescType>>,
    //     DrawingAreaErrorKind<DB::ErrorType>,
    // > {
    //     let mut drawing_area = DrawingArea::clone(self.root_area);
    //     let mut pixel_range = drawing_area.get_pixel_range();
    //     let mut drawing_area_dim = drawing_area.dim_in_pixel();

    //     let side_bar_width = Self::calculate_sidebar_width();
    //     let x_breakpoints = vec![drawing_area_dim.0 - side_bar_width];
    //     let y_breakpoints: Vec<u32> = vec![];

    //     let splitted_drawing = drawing_area.split_by_breakpoints(x_breakpoints, y_breakpoints);

    //     // Take out the plotting area
    //     Ok(ChartContext {
    //         // right_side_bar_area: Some(splitted_drawing[1].clone()),
    //         drawing_area: &drawing_area.apply_coord_spec(Cartesian2d::new(
    //             x_spec,
    //             y_spec,
    //             pixel_range,
    //         )),
    //         // off_chart_drawings: vec![],
    //         // series_anno: vec![],
    //     })
    // }
}
