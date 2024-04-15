use std::sync::Arc;

use crate::charts::overlays::OverlayData;
use chrono::{DateTime, Duration, Utc};

use crate::{
    data::{symbol::SymbolIdentity, Resolution},
    indicators::{ExponentialMovingAverage, IndicatorContainer},
};

use super::{
    context::{ChartContext, TradingChartContext},
    coord::{
        cartesian::Cartesian2d,
        types::{RangedCoordf64, RangedDateTime},
        CoordTranslate,
    },
    drawing::DrawingAreaErrorKind,
    overlays::{OhlcOverlay, Overlay},
    DrawingBackend,
};

// One Chart panel in multi chart
pub struct TradingChartData<DB: DrawingBackend, CT: CoordTranslate> {
    pub display_time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub symbol_identity: Option<SymbolIdentity>,
    pub resolution: Option<Resolution>,

    // Overlays
    pub on_chart: Vec<Box<dyn Overlay<DB, CT>>>,
    pub off_chart: Vec<Box<dyn Overlay<DB, CT>>>,
    // pub ohlc_overlay: Option<OhlcOverlay>,
    // pub ema_overlay: Option<IndicatorContainer<ExponentialMovingAverage>>,
}

fn calculate_from_to(
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    resolution: Resolution,
    ohlc_overlay: Option<&OhlcOverlay>,
) -> (DateTime<Utc>, DateTime<Utc>) {
    // If there's ohlc data, try to use it to limit the range
    let min_time = DateTime::<Utc>::from_timestamp(0, 0).unwrap();
    // 2050-1-1 00:00:00
    let max_time = DateTime::<Utc>::from_timestamp(2524608000, 0).unwrap();

    let mut max_of_from = max_time;
    let mut min_of_to = min_time;
    if let Some(candles) = ohlc_overlay.unwrap().overlay_data() {
        // if let Some(candles) = ohlc.candles.clone() {
        // let candles = candles_ref_cell.borrow();
        if candles.open_times.len() > 1 {
            let from_time_data = if candles.time_desc().unwrap() {
                *candles.open_times.first().unwrap()
            } else {
                *candles.open_times.last().unwrap()
            };
            let to_time_data = if candles.time_desc().unwrap() {
                *candles.open_times.last().unwrap()
            } else {
                *candles.open_times.first().unwrap()
            };

            let diff_duration = Duration::seconds(resolution.to_seconds() * 5);
            max_of_from = from_time_data - diff_duration;
            min_of_to = to_time_data + diff_duration;
        }
        // }
    }
    let time_range_from = from.clamp(min_time, max_of_from);
    let time_range_to = to.clamp(min_of_to, max_time);

    (time_range_from, time_range_to)
}

impl<DB: DrawingBackend, CT: CoordTranslate> TradingChartData<DB, CT> {
    pub fn new() -> Self {
        Self {
            display_time_range: None,
            symbol_identity: None,
            resolution: None,
            on_chart: vec![],
            off_chart: vec![],
        }
    }

    pub fn add_on_chart_overlay(&mut self, overlay: Box<dyn Overlay<DB, CT>>) -> &mut Self {
        self.on_chart.push(overlay);
        self
    }

    pub fn add_off_chart_overlay(&mut self, overlay: Box<dyn Overlay<DB, CT>>) -> &mut Self {
        self.off_chart.push(overlay);
        self
    }
    

    // pub fn on_range_changed()

    // Pan (start, move, end)

    // Pinch (start, end)

    // Limit crazy wheel delta values
    fn smart_wheel(delta: f64) -> f64 {
        let abs = delta.abs();
        if abs > 500.0 {
            return (200.0 + abs.ln()) * delta.signum();
        }
        return delta;
    }
    pub fn mouse_zoom(&mut self, delta: f64) {
        // let k = self.resolution.to_seconds();
        // let diff = delta * k * self.ohlc_overlay.unwrap().open_times.len();

        // self.change_display_time_range(self.display_time_range.0 - , to)
    }

    // Merge the candles into the existing candles, or set it as the current candles if there's none
    // pub fn push_candles(&mut self, new_candles: &Candles) {
    //     if let Some(ohlcs) = self.ohlc_overlay.as_ref() {
    //         if let Some(candles_ref_cell) = ohlcs.candles.as_ref() {
    //             let mut candles = candles_ref_cell.borrow_mut();
    //             candles.merge_candles(new_candles);
    //         }
    //     }
    // }

