use std::{ops::Deref, rc::Rc};

use chrono::{DateTime, Utc};

use crate::{
    charts::{
        context::ChartContext,
        coord::{
            cartesian::Cartesian2d,
            ranged1d::Ranged,
            types::{RangedCoordf64, RangedDateTime},
        },
        drawing::DrawingAreaErrorKind,
        elements::CandleStick,
        style::{ShapeStyle, GREEN, RED},
        DrawingBackend,
    },
    data::{Candle, Candles},
};

use super::OverlayDrawing;

// Calculate the width of ohlc
// Create the Ohlc
pub struct Ohlcs {
    pub(crate) candles: Candles,
    pub(crate) from_time: Option<DateTime<Utc>>,
    pub(crate) to_time: Option<DateTime<Utc>>,
    pub(crate) drawing_area_width: u32,
    pub(crate) candlesticks: Rc<Vec<CandleStick<DateTime<Utc>, f64>>>,
}

impl Ohlcs {
    pub fn new(candles: &Candles) -> Self {
        if candles.open_times.is_empty() {
            return Self {
                candles: Candles::new(),
                from_time: None,
                to_time: None,
                drawing_area_width: 0,
                candlesticks: Rc::new(vec![]),
            };
        }
        let from_time = *candles.open_times.first().unwrap();
        let to_time = *candles.open_times.last().unwrap();
        let candlesticks = Self::get_ohlcs(from_time, to_time, &candles, 400);
        Self {
            candles: candles.clone(),
            from_time: Some(from_time),
            to_time: Some(to_time),
            drawing_area_width: 0,
            candlesticks: Rc::new(candlesticks),
        }
    }

    pub fn get_new_ohlcs(
        &mut self,
        from_time: DateTime<Utc>,
        to_time: DateTime<Utc>,
        candles: Candles,
        drawing_area_width: u32,
    ) -> Rc<Vec<CandleStick<DateTime<Utc>, f64>>> {
        let now_time = Utc::now();
        if from_time == self.from_time.unwrap_or(now_time)
            && to_time == self.to_time.unwrap_or(now_time)
            && drawing_area_width == self.drawing_area_width
        {
            return self.candlesticks.clone();
        }

        // calculate new ohlcs
        let candlesticks = Rc::new(Self::get_ohlcs(
            from_time,
            to_time,
            &candles,
            drawing_area_width,
        ));
        self.candlesticks = candlesticks.clone();

        candlesticks
    }

    pub fn get_ohlcs(
        from_time: DateTime<Utc>,
        to_time: DateTime<Utc>,
        candles: &Candles,
        drawing_area_width: u32,
    ) -> Vec<CandleStick<DateTime<Utc>, f64>> {
        let candle_resolution_seconds = candles.resolution().unwrap().to_seconds();
        let diff_time = ((to_time - from_time).num_seconds() + candle_resolution_seconds)
            / candle_resolution_seconds;
        let candle_step = drawing_area_width as f64 / diff_time as f64;
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

impl<DB: DrawingBackend>
    OverlayDrawing<DB, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf64>> for Ohlcs
{
    fn draw<'a>(
        &mut self,
        chart_context: &mut ChartContext<
            'a,
            DB,
            Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf64>,
        >,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>> {
        // Draw ohlc
        let candlesticks = self.candlesticks.to_vec();
        chart_context.draw_series(candlesticks)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::Resolution;

    use super::*;

    #[test]
    fn test_get_ohlcs() {
        let ohlcs = Ohlcs::new(Candles::new());
        let bar_count = 10;
        let drawing_area_width = 400;
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
        let candlesticks = Ohlcs::get_ohlcs(from_time, to_time, &candles, drawing_area_width);
        assert_eq!(candlesticks.len(), bar_count as usize);
        for candlestick in candlesticks {
            assert_eq!(candlestick.width, expected_candle_width);
        }
    }
}
