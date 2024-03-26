use charming::{
    component::{Axis, DataZoom, Grid, Legend},
    element::{
        AreaStyle, AxisPointer, AxisPointerType, AxisType, DataBackground, LineStyle, SplitLine,
        TextStyle, Tooltip, Trigger,
    },
    series::{Candlestick, Line},
    Chart,
};
use hk_trading::data::Candles;

pub fn ohlc_gen(candles: &Candles) -> Chart {
    let dates: Vec<String> = candles.open_times.iter().map(|row| row.to_rfc3339()).collect();
    let data: Vec<Vec<f64>> = candles.open_times
        .iter()
        .enumerate()
        .map(|(row_idx, open_time)| {
            vec![
                candles.opens[row_idx],
                candles.closes[row_idx],
                candles.lows[row_idx],
                candles.highs[row_idx],
            ]
        })
        .collect();

    Chart::new()
        .legend(
            Legend::new()
                .inactive_color("#777")
                .data(vec!["日K"]),
        )
        .tooltip(
            Tooltip::new().trigger(Trigger::Axis).axis_pointer(
                AxisPointer::new()
                    .animation(false)
                    .type_(AxisPointerType::Cross)
                    .line_style(LineStyle::new().color("#376df4").width(2).opacity(1)),
            ),
        )
        .x_axis(Axis::new().type_(AxisType::Category).data(dates))
        .y_axis(
            Axis::new()
                .scale(true)
                .split_line(SplitLine::new().show(false)),
        )
        .grid(Grid::new().bottom(80))
        .data_zoom(
            DataZoom::new()
                .handle_icon(ICON)
                .text_style(TextStyle::new().color("#8392A5"))
                .data_background(
                    DataBackground::new()
                        .area_style(AreaStyle::new().color("#8392A5"))
                        .line_style(LineStyle::new().color("#8392A5").opacity(0.8)),
                )
                .brush_select(true),
        )
        .series(Candlestick::new().name("Day").data(data.clone()))
}

static ICON: &str = "path://M10.7,11.9v-1.3H9.3v1.3c-4.9,0.3-8.8,4.4-8.8,9.4c0,5,3.9,9.1,8.8,9.4v1.3h1.3v-1.3c4.9-0.3,8.8-4.4,8.8-9.4C19.5,16.3,15.6,12.2,10.7,11.9z M13.3,24.4H6.7V23h6.6V24.4z M13.3,19.6H6.7v-1.4h6.6V19.6z";
