use std::ops::{Add, Range, Sub};

use chrono::{Date, DateTime, Datelike, Duration, TimeZone, Timelike};

use crate::charts::coord::ranged1d::{DefaultFormatting, KeyPointHint, Ranged};

/// The trait that describe some time value. This is the uniformed abstraction that works
/// for both Date, DateTime and Duration, etc.
pub trait TimeValue: Eq + Sized {
    type DateType: Datelike + PartialOrd;

    /// Returns the date that is no later than the time
    fn date_floor(&self) -> Self::DateType;
    /// Returns the date that is no earlier than the time
    fn date_ceil(&self) -> Self::DateType;
    /// Returns the maximum value that is earlier than the given date
    fn earliest_after_date(date: Self::DateType) -> Self;
    /// Returns the duration between two time value
    fn subtract(&self, other: &Self) -> Duration;
    /// Add duration to time value
    fn add(&self, duration: &Duration) -> Self;
    /// Instantiate a date type for current time value;
    fn ymd(&self, year: i32, month: u32, date: u32) -> Self::DateType;
    /// Cast current date type into this type
    fn from_date(date: Self::DateType) -> Self;

    /// Map the coord spec
    fn map_coord(value: &Self, begin: &Self, end: &Self, limit: (i32, i32)) -> i32 {
        let total_span = end.subtract(begin);
        let value_span = value.subtract(begin);

        // First, lets try the nanoseconds precision
        if let Some(total_ns) = total_span.num_nanoseconds() {
            if let Some(value_ns) = value_span.num_nanoseconds() {
                return (f64::from(limit.1 - limit.0) * value_ns as f64 / total_ns as f64) as i32
                    + limit.0;
            }
        }

        // Yes, converting them to floating point may lose precision, but this is Ok.
        // If it overflows, it means we have a time span nearly 300 years, we are safe to ignore the
        // portion less than 1 day.
        let total_days = total_span.num_days() as f64;
        let value_days = value_span.num_days() as f64;

        (f64::from(limit.1 - limit.0) * value_days / total_days) as i32 + limit.0
    }

    /// Map pixel to coord spec
    fn unmap_coord(point: i32, begin: &Self, end: &Self, limit: (i32, i32)) -> Self {
        let total_span = end.subtract(begin);
        let offset = (point - limit.0) as i64;

        // Check if nanoseconds fit in i64
        if let Some(total_ns) = total_span.num_nanoseconds() {
            let pixel_span = (limit.1 - limit.0) as i64;
            let factor = total_ns / pixel_span;
            let remainder = total_ns % pixel_span;
            if factor == 0
                || i64::MAX / factor > offset.abs()
                || (remainder == 0 && i64::MAX / factor >= offset.abs())
            {
                let nano_seconds = offset * factor + (remainder * offset) / pixel_span;
                return begin.add(&Duration::nanoseconds(nano_seconds));
            }
        }

        // Otherwise, use days
        let total_days = total_span.num_days() as f64;
        let days = (((offset as f64) * total_days) / ((limit.1 - limit.0) as f64)) as i64;
        begin.add(&Duration::days(days))
    }
}

impl<Z: TimeZone> TimeValue for Date<Z> {
    type DateType = Date<Z>;
    fn date_floor(&self) -> Date<Z> {
        self.clone()
    }
    fn date_ceil(&self) -> Date<Z> {
        self.clone()
    }
    fn earliest_after_date(date: Date<Z>) -> Self {
        date
    }
    fn subtract(&self, other: &Date<Z>) -> Duration {
        self.clone() - other.clone()
    }
    fn add(&self, other: &Duration) -> Date<Z> {
        self.clone() + *other
    }

    fn ymd(&self, year: i32, month: u32, date: u32) -> Self::DateType {
        self.timezone().ymd(year, month, date)
    }

    fn from_date(date: Self::DateType) -> Self {
        date
    }
}

impl<Z: TimeZone> TimeValue for DateTime<Z> {
    type DateType = Date<Z>;
    fn date_floor(&self) -> Date<Z> {
        self.date()
    }
    fn date_ceil(&self) -> Date<Z> {
        if self.time().num_seconds_from_midnight() > 0 {
            self.date() + Duration::days(1)
        } else {
            self.date()
        }
    }
    fn earliest_after_date(date: Date<Z>) -> DateTime<Z> {
        date.and_hms(0, 0, 0)
    }

    fn subtract(&self, other: &DateTime<Z>) -> Duration {
        self.clone() - other.clone()
    }
    fn add(&self, other: &Duration) -> DateTime<Z> {
        self.clone() + *other
    }

    fn ymd(&self, year: i32, month: u32, date: u32) -> Self::DateType {
        self.timezone().ymd(year, month, date)
    }

    fn from_date(date: Self::DateType) -> Self {
        date.and_hms(0, 0, 0)
    }
}

/// The ranged coordinate for date
#[derive(Clone)]
pub struct RangedDate<D: Datelike>(D, D);

impl<D: Datelike> From<Range<D>> for RangedDate<D> {
    fn from(range: Range<D>) -> Self {
        Self(range.start, range.end)
    }
}

