use crate::charts::{coord::CoordTranslate, drawing::DrawingArea, DrawingBackend};

/// The trait indicates that the type has a dimensional data.
/// This is the abstraction for the relative sizing model.
/// A relative sizing value is able to be converted into a concrete size
/// when coupling with a type with `HasDimension` type.
pub trait HasDimension {
    /// Get the dimensional data for this object
    fn dim(&self) -> (u32, u32);
}

impl<D: DrawingBackend, C: CoordTranslate> HasDimension for DrawingArea<D, C> {
    fn dim(&self) -> (u32, u32) {
        self.dim_in_pixel()
    }
}

impl HasDimension for (u32, u32) {
    fn dim(&self) -> (u32, u32) {
        *self
    }
}

/// The trait that describes a size, it may be a relative size which the
/// size is determined by the parent size, e.g., 10% of the parent width
pub trait SizeDesc {
    /// Convert the size into the number of pixels
    ///
    /// - `parent`: The reference to the parent container of this size
    /// - **returns**: The number of pixels
    fn in_pixels<T: HasDimension>(&self, parent: &T) -> i32;
}

impl SizeDesc for i32 {
    fn in_pixels<D: HasDimension>(&self, _parent: &D) -> i32 {
        *self
    }
}

impl SizeDesc for u32 {
    fn in_pixels<D: HasDimension>(&self, _parent: &D) -> i32 {
        *self as i32
    }
}

impl SizeDesc for f32 {
    fn in_pixels<D: HasDimension>(&self, _parent: &D) -> i32 {
        *self as i32
    }
}

impl SizeDesc for f64 {
    fn in_pixels<D: HasDimension>(&self, _parent: &D) -> i32 {
        *self as i32
    }
}
