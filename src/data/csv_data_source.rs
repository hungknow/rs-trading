// map from symbol to filepath

use std::{collections::HashMap, rc::Rc, sync::Arc};

use chrono::{DateTime, Utc};

use crate::{core::SyncPublisher, event_listener::EventListener};

use super::{
    CandleDataSource, CandleDisplayDataSource, CandleDisplayDataSourceError,
    CandleDisplayDataSourceEvent, CandleDisplayDataSourceState, DataSourceMeta, Resolution,
};

pub type CandleCSVDataSourceEventHandler = fn(event: CandleDisplayDataSourceEvent);
struct CsvMetadata {
    file_path: String,
    resolution: Resolution,
}

pub struct CandleCSVDataSource {
    symbolToFilepath: HashMap<String, CsvMetadata>,
    // events: SyncPublisher<String, CandleDisplayDataSourceEvent>,
}

// Read Data from CSV

impl CandleCSVDataSource {
    pub fn new() -> Self {
        Self {
            symbolToFilepath: HashMap::new(),
            // events: SyncPublisher::new(),
        }
    }

    pub fn on(
        &mut self,
        event_name: CandleDisplayDataSourceEvent,
        event: CandleCSVDataSourceEventHandler,
    ) {
        // self.events.subscribe(event_name.id(), event);
    }

    pub fn set_display_range(
        from_time: DateTime<Utc>,
        to_time: DateTime<Utc>,
    ) -> Result<bool, CandleDisplayDataSourceError> {
        todo!()
    }

    pub fn get_event_rx_channel(
        &self,
    ) -> std::sync::mpsc::Receiver<Arc<CandleDisplayDataSourceEvent>> {
        todo!()
    }

    pub fn get_state(&self) -> Arc<CandleDisplayDataSourceState> {
        todo!()
    }

    pub fn set_data_time_range(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<bool, CandleDisplayDataSourceError> {
        todo!()
    }
}

impl CandleDataSource for CandleCSVDataSource {
    fn get_metadata(self) -> Vec<DataSourceMeta> {
        todo!()
    }

    fn get_data_source_from<'a>(&self, option: super::DataSourceGet) -> super::Candles {
        todo!()
    }
}

// impl CandleDisplayDataSource for CandleCSVDataSource {
//     fn set_display_range(
//         state: Arc<super::CandleDisplayDataSourceState>,
//         from_time: DateTime<Utc>,
//         to_time: DateTime<Utc>,
//     ) -> std::result::Result<CandleDisplayDataSourceEvent, data_source::CandleDisplayDataSourceError> {
//         todo!()
//     }
// }

impl EventListener<CandleDisplayDataSourceEvent> for CandleCSVDataSource {
    fn event_listener_add(
        &mut self,
        event_name: CandleDisplayDataSourceEvent,
        event: event_listener::Event,
    ) {
        // event.No
    }
}
