use std::{rc::Rc, sync::Arc};

use chrono::{DateTime, Duration, Utc};
use plotters::{
    backend::{DrawingBackend, SVGBackend},
    chart::{ChartBuilder, ChartContext},
    coord::CoordTranslate,
    data,
    element::PathElement,
    series::LineSeries,
};
use redux_rs::Store;
use rstrading::{
    controls::{candle_replay_reducer, candle_replay_select_display_to_index, CandleReplay, CandleReplayAction, CandleReplayCapabilities, CandleReplayEvent, CandleReplayModel}, data::{
        CandleCSVDataSource, CandleDataSource, CandleDisplayDataSource, CandleDisplayDataSourceEvent, CandleDisplayDataSourceState, Candles, DataSourceGet, DisplayTimeRangeChangedEvent
    }, drawing::{SubChart, SubChartState}, indicators::{indicator::IndicatorContainer, traits::{Indicator, TATimeAware}, ExponentialMovingAverage, ExponentialMovingAverageState}, Next, Reset, TimestampValueDS
};
use tokio::task;

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

// impl SubChart for EmaDrawing {
//     fn get_state(&self) -> dyn SubChartState {
//         self
//     }

//     fn draw<'a, DB: DrawingBackend, CT: CoordTranslate>(
//         &self,
//         chart: &mut ChartContext<'a, DB, CT>,
//     ) {
//         // chart
//         //     .draw_series(LineSeries::new(ema_line_data.clone(), BLUE.stroke_width(1)))
//         //     .unwrap()
//         //     .label("SMA 15")
//         //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(150, 50, 168)));
//     }
// }

// async fn calc_ema(inputs: &[f64], period: usize) -> Vec<f64> {
//     let ema = match ExponentialMovingAverage::new(period) {
//         Ok(ema) => ema,
//         Err(e) => panic!("EMA error: {}", e),
//     };
//     let ema_values = ema.calc(inputs);
//     // Print on chart
// }

// fn render() {
//     let backend = SVGBackend::new("output.svg", (1024, 768)).into_drawing_area();

//     // Create one chart
//     let chart = ChartBuilder::on(&backend)
//         .build_ranged(0f32..10f32, 0f32..10f32)
//         .unwrap();

//     let (area1, area2) = backend.split_horizontally((600));

//     (area1, area2)
// }

/**
 *  Data:
 * - Candle Data Source old & new state
 * - TA old state
 * - TA old value
 *  New data
 * - TA new state
 * - TA new value
 */
