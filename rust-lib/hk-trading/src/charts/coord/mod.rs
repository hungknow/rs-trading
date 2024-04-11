mod translate;
pub use translate::CoordTranslate;

use super::BackendCoord;

/// The coordinate translation that only impose shift
#[derive(Debug, Clone)]
pub struct Shift(pub BackendCoord);

impl CoordTranslate for Shift {
    type From = BackendCoord;
    fn translate(&self, from: &Self::From) -> BackendCoord {
        (from.0 + (self.0).0, from.1 + (self.0).1)
    }
}