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

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        todo!("draw_line")
    }
}
