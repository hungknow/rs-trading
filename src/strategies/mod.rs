mod rsi_strategy;
pub use rsi_strategy::{RsiStrategy, StrategyRsiConfig};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::data::{DataKind, DataEvent};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum Decision {
    Long,
    CloseLong,
    Short,
    CloseShort,
}

pub trait SignalGenerator {
    /// Optionally return a [`SignalEvent`] given input [`MarketEvent`].
    fn generate_signal(&mut self, data_event: &DataEvent<DataKind>) -> Option<SignalEvent<DataKind>>;
}

/// Strength of an advisory [`Signal`] decision produced by [`SignalGenerator`] strategy.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct SignalStrength(pub f64);

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct SignalEvent<DataKind> {
    pub created_at: DateTime<Utc>,
    pub data_event: DataEvent<DataKind>,
    pub signals: HashMap<Decision, SignalStrength>,
    // Metadata propagated from the [`MarketEvent`] that yielded this [`Signal`].
    // pub market_meta: MarketMeta,
}