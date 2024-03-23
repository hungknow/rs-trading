use std::sync::Arc;

use chrono::{DateTime, Utc};

use crate::errors::HkError;

use super::{Candles, Resolution};

pub struct DataSourceMeta {
    pub symbol: String,
    // pub source: String,
    pub resolution: Resolution,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

pub struct DataSourceGet<'a> {
    pub symbol: &'a str,
    pub resolution: Resolution,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

// #[async_trait]
pub trait CandleDataSource {
    // fn get_metadata(self) -> Vec<DataSourceMeta>;
    fn get_data_source_from<'a>(
        &self,
        option: DataSourceGet<'a>,
    ) -> impl futures::Future<Output = Result<Candles, HkError>>;
}

#[derive(Clone, PartialEq, Debug)]
pub struct CandleDisplayDataSourceState {
    pub symbol: String,
    pub resolution: Resolution,
    pub display_start_time: DateTime<Utc>,
    pub display_end_time: DateTime<Utc>,
    // pub data_start_time: DateTime<Utc>,
    // pub data_end_time: DateTime<Utc>,
    pub candles: Arc<Box<Candles>>,

    pub loading_data_time: Option<(DateTime<Utc>, DateTime<Utc>)>,
    // pub loading_data_end_time: Option<DateTime<Utc>>,
    // pub is_loading_data: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CandlesNewChunkEvent {
    pub old_state: Arc<CandleDisplayDataSourceState>,
    pub new_state: Arc<CandleDisplayDataSourceState>,
    pub new_candle_index: usize,
}

pub struct CandlesCandlesUpdatedEvent {
    pub old_state: Arc<CandleDisplayDataSourceState>,
    pub new_state: Arc<CandleDisplayDataSourceState>,
    pub candle_index: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub struct DisplayTimeRangeChangedEvent {
    pub old_state: Arc<CandleDisplayDataSourceState>,
    pub new_state: Arc<CandleDisplayDataSourceState>,
    pub from_time: DateTime<Utc>,
    pub to_time: DateTime<Utc>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CandleUpdatedEvent {
    pub old_state: Arc<CandleDisplayDataSourceState>,
    pub new_state: Arc<CandleDisplayDataSourceState>,
    pub index: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub enum CandleDisplayDataSourceEvent {
    DisplayTimeRangeChanged(DisplayTimeRangeChangedEvent),
    CandleUpdated(CandleUpdatedEvent),
    CandlesNewChunk(CandlesNewChunkEvent),
}

// impl Identifier for CandleDisplayDataSourceEvent {
//     fn id(&self) -> &str {
//         match self {
//             CandleDisplayDataSourceEvent::DisplayTimeRangeChanged(_) => {
//                 "DisplayTimeRangeChanged".to_string()
//             }
//             CandleDisplayDataSourceEvent::CandleUpdated(_) => "CandleUpdated".to_string(),
//             CandleDisplayDataSourceEvent::CandlesNewChunk(_) => "CandlesNewChunk".to_string(),
//         }
//     }
// }

pub enum CandleDisplayDataSourceError {}
// One source for symbol
pub trait CandleDisplayDataSource {
    // fn get_state(self) -> Arc<CandleDisplayDataSourceState>;
    fn set_display_range(
        state: Arc<CandleDisplayDataSourceState>,
        from_time: DateTime<Utc>,
        to_time: DateTime<Utc>,
    ) -> Result<CandleDisplayDataSourceEvent, CandleDisplayDataSourceError>;
    // fn on(
    //     &mut self,
    //     event_name: CandleDisplayDataSourceEventName,
    //     callback: fn(CandleDisplayDataSourceEvent) -> (),
    // );
    // fn get_display_data(self) -> CandleData<'static>;
}

// Test, generate candle data source
// Notify if there's new
