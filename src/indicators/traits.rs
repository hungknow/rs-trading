use std::{rc::Rc, sync::mpsc};

use chrono::{DateTime, Utc};

use crate::Reset;

pub trait Indicator {
    type InputType: Clone;
    type OutputType: Clone;
    type StateType: Clone + Reset;

    // fn new_state(&self) -> Rc<Self::StateType>;
    fn next(
        &self,
        input: (&Self::StateType, Self::InputType),
    ) -> (Self::StateType, Self::OutputType);

    // fn get_event_receiver() -> mpsc::Receiver<Self::StateType>;
}

pub struct IndicatorContainer<T: Indicator> {
    indicator: T,
    state: Rc<T::StateType>,
}

// Contains the value of TA
// and their time range
pub struct TATimeAware<T: Indicator> {
    state: Rc<T::StateType>,
    values: Vec<T::OutputType>,
    timestamp: Vec<DateTime<Utc>>,
}
