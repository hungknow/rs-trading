use crate::charts::{coord::Shift, drawing::area::{DrawingArea, IntoDrawingArea}, DrawingBackend};

pub struct MockedBackend {
    height: u32,
    width: u32,
    init_count: u32,
}

impl MockedBackend {
    pub fn new(width: u32, height: u32) -> Self {
        MockedBackend {
            height,
            width,
            init_count: 0,
        }
    }
}

#[derive(Debug)]
pub struct MockedError;

impl std::fmt::Display for MockedError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "MockedError")
    }
}

impl std::error::Error for MockedError {}

impl DrawingBackend for MockedBackend {
    type ErrorType = MockedError;

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn ensure_prepared(&mut self) -> Result<(), crate::charts::DrawingErrorKind<Self::ErrorType>> {
        self.init_count += 1;
        Ok(())
    }
}

pub fn create_mocked_drawing_area(width: u32, height: u32) -> DrawingArea<MockedBackend, Shift> {
    let mut backend = MockedBackend::new(width, height);

    backend.into_drawing_area()
}