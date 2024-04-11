use chrono::{DateTime, Utc};

use crate::{
    charts::{
        coord::CoordTranslate,
        elements::CandleStick,
        style::{ShapeStyle, GREEN, RED},
        DrawingBackend,
    },
    data::{Candle, Candles},
};

// Calculate the width of ohlc
// Create the Ohlc
pub struct Ohlcs {}

impl Ohlcs {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_ohlcs<DB: DrawingBackend, CT: CoordTranslate>(
        &self,
        from_time: DateTime<Utc>,
        to_time: DateTime<Utc>,
        candles: Candles,
        db: DB,
    ) -> Vec<CandleStick<DateTime<Utc>, f64>> {
        let (w, _) = db.get_size();
        let diff_time = (to_time - from_time).num_seconds();
        let candle_step = w as f64 / diff_time as f64;
        let candle_width = candle_step * 0.6;

        let from_open_time_index = candles.find_open_time_index_or_first(from_time).unwrap();
        let to_open_time_index = candles.find_open_time_index_or_last(to_time).unwrap();

        let mut candlestick_elements = vec![];
        for i in from_open_time_index..to_open_time_index {
            let c = CandleStick::new(
                candles.open_times[i],
                candles.opens[i],
                candles.highs[i],
                candles.lows[i],
                candles.closes[i],
                Into::<ShapeStyle>::into(&GREEN).filled(),
                Into::<ShapeStyle>::into(&RED).filled(),
                candle_width as u32,
            );
            candlestick_elements.push(c);
        }

        candlestick_elements
    }
}
