use chrono::{DateTime, Utc};

mod candle;
pub use candle::*;

pub mod historical;
pub mod traits;

/// Metadata detailing the Candle or Trade.
pub struct MarketMeta {
    /// Close value from the source market event.
    pub close: f64,
    /// Exchange timestamp from the source market event.
    pub time: DateTime<Utc>,
}