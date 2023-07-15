pub trait Reset {
    fn reset(&mut self);
}

/// Return the period used by the indicator.
pub trait Period {
    fn period(&self) -> usize;
}

pub trait Next<T> {
    type Output;
    fn next(&mut self, input: T) -> Self::Output;
}

/// Close price of a particular period.
pub trait Close {
   fn close(&self) -> f64; 
}