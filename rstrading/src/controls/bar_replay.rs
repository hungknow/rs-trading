use std::sync::Arc;

use crate::data::{Candles};

#[derive(Clone, Default)]
pub struct CandleReplayModel {
    pub candles: Arc<Box<Candles>>,
    pub display_from_index: usize,
    pub display_to_index: usize,
    pub error: Option<String>,
}

pub enum CandleReplayAction {
    NextBar,
    JumpToBar(usize),
}

pub struct CandleReplayCapabilities {}

pub struct CandleReplay {
    // model: CandleReplayModel<'a>,
}

impl<'a> CandleReplay {
    pub fn new() -> Self {
        Self {
            // model: CandleReplayModel {
            //     candles,
            //     display_from_index: 0,
            //     display_to_index: 0,
            //     error: None,
            // },
        }
    }
}

pub fn candle_replay_reducer(
    model: CandleReplayModel,
    action: CandleReplayAction,
) -> CandleReplayModel {
    match action {
        CandleReplayAction::NextBar => CandleReplayModel {
            display_to_index: if model.display_to_index + 1 < model.candles.open.len() {
                model.display_to_index + 1
            } else {
                model.display_to_index
            },
            ..model
        },
        CandleReplayAction::JumpToBar(index) => {
            if index >= model.candles.open.len() {
                return CandleReplayModel {
                    error: Some(format!("Index out of range: {}", index)),
                    ..model
                };
            }
            return CandleReplayModel {
                display_to_index: index,
                ..model
            };
        }
    }
}

pub fn candle_replay_select_display_from_index(model: &CandleReplayModel) -> usize {
    model.display_from_index
}