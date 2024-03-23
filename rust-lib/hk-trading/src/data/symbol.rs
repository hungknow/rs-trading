use super::Resolution;

pub struct Symbol {
    /*
    The name property is an identifier for a symbol within an exchange, such as AAPL or 9988 (on Hong Kong exchange).
    This identifier is visible to users.
    Note that the name value does not have to be unique and can be duplicated for several symbols.
    */
    pub name: String,
    /*
    If you need to address a symbol by a custom identifier (for example, numeric), you can use ticker.
    This identifier should be unique. It is not displayed to users.
    */
    pub ticker: String,
    pub supported_resolutions: Vec<Resolution>,
    // "futures"/"crypto"/"forex"/"index"
    pub symbol_type: String,
    
    // "ohlc"/"line"
    pub chart_data_types: Vec<String>,
}
