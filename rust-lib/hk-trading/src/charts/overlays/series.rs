use crate::charts::{elements::DynElement, BackendCoord, DrawingBackend};

type SeriesAnnoDrawFn<'a, DB> = dyn Fn(BackendCoord) -> DynElement<'a, DB, BackendCoord> + 'a;

/// The annotations (such as the label of the series, the legend element, etc)
/// When a series is drawn onto a drawing area, an series annotation object
/// is created and a mutable reference is returned.
pub struct SeriesAnno<'a, DB: DrawingBackend> {
    label: Option<String>,
    draw_func: Option<Box<SeriesAnnoDrawFn<'a, DB>>>,
}

impl<'a, DB: DrawingBackend> SeriesAnno<'a, DB> {
    pub(crate) fn new() -> Self {
        Self {
            label: None,
            draw_func: None,
        }
    }
}
