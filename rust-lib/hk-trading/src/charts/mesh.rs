use std::borrow::{Borrow, BorrowMut};

use super::{
    coord::{
        cartesian::Cartesian2d,
        ranged1d::{BoldPoints, Ranged, ValueFormatter},
    },
    drawing::DrawingAreaErrorKind,
    style::{Color, RGBColor, ShapeStyle},
    ChartContext, DrawingBackend,
};

pub struct MeshStyle<'a, 'b, X: Ranged, Y: Ranged, DB: DrawingBackend> {
    pub(super) bold_line_style: Option<ShapeStyle>,
    pub(super) n_x_labels: usize,
    pub(super) n_y_labels: usize,
    pub(super) target: &'b mut ChartContext<'a, DB, Cartesian2d<X, Y>>,
}

impl<'a, 'b, X, Y, XT, YT, DB> MeshStyle<'a, 'b, X, Y, DB>
where
    X: Ranged<ValueType = XT> + ValueFormatter<XT>,
    Y: Ranged<ValueType = YT> + ValueFormatter<YT>,
    DB: DrawingBackend,
{
    pub fn new(chart: &'b mut ChartContext<'a, DB, Cartesian2d<X, Y>>) -> Self {
        MeshStyle {
            target: chart,
            n_x_labels: 11,
            n_y_labels: 11,
            bold_line_style: None,
        }
    }
}

impl<'a, 'b, X, Y, DB> MeshStyle<'a, 'b, X, Y, DB>
where
    X: Ranged,
    Y: Ranged,
    DB: DrawingBackend,
{
    pub fn draw_mesh(&mut self) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
        let target = self.target.borrow_mut();

        let default_mesh_color_1 = RGBColor(0, 0, 0).mix(0.2);
        let default_mesh_color_2 = RGBColor(0, 0, 0).mix(0.1);

        let bold_style = self
            .bold_line_style
            .unwrap_or_else(|| (&default_mesh_color_1).into());

        // target.draw_mesh(
        // )?;
        // self.target.draw_mesh_lines(
        //     (
        //         LightPoints::new(self.n_y_labels, self.n_y_labels * self.y_light_lines_limit),
        //         LightPoints::new(self.n_x_labels, self.n_x_labels * self.x_light_lines_limit),
        //     ),
        //     (false, false),
        //     &ShapeStyle::from(&BLACK).filled(),
        //     |x, y, _| Some(format!("({}, {})", x, y)));

        self.target.draw_mesh_lines(
            (BoldPoints(self.n_y_labels), BoldPoints(self.n_x_labels)),
            (true, true),
            &bold_style,
            |xr, yr, m| { None }
        )?;

        Ok(())
        // Err(DrawingAreaErrorKind::LayoutError)
    }
}
