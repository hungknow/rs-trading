use std::{
    cell::{RefCell, RefMut},
    ops::Deref,
    rc::Rc,
};

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
    pub(crate) candles: Option<RefCell<Candles>>,
    pub(crate) from_time: Option<DateTime<Utc>>,
    pub(crate) to_time: Option<DateTime<Utc>>,
    pub(crate) drawing_area_width: u32,
    pub(crate) candlesticks: Option<Rc<Vec<CandleStick<DateTime<Utc>, f64>>>>,

    previous_from_time: Option<DateTime<Utc>>,
    previous_to_time: Option<DateTime<Utc>>,
}

impl Ohlcs {
    pub fn new() -> Self {
        return Self {
            candles: None,
            from_time: None,
            to_time: None,
            drawing_area_width: 0,
            candlesticks: None,
            previous_from_time: None,
            previous_to_time: None,
        };
    }

    pub fn from_time(&mut self, from_time: DateTime<Utc>) -> &mut Self {
        self.from_time = Some(from_time);
        self
    }

    pub fn to_time(&mut self, to_time: DateTime<Utc>) -> &mut Self {
        self.to_time = Some(to_time);
        self
    }

    pub fn drawing_area_width(&mut self, drawing_area_width: u32) -> &mut Self {
        self.drawing_area_width = drawing_area_width;
        self
    }

    pub fn candles(&mut self, candles: RefCell<Candles>) -> &mut Self {
        self.candles = Some(candles.clone());
        self
    }

    // pub fn get_new_ohlcs(
    //     &mut self,
    //     from_time: DateTime<Utc>,
    //     to_time: DateTime<Utc>,
    //     candles: Candles,
    //     drawing_area_width: u32,
    // ) -> Rc<Vec<CandleStick<DateTime<Utc>, f64>>> {
    //     let now_time = Utc::now();
    //     if from_time == self.from_time.unwrap_or(now_time)
    //         && to_time == self.to_time.unwrap_or(now_time)
    //         && drawing_area_width == self.drawing_area_width
    //     {
    //         return self.candlesticks.clone();
    //     }

    //     // calculate new ohlcs
    //     let candlesticks = Rc::new(Self::get_ohlcs(
    //         from_time,
    //         to_time,
    //         &candles,
    //         drawing_area_width,
    //     ));
    //     self.candlesticks = candlesticks.clone();

    //     candlesticks
    // }

    pub fn get_ohlcs(
        from_time: DateTime<Utc>,
        to_time: DateTime<Utc>,
        candles: RefMut<Candles>,
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
        if self.candles.is_none() {
            return Ok(());
        }
        if (!self.from_time.is_none() && self.previous_from_time != self.from_time)
            || (!self.to_time.is_none() && self.previous_to_time != self.to_time)
        {
            self.candlesticks = Some(Rc::new(Self::get_ohlcs(
                self.from_time.unwrap(),
                self.to_time.unwrap(),
                self.candles.as_mut().unwrap().borrow_mut(),
                self.drawing_area_width,
            )));
            self.previous_from_time = self.from_time;
            self.previous_to_time = self.to_time;
        }
        // Draw ohlc
        if let Some(candlesticks) = self.candlesticks.as_mut() {
            let candlesticks = candlesticks.to_vec();
            chart_context.draw_series(candlesticks)?;
        }

        Ok(())
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
        let candlesticks = Ohlcs::get_ohlcs(
            from_time,
            to_time,
            RefCell::new(candles).borrow_mut(),
            drawing_area_width,
        );
        assert_eq!(candlesticks.len(), bar_count as usize);
        for candlestick in candlesticks {
            assert_eq!(candlestick.width, expected_candle_width);
        }
    }
}
