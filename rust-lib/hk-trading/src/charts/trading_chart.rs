use crate::{
    data::{symbol::SymbolIdentity, Candles},
    indicators::{ExponentialMovingAverage, IndicatorContainer},
};

pub struct TradingChartData {
    pub symbol_identity: SymbolIdentity,
    pub ohlc_overlay: Option<Box<Candles>>,
    pub ema_overlay: Option<Box<IndicatorContainer<ExponentialMovingAverage>>>,
}

impl TradingChartData {
    // pub fn on_range_changed()
}
