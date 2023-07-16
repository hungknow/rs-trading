mod rsi_strategy;
pub use rsi_strategy::StrategyRsi;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::data::{DataKind, MarketEvent};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum Decision {
    Long,
    CloseLong,
    Short,
    CloseShort,
}

pub trait SignalGenerator {
    /// Optionally return a [`Signal`] given input [`MarketEvent`].
    fn generate_signal(&mut self, market: &MarketEvent<DataKind>) -> Option<Signal>;
}

/// Strength of an advisory [`Signal`] decision produced by [`SignalGenerator`] strategy.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct SignalStrength(pub f64);

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Signal {
    pub time: DateTime<Utc>,
    pub signals: HashMap<Decision, SignalStrength>,
    // Metadata propagated from the [`MarketEvent`] that yielded this [`Signal`].
    // pub market_meta: MarketMeta,
}