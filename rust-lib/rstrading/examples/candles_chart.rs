use plotters::{chart, drawing, prelude::*};
use rstrading::data::CandleCSVDataSource;

const DATA_HISTORIC_XAUUSD_CANDLES_1M: &str = "./candles/xauusd_1m.csv";

fn main() {
    /*
       READ CSV from file
    */
    let (csv_file_metadata, candles) = CandleCSVDataSource::load_csv_file(
        DATA_HISTORIC_XAUUSD_CANDLES_1M,
        Some(CandleCSVLoadOption { limit: 20 }),
    )
    .unwrap();

    /*
       Generate RSI data
    */

    /*
       Draw chart
    */
    let drawing_area = SVGBackend::new("test.svg", (1024, 768))
        .into_drawing_area()
        .fill(&WHITE)
        .unwrap();
    let chart_builder = chart::ChartBuilder::on(drawing_area)
        .build_cartesian_2d(x_spec, y_spec)
        .unwrap()
        .configure_mesh();
}
