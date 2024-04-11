use crate::charts::{BackendCoord, DrawingBackend, DrawingErrorKind};

trait DynDrawable<DB: DrawingBackend> {
    fn draw_dyn(
        &self,
        points: &mut dyn Iterator<Item = BackendCoord>,
        backend: &mut DB,
        parent_dim: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>>;
}

/// The container for a dynamically dispatched element
pub struct DynElement<'a, DB, Coord>
where
    DB: DrawingBackend,
    Coord: Clone,
{
    points: Vec<Coord>,
    drawable: Box<dyn DynDrawable<DB> + 'a>,
}