impl<D> Ranged for RangedDate<D>
where
    D: Datelike + TimeValue + Sub<D, Output = Duration> + Add<Duration, Output = D> + Clone,
{
    type FormatOption = DefaultFormatting;
    type ValueType = D;

    fn range(&self) -> Range<D> {
        self.0.clone()..self.1.clone()
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        TimeValue::map_coord(value, &self.0, &self.1, limit)
    }

    fn key_points<HintType: KeyPointHint>(&self, hint: HintType) -> Vec<Self::ValueType> {
        let max_points = hint.max_num_points();
        let mut ret = vec![];

        let total_days = (self.1.clone() - self.0.clone()).num_days();
        let total_weeks = (self.1.clone() - self.0.clone()).num_weeks();

        if total_days > 0 && total_days as usize <= max_points {
            for day_idx in 0..=total_days {
                ret.push(self.0.clone() + Duration::days(day_idx));
            }
            return ret;
        }

        if total_weeks > 0 && total_weeks as usize <= max_points {
            for day_idx in 0..=total_weeks {
                ret.push(self.0.clone() + Duration::weeks(day_idx));
            }
            return ret;
        }

        // When all data is in the same week, just plot properly.
        if total_weeks == 0 {
            ret.push(self.0.clone());
            return ret;
        }

        let week_per_point = ((total_weeks as f64) / (max_points as f64)).ceil() as usize;

        for idx in 0..=(total_weeks as usize / week_per_point) {
            ret.push(self.0.clone() + Duration::weeks((idx * week_per_point) as i64));
        }

        ret
    }
}


/// The ranged coordinate for the date and time
#[derive(Clone)]
pub struct RangedDateTime<DT: Datelike + Timelike + TimeValue>(DT, DT);

// impl<Z: TimeZone> AsRangedCoord for Range<DateTime<Z>> {
//     type CoordDescType = RangedDateTime<DateTime<Z>>;
//     type Value = DateTime<Z>;
// }

impl<Z: TimeZone> From<Range<DateTime<Z>>> for RangedDateTime<DateTime<Z>> {
    fn from(range: Range<DateTime<Z>>) -> Self {
        Self(range.start, range.end)
    }
}

impl<DT> Ranged for RangedDateTime<DT>
where
    DT: Datelike + Timelike + TimeValue + Clone + PartialOrd,
    DT: Add<Duration, Output = DT>,
    DT: Sub<DT, Output = Duration>,
    RangedDate<DT::DateType>: Ranged<ValueType = DT::DateType>,
{
    type FormatOption = DefaultFormatting;
    type ValueType = DT;

    fn range(&self) -> Range<DT> {
        self.0.clone()..self.1.clone()
    }

    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32 {
        TimeValue::map_coord(value, &self.0, &self.1, limit)
    }

    fn key_points<HintType: KeyPointHint>(&self, hint: HintType) -> Vec<Self::ValueType> {
        let max_points = hint.max_num_points();
        let total_span = self.1.clone() - self.0.clone();

        if let Some(total_ns) = total_span.num_nanoseconds() {
            if let Some(actual_ns_per_point) =
                compute_period_per_point(total_ns as u64, max_points, true)
            {
                let start_time_ns = u64::from(self.0.num_seconds_from_midnight()) * 1_000_000_000
                    + u64::from(self.0.nanosecond());

                let mut start_time = DT::from_date(self.0.date_floor())
                    + Duration::nanoseconds(if start_time_ns % actual_ns_per_point > 0 {
                        start_time_ns + (actual_ns_per_point - start_time_ns % actual_ns_per_point)
                    } else {
                        start_time_ns
                    } as i64);

                let mut ret = vec![];

                while start_time < self.1 {
                    ret.push(start_time.clone());
                    start_time = start_time + Duration::nanoseconds(actual_ns_per_point as i64);
                }

                return ret;
            }
        }

        // Otherwise, it actually behaves like a date
        let date_range = RangedDate(self.0.date_ceil(), self.1.date_floor());

        date_range
            .key_points(max_points)
            .into_iter()
            .map(DT::from_date)
            .collect()
    }
}

#[allow(clippy::inconsistent_digit_grouping)]
fn compute_period_per_point(total_ns: u64, max_points: usize, sub_daily: bool) -> Option<u64> {
    let min_ns_per_point = total_ns as f64 / max_points as f64;
    let actual_ns_per_point: u64 = (10u64).pow(min_ns_per_point.log10().floor() as u32);

    fn determine_actual_ns_per_point(
        total_ns: u64,
        mut actual_ns_per_point: u64,
        units: &[u64],
        base: u64,
        max_points: usize,
    ) -> u64 {
        let mut unit_per_point_idx = 0;
        while total_ns / actual_ns_per_point > max_points as u64 * units[unit_per_point_idx] {
            unit_per_point_idx += 1;
            if unit_per_point_idx == units.len() {
                unit_per_point_idx = 0;
                actual_ns_per_point *= base;
            }
        }
        units[unit_per_point_idx] * actual_ns_per_point
    }

    if actual_ns_per_point < 1_000_000_000 {
        Some(determine_actual_ns_per_point(
            total_ns,
            actual_ns_per_point,
            &[1, 2, 5],
            10,
            max_points,
        ))
    } else if actual_ns_per_point < 3600_000_000_000 {
        Some(determine_actual_ns_per_point(
            total_ns,
            1_000_000_000,
            &[1, 2, 5, 10, 15, 20, 30],
            60,
            max_points,
        ))
    } else if actual_ns_per_point < 3600_000_000_000 * 24 {
        Some(determine_actual_ns_per_point(
            total_ns,
            3600_000_000_000,
            &[1, 2, 4, 8, 12],
            24,
            max_points,
        ))
    } else if !sub_daily {
        if actual_ns_per_point < 3600_000_000_000 * 24 * 10 {
            Some(determine_actual_ns_per_point(
                total_ns,
                3600_000_000_000 * 24,
                &[1, 2, 5, 7],
                10,
                max_points,
            ))
        } else {
            Some(determine_actual_ns_per_point(
                total_ns,
                3600_000_000_000 * 24 * 10,
                &[1, 2, 5],
                10,
                max_points,
            ))
        }
    } else {
        None
    }
}
