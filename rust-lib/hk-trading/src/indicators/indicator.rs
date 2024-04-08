use std::sync::Arc;

use chrono::{DateTime, Utc};

use crate::data::symbol::SymbolIdentity;

use super::traits::Indicator;

pub struct IndicatorContainer<T: Indicator> {
    pub indicator: T,
    pub symbol_identity: SymbolIdentity,
    pub state: T::StateType,
    pub timestamp: Vec<DateTime<Utc>>,
    pub values: Vec<T::OutputType>,
}

// #[derive(Clone, Default, PartialEq, Debug)]
// pub struct IndicatorData<T: Indicator> {
//     pub state: Arc<T::StateType>,
//     pub output: Vec<T::OutputType>,
//     pub timestamp: Vec<DateTime<Utc>>,
// }

impl<T: Indicator> IndicatorContainer<T> {
    pub fn new(indicator: T, symbol_identity: SymbolIdentity) -> Self {
        Self {
            indicator,
            symbol_identity,
            state: T::StateType::default(),
            timestamp: Vec::new(),
            values: Vec::new(),
        }
    }
}
