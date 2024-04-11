use std::ops::Deref;

use crate::charts::BackendCoord;

/// The trait that translates some customized object to the backend coordinate
pub trait CoordTranslate {
    /// Specifies the object to be translated from
    type From;

    fn translate(&self, from: &Self::From) -> BackendCoord;

    /// Get the Z-value of current coordinate
    fn depth(&self, _from: &Self::From) -> i32 {
        0
    }
}

impl<C, T> CoordTranslate for T
where
    C: CoordTranslate,
    T: Deref<Target = C>,
{
    type From = C::From;
    fn translate(&self, from: &Self::From) -> BackendCoord {
        self.deref().translate(from)
    }
}
