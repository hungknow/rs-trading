use crate::charts::BackendCoord;

/// The trait that translates some customized object to the backend coordinate
pub trait CoordTranslate {
    /// Specifies the object to be translated from
    type From;

    fn translate(&self, from: &Self::From) -> BackendCoord;

}
