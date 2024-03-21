use crate::Close;

#[derive(Debug, PartialEq)]
pub struct Bar {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            open: 0.0,
            close: 0.0,
            low: 0.0,
            high: 0.0,
            volume: 0.0,
        }
    }

    pub fn close<T: Into<f64>>(mut self, val: T) -> Self {
        self.close = val.into();
        self
    }
}

impl Close for Bar {
    fn close(&self) -> f64 {
        self.close
    }
}
