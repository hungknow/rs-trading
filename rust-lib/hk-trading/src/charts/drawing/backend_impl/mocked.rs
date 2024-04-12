use std::{cell::RefCell, rc::Rc};

use crate::charts::{
    context::ChartContext,
    coord::{CoordTranslate, Shift},
    drawing::{
        area::{DrawingArea, IntoDrawingArea},
        Rect,
    },
    DrawingBackend,
};

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

// pub fn create_mocked_chart_context<CT: CoordTranslate>(
//     width: u32,
//     height: u32,
//     ct: CT,
// ) -> ChartContext<'static, MockedBackend, CT> {
//     let mut backend = MockedBackend::new(width, height);

//     ChartContext {
//         drawing_area: DrawingArea {
//             backend: Rc::new(RefCell::new(backend)),
//             rect: Rect {
//                 x0: 0 as i32,
//                 y0: 0 as i32,
//                 x1: width as i32,
//                 y1: height as i32,
//             },
//             coord: ct,
//         },
//         series_anno: vec![],
//     }
// }

pub fn create_mocked_drawing_area<F: FnOnce(&mut MockedBackend)>(
    width: u32,
    height: u32,
    setup: F,
) -> DrawingArea<MockedBackend, Shift> {
    let mut backend = MockedBackend::new(width, height);
    setup(&mut backend);
    backend.into_drawing_area()
}
