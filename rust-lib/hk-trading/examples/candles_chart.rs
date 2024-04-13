use std::borrow::Borrow;
use std::env;

use chrono::{DateTime, Utc};
use hk_trading::charts::drawing::IntoDrawingArea;
use hk_trading::charts::overlays::Ohlcs;
use hk_trading::charts::{style::colors::WHITE, svg_backend::SVGBackend, ChartBuilder};
use hk_trading::charts::{DrawingBackend, TradingChartData};
use hk_trading::data::{CandleCSVDataSource, CandleCSVLoadOption};

const DATA_HISTORIC_XAUUSD_CANDLES_1M: &str = "./candles/xauusd_1m.csv";

fn main() {
    println!("Current DIR: {}", env::current_dir().unwrap().display());
    /*
       READ CSV from file
    */
    let (csv_file_metadata, candles) = CandleCSVDataSource::load_csv_file(
        DATA_HISTORIC_XAUUSD_CANDLES_1M,
        Some(CandleCSVLoadOption {
            limit: Some(20),
            offset: None,
        }),
    )
    .unwrap();

    /*
       Generate RSI data
    */
    let (highest, lowest) = candles.borrow().get_highest_lowest().unwrap();
    let ohlcs = Ohlcs::new(&candles);

    /*
       Draw chart
    */
    let drawing_backend = SVGBackend::with_file_path("candles_chart.svg", (1024, 768));
    let drawing_area = drawing_backend.into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let from_time = candles.open_times[0];
    let to_time = candles.open_times[candles.open_times.len() - 1];

    println!("from_time: {}, to_time: {}", from_time, to_time);

    let mut chart_context = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(from_time..to_time, lowest..highest)
        .unwrap();

    let mut trading_chart_data = TradingChartData::new();
    trading_chart_data.with_ohlc_overlay(Box::new(ohlcs));
    trading_chart_data.draw(&mut chart_context).unwrap();

    drawing_area.present().expect("Expect");
}