    pub fn change_display_time_range(&mut self, from: DateTime<Utc>, to: DateTime<Utc>) {
        // Iterate through all overlays and update the time range
        // for overlay in self.on_chart.iter_mut() {
        //     overlay.overlay_data_mut().unwrap().change_display_time_range(from, to);
        // }
        //     let (time_range_from, time_range_to) = calculate_from_to(
        //         from,
        //         to,
        //         self.resolution.unwrap(),
        //         self.ohlc_overlay.as_ref(),
        //     );

        //     self.display_time_range = Some((time_range_from, time_range_to));
    }

    pub fn draw<'a>(
        &mut self,
        trading_chart_context: &mut TradingChartContext<
            DB,
            CT,
            // Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf64>,
        >,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
    where
        DB: DrawingBackend,
    {
        // The on-chart overlays will be drawed on the main drawing area
        for overlay in self.on_chart.iter_mut() {
            overlay.draw(&mut ChartContext::new(
                &trading_chart_context.main_drawing_area,
            ))?;
        }

        // Each offchart overlay will be drawed on its corresponding offchart drawing area
        for (index, overlay) in self.off_chart.iter_mut().enumerate() {
            overlay.draw(&mut ChartContext::new(
                &trading_chart_context.off_chart_drawing_areas[index]
            ))?;
        }

        // Draw the bottom axis

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        charts::{drawing::create_mocked_drawing_area, ChartBuilder},
        data::Candles,
    };

    use super::*;

    #[test]
    fn test_calculate_from_to() {
        let resolution_5m_seconds = Resolution::M5.to_seconds();
        let (from, to) = calculate_from_to(
            DateTime::<Utc>::from_timestamp(0, 0).unwrap(),
            DateTime::<Utc>::from_timestamp(0, 0).unwrap(),
            Resolution::M5,
            None,
        );
        assert_eq!(from, DateTime::<Utc>::from_timestamp(0, 0).unwrap());
        assert_eq!(to, DateTime::<Utc>::from_timestamp(0, 0).unwrap());

        let (from, to) = calculate_from_to(
            DateTime::<Utc>::from_timestamp(1000, 0).unwrap(),
            DateTime::<Utc>::from_timestamp(2000, 0).unwrap(),
            Resolution::M5,
            None,
        );
        assert_eq!(
            from,
            DateTime::<Utc>::from_timestamp(resolution_5m_seconds * 5, 0).unwrap()
        );
        assert_eq!(to, DateTime::<Utc>::from_timestamp(0, 0).unwrap());
    }

    #[test]
    fn test_draw() {
        let mut trading_chart = TradingChartData::new();

        // prepare ohlc data
        let from_time = DateTime::<Utc>::from_timestamp(0, 0).unwrap();
        let to_time = DateTime::<Utc>::from_timestamp(2, 0).unwrap();
        let mut candles = Candles::new();
        candles
            .push_data_non_overlapped(
                DateTime::<Utc>::from_timestamp(0, 0).unwrap(),
                0.0,
                10.0,
                5.0,
                0.0,
                None,
            )
            .unwrap();
        candles
            .push_data_non_overlapped(
                DateTime::<Utc>::from_timestamp(1, 0).unwrap(),
                0.0,
                200.0,
                100.0,
                0.0,
                None,
            )
            .unwrap();

        let mut ohlcs = OhlcOverlay::new();
        ohlcs.candles(Arc::new(candles));

        // create backend for drawing
        let mut mocked_drawing_area = create_mocked_drawing_area(1000, 500, |_| {});
        // .apply_coord_spec(
        //     Cartesian2d::<RangedDateTime<DateTime<Utc>>, RangedCoordf64>::new(
        //         from_time..to_time,
        //         0.0..200.0,
        //         (0..1024, 0..768),
        //     ),
        // );

        // create chart context
        // let mut chart_context = ChartContext {
        //     drawing_area: mocked_drawing_area,
        //     right_side_bar_area: None,
        // };

        let mut chart_context = ChartBuilder::on(&mocked_drawing_area)
            .build_trading_chart_context(from_time..to_time, 0.0..200.0, 0)
            .unwrap();

        // draw
        trading_chart.add_on_chart_overlay(Box::new(ohlcs));
        trading_chart.draw(&mut chart_context).unwrap();
    }
}
