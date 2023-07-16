use std::collections::HashMap;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{indicators::RelativeStrengthIndex, data::{DataEvent, DataKind}, Next};

use super::{SignalGenerator, SignalEvent, Decision, SignalStrength};

/// Configuration for constructing a [`RSIStrategy`] via the new() constructor method.
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Debug, Deserialize, Serialize)]
pub struct StrategyRsiConfig {
    pub rsi_period: usize,
}

#[derive(Clone, Debug)]
pub struct RsiStrategy {
    rsi: RelativeStrengthIndex,
}

impl RsiStrategy {
    /// Constructs a new [`RSIStrategy`] component using the provided configuration struct.
    pub fn new(config: StrategyRsiConfig) -> Self {
        let rsi_indicator = RelativeStrengthIndex::new(config.rsi_period)
            .expect("Failed to construct RSI indicator");

        Self { rsi: rsi_indicator }
    }

    fn generate_signals_map(rsi: f64) -> HashMap<Decision, SignalStrength> {
        let mut signals = HashMap::with_capacity(4);

        if rsi < 40.0 {
            signals.insert(Decision::Long, RsiStrategy::calculate_signal_strength());
        }
        if rsi > 60.0 {
            signals.insert(
                Decision::CloseLong,
                RsiStrategy::calculate_signal_strength(),
            );
        }
        if rsi > 60.0 {
            signals.insert(Decision::Short, RsiStrategy::calculate_signal_strength());
        }
        if rsi < 40.0 {
            signals.insert(
                Decision::CloseShort,
                RsiStrategy::calculate_signal_strength(),
            );
        }

        signals
    }

    fn calculate_signal_strength() -> SignalStrength {
        SignalStrength(1.0)
    }
}

impl SignalGenerator for RsiStrategy {
    fn generate_signal(&mut self, data_event: &DataEvent<DataKind>) -> Option<SignalEvent<DataKind>> {
        let candle_close = match &data_event.kind {
            DataKind::Candle(candle) => candle.close,
            _ => return None,
        }; 

        // Calculate the next RSI value using the new MarketEvent Candle data
        let rsi = self.rsi.next(candle_close);

         // Generate advisory signals map
         let signals = RsiStrategy::generate_signals_map(rsi);

         // If signals map is empty, return no SignalEvent
         if signals.is_empty() {
             return None;
         }

        Some(SignalEvent {
            created_at: Utc::now(),
            data_event: data_event.clone(),
            // market_meta: MarketMeta {
            //     close: candle_close,
            //     time: market.exchange_time,
            // },
            signals,
        })
    }
}