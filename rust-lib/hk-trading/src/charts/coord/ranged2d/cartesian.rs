use crate::charts::{coord::{ranged1d::Ranged, CoordTranslate}, BackendCoord};

/// A 2D Cartesian coordinate system described by two 1D ranged coordinate specs.
#[derive(Clone)]
pub struct Cartesian2d<X: Ranged, Y: Ranged> {
    logic_x: X,
    logic_y: Y,
    back_x: (i32, i32),
    back_y: (i32, i32),
}

impl<X: Ranged, Y: Ranged> CoordTranslate for Cartesian2d<X, Y> {
    type From = (X::ValueType, Y::ValueType);

    fn translate(&self, from: &Self::From) -> BackendCoord {
        (
            self.logic_x.map(&from.0, self.back_x),
            self.logic_y.map(&from.1, self.back_y),
        )
    }
}
