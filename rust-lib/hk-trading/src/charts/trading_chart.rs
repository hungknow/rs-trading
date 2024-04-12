use crate::charts::overlays::OverlayDrawing;
use chrono::{DateTime, Duration, Utc};

use crate::{
    data::{symbol::SymbolIdentity, Candles, Resolution},
    indicators::{ExponentialMovingAverage, IndicatorContainer},
};

use super::{
    context::ChartContext,
    coord::{
        cartesian::Cartesian2d,
        types::{RangedCoordf64, RangedDateTime},
    },
    drawing::DrawingAreaErrorKind,
    overlays::ohlcs::Ohlcs,
    DrawingBackend,
};

pub struct TradingChartData {
    pub display_time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub symbol_identity: Option<SymbolIdentity>,
    pub resolution: Option<Resolution>,

    // Overlays
    pub ohlc_overlay: Option<Box<Ohlcs>>,
    pub ema_overlay: Option<Box<IndicatorContainer<ExponentialMovingAverage>>>,
}

fn calculate_from_to(
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    resolution: Resolution,
    ohlc_overlay: Option<&Box<Ohlcs>>,
) -> (DateTime<Utc>, DateTime<Utc>) {
    // If there's ohlc data, try to use it to limit the range
    let min_time = DateTime::<Utc>::from_timestamp(0, 0).unwrap();
    // 2050-1-1 00:00:00
    let max_time = DateTime::<Utc>::from_timestamp(2524608000, 0).unwrap();

    let mut max_of_from = max_time;
    let mut min_of_to = min_time;
    if let Some(ohlc) = ohlc_overlay {
        if ohlc.candles.open_times.len() > 1 {
            let from_time_data = if ohlc.candles.time_desc().unwrap() {
                *ohlc.candles.open_times.first().unwrap()
            } else {
                *ohlc.candles.open_times.last().unwrap()
            };
            let to_time_data = if ohlc.candles.time_desc().unwrap() {
                *ohlc.candles.open_times.last().unwrap()
            } else {
                *ohlc.candles.open_times.first().unwrap()
            };

            let diff_duration = Duration::seconds(resolution.to_seconds() * 5);
            max_of_from = from_time_data - diff_duration;
            min_of_to = to_time_data + diff_duration;
        }
    }
    let time_range_from = from.clamp(min_time, max_of_from);
    let time_range_to = to.clamp(min_of_to, max_time);

    (time_range_from, time_range_to)
}

impl TradingChartData {
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
    pub fn push_candles(&mut self, new_candles: &Candles) {
        self.ohlc_overlay
            .as_mut()
            .unwrap()
            .candles
            .merge_candles(new_candles);
        // if let Some(candles) = self.ohlc_overlay {
        //     candles.merge_candles(new_candles);
        // } else {
        //     self.ohlc_overlay = Some(Box::new(new_candles.clone()));
        // }
    }

    pub fn change_display_time_range(&mut self, from: DateTime<Utc>, to: DateTime<Utc>) {
        let (time_range_from, time_range_to) = calculate_from_to(
            from,
            to,
            self.resolution.unwrap(),
            self.ohlc_overlay.as_ref(),
        );

        self.display_time_range = Some((time_range_from, time_range_to));
    }

    pub fn draw<'a, DB>(
        &mut self,
        chart_context: &mut ChartContext<
            'a,
            DB,
            Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordf64>,
        >,
    ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
    where
        DB: DrawingBackend,
    {
        // Draw ohlc
        if let Some(ohlc_overlay) = self.ohlc_overlay.as_mut() {
            ohlc_overlay.draw(chart_context)?;
        }

        /*
           Overlays
        */
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::charts::drawing::{create_mocked_drawing_area, MockedBackend};

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
            DateTime::<Utc>::from_timestamp(1000 + resolution_5m_seconds * 5, 0).unwrap()
        );
        assert_eq!(to, DateTime::<Utc>::from_timestamp(0, 0).unwrap());
    }

    #[test]
    fn test_draw() {
        let mut trading_chart = TradingChartData {
            display_time_range: None,
            symbol_identity: None,
            resolution: None,
            ohlc_overlay: None,
            ema_overlay: None,
        };
        // prepare ohlc data
        let from_time = DateTime::<Utc>::from_timestamp(0, 0).unwrap();
        let to_time = DateTime::<Utc>::from_timestamp(1000, 0).unwrap();

        // create backend for drawing
        let mut mocked_backend =
            create_mocked_drawing_area(1000, 500, |_| {}).apply_coord_spec(Cartesian2d::<
                RangedDateTime<DateTime<Utc>>,
                RangedCoordf64,
            >::new(
                from_time..to_time,
                0.0..200.0,
                (0..1024, 0..768),
            ));
        // create chart context
        let mut chart_context = ChartContext{
            drawing_area: mocked_backend,
            series_anno: vec![],
        };
        // draw
        trading_chart.draw(&mut chart_context).unwrap();
    }
}
