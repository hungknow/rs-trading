use std::sync::Arc;

use chrono::{DateTime, Utc};

use super::traits::Indicator;

pub struct IndicatorContainer<T: Indicator> {
    pub indicator: T,
    pub state: Arc<T::StateType>,
    pub output: Vec<T::OutputType>,
    pub timestamp: Vec<DateTime<Utc>>,
}
