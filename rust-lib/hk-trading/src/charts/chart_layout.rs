use chrono::{DateTime, Utc};

use crate::data::Resolution;

pub struct ChartLayoutState {
    chart_name: String,
    symbol: String,
    resolution: Resolution,
    id: i64,
    updated_at: u64,
}

pub struct ChartLayoutSave {
    id: Option<i64>,
    chart_name: String,
    symbol: String,
    resolution: Resolution,
    content: String,
}

pub struct ChartDisplayState {
    from_time: DateTime<Utc>,
    to_time: DateTime<Utc>
}

pub struct TimeRange {
    /// Left-bound of the range | Index (in IB mode)
    t1: i64,

    /// Right-bound of the range | Index (in IB mode)
    t2: i64,

    exp: bool
}
