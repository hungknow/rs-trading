use chrono::{DateTime, Utc};

use crate::{
    charts::{
        coord::CoordTranslate,
        elements::CandleStick,
        style::{ShapeStyle, GREEN, RED},
        DrawingBackend,
    },
    data::Candles,
};

// Calculate the width of ohlc
// Create the Ohlc
pub struct Ohlcs {}

impl Ohlcs {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_ohlcs(
        &self,
        from_time: DateTime<Utc>,
        to_time: DateTime<Utc>,
        candles: Candles,
        drawing_width_size: u32,
    ) -> Vec<CandleStick<DateTime<Utc>, f64>> {
        let candle_resolution_seconds = candles.resolution().unwrap().to_seconds();
        let diff_time = ((to_time - from_time).num_seconds() + candle_resolution_seconds)
            / candle_resolution_seconds;
        let candle_step = drawing_width_size as f64 / diff_time as f64;
        let candle_width = candle_step * 0.6;

        let from_open_time_index = candles.find_open_time_index_or_first(from_time).unwrap();
        let to_open_time_index = candles.find_open_time_index_or_last(to_time).unwrap();

        let mut candlestick_elements = vec![];
        for i in from_open_time_index..=to_open_time_index {
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

#[cfg(test)]
mod tests {
    use crate::data::Resolution;

    use super::*;

    #[test]
    fn test_get_ohlcs() {
        let ohlcs = Ohlcs::new();
        let bar_count = 10;
        let drawing_width = 400;
        let expected_candle_width = 21; // (400 / (10 + 1)) * 0.6 = 21
        let resolution_seconds = Resolution::M5.to_seconds();
        let from_time = DateTime::<Utc>::from_timestamp(0, 0).unwrap();
        // 10 bars
        let to_time = DateTime::<Utc>::from_timestamp(resolution_seconds * bar_count, 0).unwrap();
        let mut candles = Candles::new();
        for i in 0..bar_count {
            let open_time = DateTime::<Utc>::from_timestamp(resolution_seconds * i, 0).unwrap();
            let _ = candles.push_data_non_overlapped(open_time, 0.0, 0.0, 0.0, 0.0, None);
        }
        let candlesticks = ohlcs.get_ohlcs(from_time, to_time, candles, drawing_width);
        assert_eq!(candlesticks.len(), bar_count as usize);
        for candlestick in candlesticks {
            assert_eq!(candlestick.width, expected_candle_width);
        }
    }
}
