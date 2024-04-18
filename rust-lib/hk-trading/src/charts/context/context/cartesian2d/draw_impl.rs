use crate::charts::{
    coord::{
        cartesian::{Cartesian2d, MeshLine},
        ranged1d::{KeyPointHint, Ranged},
    },
    drawing::DrawingAreaErrorKind,
    style::ShapeStyle,
    ChartContext, DrawingBackend,
};

impl<'a, DB: DrawingBackend, X: Ranged, Y: Ranged> ChartContext<'a, DB, Cartesian2d<X, Y>> {
    pub fn draw_mesh_lines<FmtLabel, YH: KeyPointHint, XH: KeyPointHint>(
        &mut self,
        (r, c): (YH, XH),
        (x_mesh, y_mesh): (bool, bool),
        mesh_line_style: &ShapeStyle,
        mut fmt_label: FmtLabel,
    ) -> Result<(Vec<(i32, String)>, Vec<(i32, String)>), DrawingAreaErrorKind<DB::ErrorType>>
    where
        FmtLabel: FnMut(&X, &Y, &MeshLine<X, Y>) -> Option<String>,
    {
        self.drawing_area.draw_mesh(
            |b, l| {
                let draw = match l {
                    MeshLine::XMesh((x, _), _, _) => x_mesh,
                    MeshLine::YMesh((y, _), _, _) => y_mesh,
                };
                if draw {
                    l.draw(b, mesh_line_style)
                } else {
                    Ok(())
                }
            },
            r,
            c,
        )?;

        //TODO: Fill in x_labels and y_labels
        Ok((vec![], vec![]))
    }

    // pub(crate) fn draw_mesh<FmtLabel, YH: KeyPointHint, XH: KeyPointHint>(
}
