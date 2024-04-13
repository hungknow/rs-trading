use std::error::Error;

mod style;
pub use style::*;

#[derive(Debug)]
pub enum DrawingErrorKind<E: Error + Send + Sync> {
    /// A drawing backend error
    DrawingError(E),
    /// A font rendering error
    FontError(Box<dyn Error + Send + Sync + 'static>),
}

/// A coordinate in the pixel-based backend. The coordinate follows the framebuffer's convention,
/// which defines the top-left point as (0, 0).
pub type BackendCoord = (i32, i32);

pub trait DrawingBackend: Sized {
    /// The error type reported by the backend
    type ErrorType: Error + Send + Sync;

    /// Get the dimension of the drawing backend in pixels
    fn get_size(&self) -> (u32, u32);

    /// Ensure the backend is ready to draw
    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>>;

    /// Finalize the drawing step and present all the changes.
    /// This is used as the real-time rendering support.
    /// The backend may implement in the following way, when `ensure_prepared` is called
    /// it checks if it needs a fresh buffer and `present` is called rendering all the
    /// pending changes on the screen.
    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>>;

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        todo!("draw_line")
    }

    /// Draw a rectangle on the drawing backend
    /// - `upper_left`: The coordinate of the upper-left corner of the rect
    /// - `bottom_right`: The coordinate of the bottom-right corner of the rect
    /// - `style`: The style
    /// - `fill`: If the rectangle should be filled
    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        todo!("draw_rect")
    }
}
