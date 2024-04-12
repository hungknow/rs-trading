use std::cmp::Ordering;

use crate::charts::{style::ShapeStyle, BackendCoord, DrawingBackend};

use super::{Drawable, PointCollection};

#[derive(Clone)]
pub struct CandleStick<X, Y: PartialOrd> {
    pub(crate) style: ShapeStyle,
    pub(crate) width: u32,
    points: [(X, Y); 4],
}

impl<X: Clone, Y: PartialOrd> CandleStick<X, Y> {
    /// Create a new candlestick element, which requires the Y coordinate can be compared
    ///
    /// - `x`: The x coordinate
    /// - `open`: The open value
    /// - `high`: The high value
    /// - `low`: The low value
    /// - `close`: The close value
    /// - `gain_style`: The style for gain
    /// - `loss_style`: The style for loss
    /// - `width`: The width
    /// - **returns** The newly created candlestick element
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn new<GS: Into<ShapeStyle>, LS: Into<ShapeStyle>>(
        x: X,
        open: Y,
        high: Y,
        low: Y,
        close: Y,
        gain_style: GS,
        loss_style: LS,
        width: u32,
    ) -> Self {
        Self {
            style: match open.partial_cmp(&close) {
                Some(Ordering::Less) => gain_style.into(),
                _ => loss_style.into(),
            },
            width,
            points: [
                (x.clone(), open),
                (x.clone(), high),
                (x.clone(), low),
                (x, close),
            ],
        }
    }
}

impl<'a, X: 'a, Y: PartialOrd + 'a> PointCollection<'a, (X, Y)> for &'a CandleStick<X, Y> {
    type Point = &'a (X, Y);
    type IntoIter = &'a [(X, Y)];

    fn point_iter(self) -> Self::IntoIter {
        &self.points
    }
}

impl<X, Y: PartialOrd, DB: DrawingBackend> Drawable<DB> for CandleStick<X, Y> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        pos: I,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), crate::charts::DrawingErrorKind<<DB as DrawingBackend>::ErrorType>> {
        // backend.draw_line(from, to, style)
        todo!();
        // width of candle
        // pixel_of_one_candle * 0.6
        // open = scale * open + shift
        //      scale = (- height_in_pixel / (high - low))
        //      shift = (- high * scale)
        Ok(())
    }
}
