use std::rc::Rc;

use chrono::{DateTime, Utc};

pub trait Indicator {
    type InputType: Clone;
    type OutputType: Clone;
    type StateType;

    // fn new_state(&self) -> Rc<Self::StateType>;
    fn next(
        &self,
        input: (&Self::StateType, Self::InputType),
    ) -> (Self::StateType, Self::OutputType);
}

// Contains the value of TA
// and their time range
struct TATimeAware<T: Indicator> {
    state: Rc<T::StateType>,
    values: Vec<T::OutputType>,
    timestamp: Vec<DateTime<Utc>>,
}
