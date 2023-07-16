use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

// The kind of data emitted by the data generator
#[derive(Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum DataKind {
    // Trade(PublicTrade),
    // OrderBookL1(OrderBookL1),
    // OrderBook(OrderBook),
    Candle(Candle),
    // Liquidation(Liquidation),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct MarketEvent<T> {
    pub exchange_time: DateTime<Utc>,
    pub received_time: DateTime<Utc>,
    pub kind: T,
}