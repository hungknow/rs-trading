use std::{rc::Rc, sync::Arc};

use chrono::{DateTime, Utc};
use plotters::{
    backend::{DrawingBackend, SVGBackend},
    chart::{ChartBuilder, ChartContext},
    coord::CoordTranslate,
    element::PathElement,
    series::LineSeries,
};
use rstrading::{
    data::{
        CandleCSVDataSource, CandleDataSource, CandleDisplayDataSource,
        CandleDisplayDataSourceEvent, CandleDisplayDataSourceEventName,
        CandleDisplayDataSourceState, DataSourceGet,
    },
    drawing::{SubChart, SubChartState},
    indicators::{traits::Indicator, ExponentialMovingAverage, ExponentialMovingAverageState},
    Next, TA, TimestampValueDS,
};

struct EmaDrawing {
    ema: ExponentialMovingAverage,
    period: usize,
    inputs: Vec<f64>,
    outputs: Vec<f64>,
}

// impl SubChartState for EmaDrawing {
//     fn is_state_changed(self) -> DateTime<Utc> {
//         false
//     }
// }

impl SubChart for EmaDrawing {
    fn get_state(&self) -> impl SubChartState {
        self
    }

    fn draw<'a, DB: DrawingBackend, CT: CoordTranslate>(
        &self,
        chart: &mut ChartContext<'a, DB, CT>,
    ) {
        chart
            .draw_series(LineSeries::new(ema_line_data.clone(), BLUE.stroke_width(1)))
            .unwrap()
            .label("SMA 15")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(150, 50, 168)));
    }
}

// async fn calc_ema(inputs: &[f64], period: usize) -> Vec<f64> {
//     let ema = match ExponentialMovingAverage::new(period) {
//         Ok(ema) => ema,
//         Err(e) => panic!("EMA error: {}", e),
//     };
//     let ema_values = ema.calc(inputs);
//     // Print on chart
// }

fn render() {
    let backend = SVGBackend::new("output.svg", (1024, 768)).into_drawing_area();

    // Create one chart
    let chart = ChartBuilder::on(&backend)
        .build_ranged(0f32..10f32, 0f32..10f32)
        .unwrap();

    let (area1, area2) = backend.split_horizontally((600));

    (area1, area2)
}

fn calculate_new_ta<T: TA>(
    old_state: Rc<CandleDisplayDataSourceState>,
    new_state: Rc<CandleDisplayDataSourceState>,
    ta_executor: T,
    ta_container: TATimeAware<T>,
) {
    let timestamp = state.display_data.close_time[event.index];
    let close_data = state.display_data.close[event.index];
    let mut from_data_index = 0;
    let mut to_data_index = 0;
    let state_type: ta_container::StateType = ta_container.state;

    // If the new from time is less than the previous from time, create the TA indicator from the start.
    if old_state.start_time > new_state.start_time {
        ta_container.state.clone().reset();
    }
    // If the new to time is greater than the previous to time, use the previous TA indicator
    if old_state.end_time < new_state.end_time {
        // Use the previous TA indicator
        ta_container
    } else {
    }

    // find the index of the new candle stick

    // find the index of TA indicator

    // generate the new TA value
    let (new_state, new_data) = ta_executor.next((&ta_container.state, close_data));
    let mut new_data_list = ta_container.value_list.clone();
    // new_data_list.as_ref().pu
    new_ta_time_aware = TATimeAware {
        state: new_state,
        // value_list: new_data.clone().,
    }; 
}

// TA's from_time < candle's from_time:
// TA's end_time > candle's to_time:
// fn new_ta<'a, T: TA>(
//     // existing TA,
//     ta_executor: T,
//     ta_container: TAContainer<T>,
//     // exponential_moving_average_state: Rc<T::StateType>,
//     // ta_value: &[T::ValueType],
//     ta_from_index: usize,
//     // candleDisplayDataSourceState: Rc<CandleDisplayDataSourceState>,
//     value_list: impl TimestampValueDS<f64>,
//     // new candle data
//     candle_from_index: usize,
// ) -> (TAContainer<T>) {
//     let mut new_ta_value_list = ta_container.value_list.value()[0..ta_from_index].to_vec();
//     let mut new_ta_timestamp_list = ta_container.value_list.timestamp()[0..ta_from_index].to_vec();
//     let mut new_exponential_moving_average_state: Rc<T::StateType> = ta_container.state;
//     let mut new_value: T::ValueType;
//     let value_list_real = value_list.value();
//     for i in candle_from_index..new_ta_value_list.len() {
//         // Generate the new value for TA
//         (new_exponential_moving_average_state, new_value) =
//             ta_executor.next((new_exponential_moving_average_state, value_list_real[i]));
//             new_ta_timestamp_list.push()
//         new_ta_value_list.push(new_value);
//     }
//     TAContainer {
//         state: new_exponential_moving_average_state,
//         value_list: new_ta_value_list.as_slice(),
//     }
// }

