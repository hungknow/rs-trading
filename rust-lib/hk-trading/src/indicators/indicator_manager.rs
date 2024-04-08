use std::collections::HashMap;

use crate::data::symbol::SymbolIdentity;

use super::traits::{Indicator, IndicatorContainer, IndicatorType};

pub struct IndicatorManager {
    // The map 
    // map: HashMap<(SymbolIdentity, IndicatorType), IndicatorState<dyn Indicator>>,
}

impl IndicatorManager {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn get_indicator(&self, symbol_identity: SymbolIdentity, indicator_type: IndicatorType) -> Option<&IndicatorContainer<dyn Indicator>> {
        // self.map.get(&(symbol_identity, indicator_type)
        match (symbol_identity, indicator_type) {
            (SymbolIdentity::Symbol(symbol), IndicatorType::RSI) => {
                // let indicator = RSI::new();
                // let state = IndicatorState::new(indicator, symbol_identity);
                // self.map.insert((symbol_identity, indicator_type), state);
                // Some(&state)
                None
            }
            _ => None,
        }
    }
}
