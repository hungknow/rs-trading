use std::borrow::Borrow;
use std::sync::Arc;

use chrono::Duration;
use hk_trading::charts::drawing::IntoDrawingArea;
use hk_trading::charts::overlays::OhlcOverlay;
use hk_trading::charts::style::BLACK_1;
use hk_trading::charts::TradingChartData;
use hk_trading::charts::{svg_backend::SVGBackend, ChartBuilder};
use hk_trading::data::{CandleCSVDataSource, CandleCSVLoadOption};
use hk_trading::path_utils::{get_hk_trading_file_path, DATA_HISTORIC_XAUUSD_CANDLES_1M};

fn main() {
    let (file_path, mut candle_chart_csv_dir) =
        get_hk_trading_file_path(DATA_HISTORIC_XAUUSD_CANDLES_1M).unwrap();
    let csv_file_path = file_path.to_str().unwrap();

    println!("Load file: {}", file_path.display());

    candle_chart_csv_dir.push("candles_chart.svg");
    let candle_chart_csv_file_path = candle_chart_csv_dir.to_str().unwrap();
    println!("Write SVG to file: {}", candle_chart_csv_file_path);

    /*
       Read Candles from CSV file
    */
    let (_, candles) = CandleCSVDataSource::load_csv_file(
        csv_file_path,
        Some(CandleCSVLoadOption {
            limit: Some(30),
            offset: None,
        }),
    )
    .unwrap();

    // the width and height of SVG chart
    let (width, height) = (1270, 768);

    let margin_left_right = Duration::minutes(1);
    let from_time = candles.open_times[0] - margin_left_right;
    let to_time = candles.open_times[candles.open_times.len() - 1] + margin_left_right;
    println!(
        "from_time: {}, to_time: {}, len: {}",
        from_time,
        to_time,
        candles.open_times.len()
    );

    /*
       Generate RSI data
    */
    let (highest, lowest) = candles.borrow().get_highest_lowest().unwrap();

    /*
       Prepare overlays
    */
    let mut ohlcs = OhlcOverlay::new();
    ohlcs
        .drawing_area_width(width)
        .candles(Arc::new(candles))
        .from_time(from_time)
        .to_time(to_time);

    /*
       Draw chart
    */
    let drawing_backend = SVGBackend::with_file_path(candle_chart_csv_file_path, (width, height));
    let drawing_area = drawing_backend.into_drawing_area();
    drawing_area.fill(&BLACK_1).unwrap();

    let mut chart_context = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(
            from_time..to_time + Duration::minutes(1),
            lowest - 0.2..highest + 0.2,
        )
        .unwrap();

    let mut trading_chart_data = TradingChartData::new();
    trading_chart_data.with_ohlc_overlay(ohlcs);
    // chart_context.s
    // drawing_area.
    trading_chart_data.draw(&mut chart_context).unwrap();

    drawing_area.present().expect("Expect");
}
