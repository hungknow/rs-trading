use std::ops::Add;

use crate::data::Resolution;
use chrono::{DateTime, Duration, Utc};

#[derive(Clone, Default)]
pub struct CandleReplayModel {
    pub resolution: Resolution,
    pub display_from_timestamp: DateTime<Utc>,
    pub display_to_timestamp: DateTime<Utc>,
    pub error: Option<String>,
}

pub enum CandleReplayAction {
    SetData(Resolution, DateTime<Utc>, DateTime<Utc>),
    NextBar,
    // JumpToBar(usize),
}

pub struct CandleReplayReducer{}
// impl hk_redux_rs::Reducer<CandleReplayModel, CandleReplayAction> for CandleReplayReducer {
//     fn reduce(&self, model: CandleReplayModel, action: CandleReplayAction) -> CandleReplayModel {
//         match action {
//             CandleReplayAction::SetData(resolution, from_timestamp, to_timestamp) => CandleReplayModel {
//                 resolution,
//                 display_from_timestamp: from_timestamp,
//                 display_to_timestamp: to_timestamp,
//                 ..model
//             },
//             CandleReplayAction::NextBar => CandleReplayModel {
//                 display_from_timestamp: model
//                     .display_from_timestamp
//                     .add(Duration::seconds(model.resolution.to_seconds())),
//                 ..model
//             },
//         }
//     }
// }

// pub type CandleReplayStore = StoreWithMiddleware<hk_redux_rs::Store<CandleReplayModel, CandleReplayAction, CandleReplayReducer>>;

pub fn candle_replay_reducer(
    model: CandleReplayModel,
    action: CandleReplayAction,
) -> CandleReplayModel {
    match action {
        CandleReplayAction::SetData(resolution, from_timestamp, to_timestamp) => CandleReplayModel {
            resolution,
            display_from_timestamp: from_timestamp,
            display_to_timestamp: to_timestamp,
            ..model
        },
        CandleReplayAction::NextBar => CandleReplayModel {
            display_from_timestamp: model
                .display_from_timestamp
                .add(Duration::seconds(model.resolution.to_seconds())),
            ..model
        },
    }
}

pub fn candle_replay_select_display_from_timestamp(model: &CandleReplayModel) -> DateTime<Utc> {
    model.display_from_timestamp
}

pub fn candle_replay_select_display_to_timestamp(model: &CandleReplayModel) -> DateTime<Utc> {
    model.display_to_timestamp
}

// pub fn candle_replay_select_candles(model: &CandleReplayModel) -> Arc<Box<Candles>> {
//     model.candles.clone()
// }

