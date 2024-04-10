pub struct Grid {
    /// Grid ID
    pub id: i64,
    /// Upper bound of price-range
    pub y_hi: f64,
    /// Upper bound of price-range
    pub y_lo: f64,
    /// Grid price step
    pub y_step: f64,
    /// Grid time step
    pub x_step: f64,
    /// Scale transform coefficient
    pub A: f64,
    pub B: f64,

    /// Grid height (px)
    pub height: i64,
    /// Grid width (without sidebar, px) 
    pub width: i64,
}

// Grid contains list of Overlays