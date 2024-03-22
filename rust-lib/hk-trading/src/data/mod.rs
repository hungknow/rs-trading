use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod candle;
pub use candle::*;

mod data_source;
pub use data_source::*;
// mod csv_data_source;
// pub use csv_data_source::*;

pub mod historical;
mod hk_trading_candles_data_source;
pub use hk_trading_candles_data_source::*;
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
    TimestampValue(TimestampValue),
    // Liquidation(Liquidation),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct DataEvent<T> {
    pub kind: T,
    // pub exchange_time: Option<DateTime<Utc>>,
    // pub received_time: Option<DateTime<Utc>>,
}