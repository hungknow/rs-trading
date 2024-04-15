use std::collections::VecDeque;

use crate::charts::{
    coord::Shift,
    drawing::area::{DrawingArea, IntoDrawingArea},
    style::RGBAColor,
    BackendCoord, BackendStyle, DrawingBackend, DrawingErrorKind,
};

pub struct MockedBackend {
    height: u32,
    width: u32,
    init_count: u32,
    pub draw_count: u32,
    pub num_draw_line_call: u32,
    pub num_draw_rect_call: u32,
    check_draw_line: VecDeque<Box<dyn FnMut(RGBAColor, u32, BackendCoord, BackendCoord)>>,
    check_draw_rect: VecDeque<Box<dyn FnMut(RGBAColor, u32, bool, BackendCoord, BackendCoord)>>,
    drop_check: Option<Box<dyn FnMut(&Self)>>,
}

macro_rules! def_set_checker_func {
    (drop_check, $($param:ty),*) => {
        pub fn drop_check<T: FnMut($($param,)*) + 'static>(&mut self, check:T) -> &mut Self {
            self.drop_check = Some(Box::new(check));
            self
        }
    };
    ($name:ident, $($param:ty),*) => {
        pub fn $name<T: FnMut($($param,)*) + 'static>(&mut self, check:T) -> &mut Self {
            self.$name.push_back(Box::new(check));
            self
        }
    }
}

impl MockedBackend {
    pub fn new(width: u32, height: u32) -> Self {
        MockedBackend {
            height,
            width,
            init_count: 0,
            draw_count: 0,
            num_draw_line_call: 0,
            num_draw_rect_call: 0,
            check_draw_line: vec![].into(),
            check_draw_rect: vec![].into(),
            drop_check: None,
        }
    }

    def_set_checker_func!(check_draw_line, RGBAColor, u32, BackendCoord, BackendCoord);
    def_set_checker_func!(
        check_draw_rect,
        RGBAColor,
        u32,
        bool,
        BackendCoord,
        BackendCoord
    );
    def_set_checker_func!(drop_check, &Self);
}

impl Drop for MockedBackend {
    fn drop(&mut self) {
        // `self.drop_check` is typically a testing function; it can panic.
        // The current `drop` call may be a part of stack unwinding caused
        // by another panic. If so, we should never call it.
        if std::thread::panicking() {
            return;
        }

        let mut temp = None;
        std::mem::swap(&mut temp, &mut self.drop_check);

        if let Some(mut checker) = temp {
            checker(self);
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

impl MockedBackend {
    fn check_before_draw(&mut self) {
        self.draw_count += 1;
        //assert_eq!(self.init_count, self.draw_count);
    }
}

impl DrawingBackend for MockedBackend {
    type ErrorType = MockedError;

    fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn ensure_prepared(&mut self) -> Result<(), crate::charts::DrawingErrorKind<Self::ErrorType>> {
        self.init_count += 1;
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<MockedError>> {
        self.init_count = 0;
        self.draw_count = 0;
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.check_before_draw();
        self.num_draw_line_call += 1;
        let color = style.color();
        let color = RGBAColor(color.rgb.0, color.rgb.1, color.rgb.2, color.alpha);
        if let Some(mut checker) = self.check_draw_line.pop_front() {
            checker(color, style.stroke_width(), from, to);

            if self.check_draw_line.is_empty() {
                self.check_draw_line.push_back(checker);
            }
        }
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        self.check_before_draw();
        self.num_draw_rect_call += 1;
        let color = style.color();
        let color = RGBAColor(color.rgb.0, color.rgb.1, color.rgb.2, color.alpha);
        if let Some(mut checker) = self.check_draw_rect.pop_front() {
            checker(color, style.stroke_width(), fill, upper_left, bottom_right);

            if self.check_draw_rect.is_empty() {
                self.check_draw_rect.push_back(checker);
            }
        }
        Ok(())
    }
}

pub fn create_mocked_drawing_area<F: FnOnce(&mut MockedBackend)>(
    width: u32,
    height: u32,
    setup: F,
) -> DrawingArea<MockedBackend, Shift> {
    let mut backend = MockedBackend::new(width, height);
    setup(&mut backend);
    backend.into_drawing_area()
}
