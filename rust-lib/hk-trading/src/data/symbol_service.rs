use crate::errors::HkError;
use super::symbol::Symbol;

pub struct SearchSymbols {
    pub query: String,
    pub is_loading: bool,
    pub symbols: Vec<Symbol>,
}

pub struct SymbolService {
    // Store the list of common symbols
    pub common_symbols: Vec<Symbol>,
    // Store the result of the search for symbol
    pub search_symbols: Option<SearchSymbols>,
}

impl SymbolService {
    pub fn new() -> Self {
        SymbolService {
            common_symbols: Vec::new(),
            search_symbols: None,
        }
    }

    // Load the common symbols
    pub fn search_symbols(&self, query: &str) -> Result<Vec<Symbol>, HkError> {
        // call the search API
        Err(HkError::InvalidParameter)
    }
}
