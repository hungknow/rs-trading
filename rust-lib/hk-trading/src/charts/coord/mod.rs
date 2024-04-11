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

/// The primitive types supported by Plotters coordinate system
pub mod types {
    pub use super::ranged1d::types::*;
}

pub mod ranged1d;
mod ranged2d;

/// Groups Cartesian ranged coordinates in 2d and 3d.
pub mod cartesian {
    pub use super::ranged2d::cartesian::{Cartesian2d};
}