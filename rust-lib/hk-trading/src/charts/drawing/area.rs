use std::{cell::RefCell, error::Error, rc::Rc};

use crate::charts::{
    coord::CoordTranslate,
    elements::{CoordMapper, Drawable, PointCollection},
    DrawingBackend, DrawingErrorKind,
};

/// The representation of the rectangle in backend canvas
#[derive(Clone, Debug)]
pub struct Rect {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl Rect {
    /// Make the coordinate in the range of the rectangle
    pub fn truncate(&self, p: (i32, i32)) -> (i32, i32) {
        (p.0.min(self.x1).max(self.x0), p.1.min(self.y1).max(self.y0))
    }
}

/// The abstraction of a drawing area. Plotters uses drawing area as the fundamental abstraction for the
/// high level drawing API. The major functionality provided by the drawing area is
/// 1. Layout specification - Split the parent drawing area into sub-drawing-areas
/// 2. Coordinate Translation - Allows guest coordinate system attached and used for drawing.
/// 3. Element based drawing - drawing area provides the environment the element can be drawn onto it.
pub struct DrawingArea<DB: DrawingBackend, CT: CoordTranslate> {
    backend: Rc<RefCell<DB>>,
    rect: Rect,
    coord: CT,
}

// pub struct DrawingAreaState {

// }

/// The error description of any drawing area API
#[derive(Debug)]
pub enum DrawingAreaErrorKind<E: Error + Send + Sync> {
    /// The error is due to drawing backend failure
    BackendError(DrawingErrorKind<E>),
    /// We are not able to get the mutable reference of the backend,
    /// which indicates the drawing backend is current used by other
    /// drawing operation
    SharingError,
    /// The error caused by invalid layout
    LayoutError,
}

type DrawingAreaError<T: DrawingBackend> = DrawingAreaErrorKind<T::ErrorType>;

impl<DB: DrawingBackend, CT: CoordTranslate> DrawingArea<DB, CT> {
    pub fn draw_grid() {
        // Clear the canvas
        // Draw grid mesh
        // Sort the overlays by z-index
        // Draw each overlay
        // If overlay isn't displayed, skip
        // Draw the crosshair
    }

    fn backend_ops<R, O: FnOnce(&mut DB) -> Result<R, DrawingErrorKind<DB::ErrorType>>>(
        &self,
        ops: O,
    ) -> Result<R, DrawingAreaError<DB>> {
    }
    // Draw element
    pub fn draw<'a, E, B>(&self, element: &'a E) -> Result<(), DrawingAreaError<DB>>
    where
        B: CoordMapper,
        &'a E: PointCollection<'a, CT::From, B>,
        E: Drawable<DB, B>,
    {
        let backend_coords = element.point_iter().into_iter().map(|p| {
            let b = p.borrow();
            B::map(&self.coord, b, &self.rect)
        });
        self.backend_ops(move |b| element.draw(backend_coords, b, self.dim_in_pixel()))
    }

    /// Get the area dimension in pixel
    pub fn dim_in_pixel(&self) -> (u32, u32) {
        (
            (self.rect.x1 - self.rect.x0) as u32,
            (self.rect.y1 - self.rect.y0) as u32,
        )
    }
}