fn calculate_new_ta<T: Indicator<InputType = f64>>(
    old_state: &CandleDisplayDataSourceState,
    new_state: &CandleDisplayDataSourceState,
    ta_container: IndicatorContainer<T>,
) -> (IndicatorContainer<T>) {
    // If the new from time is less than the previous from time, create the TA indicator from the start.
    // if old_state.candles.get_last_close_time() > new_state.candles.get_last_close_time() {
    //     // ta_container.new_state.clone().reset();
    //     ta_state = Arc::new(ta_state.as_ref().clone().reset());
    // }
    // // If the new to time is greater than the previous to time, use the previous TA indicator
    // if old_state.data_end_time < new_state.data_end_time {
    //     // Use the previous TA indicator
    //     // ta_container
    // } else {
    // }

    //TODO:
    // find the index of the new candle stick
    // find the index of TA indicator

    let mut ta_state = ta_container.state.clone();
    ta_state.reset();

    // let mut cloned_ta_state = ta_container.state.clone();
    let mut ta_timestamp: Vec<DateTime<Utc>> = vec![];
    let mut new_ta_data_vec: Vec<T::OutputType> = vec![];
    let mut new_ta_data: T::OutputType;
    // using the close value, generate the new TA value
    for &close_value in &new_state.candles.close {
        (ta_state, new_ta_data) = ta_container.indicator.next((ta_state.as_ref(), close_value));
        // Append the new TA data into the array
        ta_timestamp.push(new_state.candles.close_time.clone());
        new_ta_data_vec.push(new_ta_data);
    }

    IndicatorContainer{
        indicator: ta_container.indicator,
        state: ta_state,
        output: new_ta_data_vec,
        timestamp: ta_timestamp,
    }
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
//     // candleDisplayDataSourceState:Arc<CandleDisplayDataSourceState>,
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
// fn handleNewData(
//     oldCandleDisplayDataSourceState: Arc<CandleDisplayDataSourceState>,
//     newCandleDisplayDataSourceState: Arc<CandleDisplayDataSourceState>,
//     exponentialMovingAverage: &ExponentialMovingAverage,
//     exponentialMovingAverageState: &ExponentialMovingAverageState,
//     candleDisplayDataSourceEvent: CandleDisplayDataSourceEvent,
// ) -> (ExponentialMovingAverageState) {
//     match (candleDisplayDataSourceEvent) {
//         CandleDisplayDataSourceEvent::DisplayTimeRangeChanged(event) => {
//             // with the new display state, calculate the new candle stick
//             // Trigger the data pipeline after reading one candle stick
//             // let new_inputs = candleDisplayDS.get_display_data();
//             // let ema_value = calc_ema(new_inputs, 3);

//             // Draw the new candle stick
//             // Calculate the new TA value
//         }
//         CandleDisplayDataSourceEvent::CandleUpdated(event) => {
//             let timestamp = event.new_state.display_data.close_time[event.index];
//             let closeData = event.new_state.display_data.close[event.index];
//             // if timestamp is greater than n
//         }
//         CandleDisplayDataSourceEvent::CandlesNewChunk(event) => {
//             // if timestamp is greater than n
//         }
//     }

//     // If the new to time is greater than the previous to time, use the previous TA indicator

//     // Find the
//     // For each new candle stick, calculate the new value with TA indicator

//     return exponentialMovingAverage;
// }

// trait IndicatorCandleDisplayDataource {
//     fn handle_event(
//         &self,
//         old_state:Arc<CandleDisplayDataSourceState>,
//         event: CandleDisplayDataSourceEvent,
//     );
// }

// impl IndicatorCandleDisplayDataource for Indicator {
//     fn handle_event(
//         &self,
//         old_state:Arc<CandleDisplayDataSourceState>,
//         event: CandleDisplayDataSourceEvent,
//     ) {
//         match (event) {
//             CandleDisplayDataSourceEvent::DisplayTimeRangeChanged(event) => {
//                 // with the new display state, calculate the new candle stick
//                 // Trigger the data pipeline after reading one candle stick
//                 // let new_inputs = candleDisplayDS.get_display_data();
//                 // let ema_value = calc_ema(new_inputs, 3);
//             }
//             _ => {}
//         }
//     }
// }

async fn handleCandleDisplayDataSourceEvent<TA: Indicator<InputType = f64>>(
    event: &DisplayTimeRangeChangedEvent,
    ta_executors: Vec<(TA, Arc<TA::StateType>)>,
) {
    // match event.as_ref() {
    //     &CandleDisplayDataSourceEvent::DisplayTimeRangeChanged(event) => {
    // with the new display state, calculate the new candle stick
    // Trigger the data pipeline after reading one candle stick
    // let new_inputs = candleDisplayDS.get_display_data();
    // let ema_value = calc_ema(new_inputs, 3);

    //TODO: Spawn the thread task to process the new data
    // task::spawn_blocking(|| {
    // Calculate the new TA value
    for (ta_executor, ta_state) in ta_executors {
        calculate_new_ta(
            event.old_state.as_ref(),
            event.new_state.as_ref(),
            ta_executor,
            ta_state,
        );
    }

    // });

}

#[tokio::main]
async fn main() {
    // Prepare the chart
    // The candle is read from CSV file
    let candleDS = CandleCSVDataSource::new();
    let bar_replay = CandleReplay::new();

    let store = Store::new(candle_replay_reducer);
    // let mut bar_replay_model = CandleReplayModel {
    //     candles: &Candles{
    //         open: vec![],
    //         high: vec![],
    //         low: vec![],
    //         close: vec![],
    //         volume: vec![],
    //         close_time: vec![],
    //         trade_count: vec![],
    //     },
    //     display_from_index: 0,
    //     display_to_index: 0,
    //     error: None,
    // };
    let candle_replay_capacities = &CandleReplayCapabilities{};
    // let metadatas = candleDS.get_metadata();
    // let targetMetadata = metadatas[0];

    // TA
    let mut exponential_moving_average_container = IndicatorContainer {
        indicator: ExponentialMovingAverage::new(),
        state: Arc::new(ExponentialMovingAverageState::new(3)),
        output: vec![],
        timestamp: vec![],
    };

    // Create RSI indicator
    // Trigger the data pipeline after reading one candle stick
    // Receive the event of candle stick
    // Clone RSI indicator, then calculate new value for RSI indicator

    // let mut candle_ds_rx_channel = candleDS.get_event_rx_channel();
    // // spawn thread to listen for the event
    // let event_listener_handler = task::spawn(async move {
    //     // let mut candleDisplayDataSourceState = candleDisplayDataSourceState.clone();
    //     loop {
    //         // futures::select! {
    //         // tokio::select! {
    //         // match {
    //         let event = candle_ds_rx_channel.recv();
    //         match event {
    //             Ok(event) => {
    //                 match (event.as_ref()) {
    //                     CandleDisplayDataSourceEvent::DisplayTimeRangeChanged(event) => {
    //                         //         // with the new display state, calculate the new candle stick
    //                         //         // Trigger the data pipeline after reading one candle stick
    //                         //         // let new_inputs = candleDisplayDS.get_display_data();
    //                         //         // let ema_value = calc_ema(new_inputs, 3);
    //                         //         candleDisplayDataSourceState = event.new_state.clone();
    //                         handleCandleDisplayDataSourceEvent(
    //                             event,
    //                             vec![(
    //                                 exponential_moving_average,
    //                                 exponential_moving_average_state.clone(),
    //                             )],
    //                         )
    //                         .await;
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //             Err(err) => {
    //                 // No more event to receive
    //                 // exit loop
    //                 println!("{}", err);
    //                 break;
    //             }
    //         }
    //         // }
    //         // }
    //     }
    // });

    // Get the available data time range from candle data source
    // let candleDisplayDataSourceState = candleDS.get_state();
    // let data_start_time = candleDisplayDataSourceState.data_start_time.clone();
    // let data_end_time = candleDisplayDataSourceState.data_end_time.clone();

    // let next_end_time = data_start_time + Duration::minutes(5);
    // // Set the new time range
    // candleDS.set_data_time_range(
    //     // new start time
    //     data_start_time,
    //     // old start time
    //     next_end_time,
    // );
    // bar_replay_model = candle_replay_reducer(bar_replay_model, &CandleReplayEvent::NextBar);
    store.dispatch(CandleReplayAction::NextBar).await;

    let display_to_index = store.select(candle_replay_select_display_to_index).await;


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

    tokio::join!(event_listener_handler);
}
