use std::borrow::Borrow;
use std::env;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Duration, Utc};
use hk_trading::charts::drawing::IntoDrawingArea;
use hk_trading::charts::overlays::Ohlcs;
use hk_trading::charts::{style::colors::WHITE, svg_backend::SVGBackend, ChartBuilder};
use hk_trading::charts::{DrawingBackend, TradingChartData};
use hk_trading::data::{CandleCSVDataSource, CandleCSVLoadOption};

const DATA_HISTORIC_XAUUSD_CANDLES_1M: &str = "candles/xauusd_1m.csv";

fn find_file() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().unwrap();

    let checking_paths = vec![
        [DATA_HISTORIC_XAUUSD_CANDLES_1M, ""],
        ["hk-trading", DATA_HISTORIC_XAUUSD_CANDLES_1M],
    ];

    for p in checking_paths {
        let mut path = current_dir.clone();
        for pp in p.iter() {
            if pp.is_empty() {
                continue;
            }
            path.push(pp);
        }

        if Path::new(&path).exists() {
            return Some(path);
        }
    }

    None
}

fn main() {
    let file_path = find_file().unwrap();
    let csv_file_path = file_path.to_str().unwrap();
    println!("Load file: {}", file_path.display());

    let mut candle_chart_csv_dir = file_path.clone();
    candle_chart_csv_dir.pop();
    candle_chart_csv_dir.push("candles_chart.svg");
    let candle_chart_csv_file_path = candle_chart_csv_dir.to_str().unwrap();
    println!("Write tocandle_chart_csv_file_path file: {}", candle_chart_csv_file_path);

    /*
       READ CSV from file
    */
    let (csv_file_metadata, candles) = CandleCSVDataSource::load_csv_file(
        csv_file_path,
        Some(CandleCSVLoadOption {
            limit: Some(100),
            offset: None,
        }),
    )
    .unwrap();

    let (width, height) = (1024, 768);

    /*
       Generate RSI data
    */
    let (highest, lowest) = candles.borrow().get_highest_lowest().unwrap();
    let mut ohlcs = Ohlcs::new(&candles, width);

    /*
       Draw chart
    */
    let drawing_backend = SVGBackend::with_file_path(candle_chart_csv_file_path, (width, height));
    let drawing_area = drawing_backend.into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let margin_left_right = Duration::minutes(0);

    let from_time = candles.open_times[0] - margin_left_right;
    let to_time = candles.open_times[candles.open_times.len() - 1] + margin_left_right;

    // Set the time range to display on chart
    ohlcs.from_time(from_time);
    ohlcs.to_time(to_time);

    println!(
        "from_time: {}, to_time: {}, len: {}",
        from_time,
        to_time,
        candles.open_times.len()
    );

    let mut chart_context = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(
            from_time..to_time + Duration::minutes(1),
            lowest - 0.2..highest + 0.2,
        )
        .unwrap();

    let mut trading_chart_data = TradingChartData::new();
    trading_chart_data.with_ohlc_overlay(Box::new(ohlcs));
    trading_chart_data.draw(&mut chart_context).unwrap();

    drawing_area.present().expect("Expect");
}