// Structure contains
// - List of candle
// - List of indicators
// - Display time range

// There's updated timestame
fn handleNewData(
    oldCandleDisplayDataSourceState: Rc<CandleDisplayDataSourceState>,
    newCandleDisplayDataSourceState: Rc<CandleDisplayDataSourceState>,
    exponentialMovingAverage: &ExponentialMovingAverage,
    exponentialMovingAverageState: &ExponentialMovingAverageState,
    candleDisplayDataSourceEvent: CandleDisplayDataSourceEvent,
) -> (ExponentialMovingAverageState) {
    match (candleDisplayDataSourceEvent) {
        CandleDisplayDataSourceEvent::DisplayTimeRangeChanged(event) => {
            // with the new display state, calculate the new candle stick
            // Trigger the data pipeline after reading one candle stick
            // let new_inputs = candleDisplayDS.get_display_data();
            // let ema_value = calc_ema(new_inputs, 3);

            // Draw the new candle stick
            // Calculate the new TA value
        }
        CandleDisplayDataSourceEvent::CandleUpdated(event) => {
            let timestamp = event.state.display_data.close_time[event.index];
            let closeData = event.state.display_data.close[event.index];
            // if timestamp is greater than n
        }
        CandleDisplayDataSourceEvent::CandlesNewChunk(event) => {
            // if timestamp is greater than n
        }
    }

    // If the new to time is greater than the previous to time, use the previous TA indicator

    // Find the
    // For each new candle stick, calculate the new value with TA indicator

    return exponentialMovingAverage;
}

fn main() {
    // Prepare the chart
    // The candle is read from CSV file
    let candleDS = CandleCSVDataSource::new();
    // let metadatas = candleDS.get_metadata();
    // let targetMetadata = metadatas[0];
    
    // TA
    let exponentialMovingAverage = ExponentialMovingAverage::new().unwrap();

    // All states
    let candleDisplayDataSourceState: Rc<CandleDisplayDataSourceState>;
    let exponentialMovingAverageState = ExponentialMovingAverageState::new(3);

    // Event listener
    // candleDS.on(
    //     CandleDisplayDataSourceEventName::DisplayTimeRangeChanged,
    //     |event| {
    //         // with the new display state, calculate the new candle stick
    //         // Trigger the data pipeline after reading one candle stick
    //         // let new_inputs = candleDisplayDS.get_display_data();
    //         // let ema_value = calc_ema(new_inputs, 3);
    //         match (event) {
    //             CandleDisplayDataSourceEvent::DisplayTimeRangeChanged(event) => {
    //                 // with the new display state, calculate the new candle stick
    //                 // Trigger the data pipeline after reading one candle stick
    //                 // let new_inputs = candleDisplayDS.get_display_data();
    //                 // let ema_value = calc_ema(new_inputs, 3);
    //             },
    //             _ => {},
    //         }

    //         // Redraw the chart
    //     },
    // );

    // Read the candlestick from time to time
    // let candleDS = candleDS.get_data_source_from(DataSourceGet {
    //     symbol: targetMetadata.symbol,
    //     resolution: targetMetadata.resolution,
    //     start_time: targetMetadata.start_time,
    //     end_time: targetMetadata.end_time,
    // });

    // Create RSI indicator
    // Trigger the data pipeline after reading one candle stick
    // Receive the event of candle stick
    // Clone RSI indicator, then calculate new value for RSI indicator

    // candleDisplayDS.on(
    //     CandleDisplayDataSourceEventName::DisplayTimeRangeChanged,
    //     |state| {
    //         // with the new display state, calculate the new candle stick
    //         // Trigger the data pipeline after reading one candle stick
    //         let new_inputs = candleDisplayDS.get_display_data();
    //         let ema_value = calc_ema(new_inputs, 3);

    //         // Redraw the chart
    //     },
    // );

    // Set the new time range
    candleDS.set_display_range(
        // new start time
        candleDisplayDataSourceState.start_time,
        // old start time
        candleDisplayDataSourceState.end_time,
    )
    // ) {
    //     // Some(event) => {
    //         // Update the new state
    //         // Call function to handle the new state
    //         (exponentialMovingAverageState) = handleNewData(
    //             candleDisplayDataSourceState,
    //             state,
    //             &exponentialMovingAverage,
    //             exponentialMovingAverageState,
    //             event,
    //         );
    //         candleDisplayDataSourceState = state;
    //     }
    //     Err(CandleDisplayDataSourceError) => {
    //         println("Fail to set the new time range");
    //     }
    // }
    // Receive the new display value
    // Trigger the data pipeline after reading one candle stick
    // If the new from time is less than the previous from time, create the TA indicator from the start.
    // If the new to time is greater than the previous to time, use the previous TA indicator
    // For each new candle stick, calculate the new value with TA indicator

    // Fetch the new candle stick from the data source based on the time range changed
    // Chart receive the new time range changed event, it try to
}
