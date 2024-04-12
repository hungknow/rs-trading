use std::ops::Range;

use crate::charts::{
    coord::{ranged1d::Ranged, CoordTranslate},
    BackendCoord,
};

/// A 2D Cartesian coordinate system described by two 1D ranged coordinate specs.
#[derive(Clone)]
pub struct Cartesian2d<X: Ranged, Y: Ranged> {
    logic_x: X,
    logic_y: Y,
    back_x: (i32, i32),
    back_y: (i32, i32),
}

impl<X: Ranged, Y: Ranged> Cartesian2d<X, Y> {
    /// Create a new 2D cartesian coordinate system
    /// - `logic_x` and `logic_y` : The description for the 1D coordinate system
    /// - `actual`: The pixel range on the screen for this coordinate system
    pub fn new<IntoX: Into<X>, IntoY: Into<Y>>(
        logic_x: IntoX,
        logic_y: IntoY,
        actual: (Range<i32>, Range<i32>),
    ) -> Self {
        Self {
            logic_x: logic_x.into(),
            logic_y: logic_y.into(),
            back_x: (actual.0.start, actual.0.end),
            back_y: (actual.1.start, actual.1.end),
        }
    }

    /// Get the range of X axis
    pub fn get_x_range(&self) -> Range<X::ValueType> {
        self.logic_x.range()
    }

    /// Get the range of Y axis
    pub fn get_y_range(&self) -> Range<Y::ValueType> {
        self.logic_y.range()
    }

    /// Get the horizental backend coordinate range where X axis should be drawn
    pub fn get_x_axis_pixel_range(&self) -> Range<i32> {
        self.logic_x.axis_pixel_range(self.back_x)
    }

    /// Get the vertical backend coordinate range where Y axis should be drawn
    pub fn get_y_axis_pixel_range(&self) -> Range<i32> {
        self.logic_y.axis_pixel_range(self.back_y)
    }

    /// Get the 1D coordinate spec for X axis
    pub fn x_spec(&self) -> &X {
        &self.logic_x
    }

    /// Get the 1D coordinate spec for Y axis
    pub fn y_spec(&self) -> &Y {
        &self.logic_y
    }
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
