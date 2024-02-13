use std::rc::Rc;

use chrono::{DateTime, Utc};

use crate::data::TimestampValue;

pub trait Reset {
    fn reset(&mut self);
}

/// Return the period used by the indicator.
pub trait Period {
    fn period(&self) -> usize;
}

pub trait Next<T> {
    type Output;
    fn next(&self, input: T) -> Self::Output;
}

// pub trait TA {
//     type ValueType: Clone;
//     type StateType;

//     // fn new_state(&self) -> Rc<Self::StateType>;
//     fn next(
//         &self,
//         input: (&Self::StateType, Self::ValueType),
//     ) -> (Self::StateType, Self::ValueType);
// }

/// Close price of a particular period.
pub trait Close {
    fn close(&self) -> f64;
}

pub trait TimestampValueDS<T: Clone> {
    fn timestamp(&self) -> &[DateTime<Utc>];
    fn value(&self) -> &[T];
}

// impl dyn TimestampValueDS {
//     pub fn get_at_index(&self, index: usize) -> TimestampValue {
//         let timestamp = &self.timestamp()[index];
//         let value = &self.value()[index];
//         TimestampValue {
//             timestamp: *timestamp,
//             value: *value,
//         }
//     }
// }

