use crate::charts::coord::ranged1d::{
    DefaultFormatting, KeyPointHint, NoDefaultFormatting, Ranged,
};
use std::ops::Range;

macro_rules! gen_key_points_comp {
    (float, $name:ident, $type:ty) => {
        fn $name(range: ($type, $type), max_points: usize) -> Vec<$type> {
            if max_points == 0 {
                return vec![];
            }

            let range = (range.0.min(range.1) as f64, range.1.max(range.0) as f64);

            assert!(!(range.0.is_nan() || range.1.is_nan()));

            if (range.0 - range.1).abs() < std::f64::EPSILON {
                return vec![range.0 as $type];
            }

            let mut scale = (10f64).powf((range.1 - range.0).log(10.0).floor());
            // The value granularity controls how we round the values.
            // To avoid generating key points like 1.00000000001, we round to the nearest multiple of the
            // value granularity.
            // By default, we make the granularity as the 1/10 of the scale.
            let mut value_granularity = scale / 10.0;
            fn rem_euclid(a: f64, b: f64) -> f64 {
                let ret = if b > 0.0 {
                    a - (a / b).floor() * b
                } else {
                    a - (a / b).ceil() * b
                };
                if (ret - b).abs() < std::f64::EPSILON {
                    0.0
                } else {
                    ret
                }
            }

            // At this point we need to make sure that the loop invariant:
            // The scale must yield number of points than requested
            if 1 + ((range.1 - range.0) / scale).floor() as usize > max_points {
                scale *= 10.0;
                value_granularity *= 10.0;
            }

            'outer: loop {
                let old_scale = scale;
                for nxt in [2.0, 5.0, 10.0].iter() {
                    let mut new_left = range.0 - rem_euclid(range.0, old_scale / nxt);
                    if new_left < range.0 {
                        new_left += old_scale / nxt;
                    }
                    let new_right = range.1 - rem_euclid(range.1, old_scale / nxt);

                    let npoints = 1.0 + ((new_right - new_left) / old_scale * nxt);

                    if npoints.round() as usize > max_points {
                        break 'outer;
                    }

                    scale = old_scale / nxt;
                }
                scale = old_scale / 10.0;
                value_granularity /= 10.0;
            }

            let mut ret = vec![];
            // In some extreme cases, left might be too big, so that (left + scale) - left == 0 due to
            // floating point error.
            // In this case, we may loop forever. To avoid this, we need to use two variables to store
            // the current left value. So we need keep a left_base and a left_relative.
            let left = {
                let mut value = range.0 - rem_euclid(range.0, scale);
                if value < range.0 {
                    value += scale;
                }
                value
            };
            let left_base = (left / value_granularity).floor() * value_granularity;
            let mut left_relative = left - left_base;
            let right = range.1 - rem_euclid(range.1, scale);
            while (right - left_relative - left_base) >= -std::f64::EPSILON {
                let new_left_relative =
                    (left_relative / value_granularity).round() * value_granularity;
                if new_left_relative < 0.0 {
                    left_relative += value_granularity;
                }
                ret.push((left_relative + left_base) as $type);
                left_relative += scale;
            }
            return ret;
        }
    };
    (integer, $name:ident, $type:ty) => {
        fn $name(range: ($type, $type), max_points: usize) -> Vec<$type> {
            let mut scale: $type = 1;
            let range = (range.0.min(range.1), range.0.max(range.1));
            let range_size = range.1 as f64 - range.0 as f64;
            'outer: while (range_size / scale as f64).ceil() > max_points as f64 {
                let next_scale = scale * 10;
                for new_scale in [scale * 2, scale * 5, scale * 10].iter() {
                    scale = *new_scale;
                    if (range_size / *new_scale as f64).ceil() < max_points as f64 {
                        break 'outer;
                    }
                }
                scale = next_scale;
            }

            let (mut left, right) = (
                range.0 + (scale - range.0 % scale) % scale,
                range.1 - range.1 % scale,
            );

            let mut ret = vec![];
            while left <= right {
                ret.push(left as $type);
                if left < right {
                    left += scale;
                } else {
                    break;
                }
            }

            return ret;
        }
    };
}

gen_key_points_comp!(float, compute_f32_key_points, f32);
gen_key_points_comp!(float, compute_f64_key_points, f64);
gen_key_points_comp!(integer, compute_i32_key_points, i32);
gen_key_points_comp!(integer, compute_u32_key_points, u32);
gen_key_points_comp!(integer, compute_i64_key_points, i64);
gen_key_points_comp!(integer, compute_u64_key_points, u64);
gen_key_points_comp!(integer, compute_i128_key_points, i128);
gen_key_points_comp!(integer, compute_u128_key_points, u128);
gen_key_points_comp!(integer, compute_isize_key_points, isize);
gen_key_points_comp!(integer, compute_usize_key_points, usize);

macro_rules! make_numeric_coord {
    ($type:ty, $name:ident, $key_points:ident, $doc: expr, $fmt: ident) => {
        #[doc = $doc]
        #[derive(Clone)]
        pub struct $name($type, $type);
        impl From<Range<$type>> for $name {
            fn from(range: Range<$type>) -> Self {
                return $name(range.start, range.end);
            }
        }
        impl Ranged for $name {
            type FormatOption = $fmt;
            type ValueType = $type;
            #[allow(clippy::float_cmp)]
            fn map(&self, v: &$type, limit: (i32, i32)) -> i32 {
                // Corner case: If we have a range that have only one value,
                // then we just assign everything to the only point
                if self.1 == self.0 {
                    return (limit.1 - limit.0) / 2;
                }

                let logic_length = (*v as f64 - self.0 as f64) / (self.1 as f64 - self.0 as f64);

                let actual_length = limit.1 - limit.0;

                if actual_length == 0 {
                    return limit.1;
                }

                if logic_length.is_infinite() {
                    if logic_length.is_sign_positive() {
                        return limit.1;
                    } else {
                        return limit.0;
                    }
                }

                if actual_length > 0 {
                    return limit.0 + (actual_length as f64 * logic_length + 1e-3).floor() as i32;
                } else {
                    return limit.0 + (actual_length as f64 * logic_length - 1e-3).ceil() as i32;
                }
            }
            fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<$type> {
                $key_points((self.0, self.1), hint.max_num_points())
            }
            fn range(&self) -> Range<$type> {
                return self.0..self.1;
            }
        }
    };
    ($type:ty, $name:ident, $key_points:ident, $doc: expr) => {
        make_numeric_coord!($type, $name, $key_points, $doc, DefaultFormatting);
    };
}

make_numeric_coord!(
    f32,
    RangedCoordf32,
    compute_f32_key_points,
    "The ranged coordinate for type f32",
    NoDefaultFormatting
);
make_numeric_coord!(
    f64,
    RangedCoordf64,
    compute_f64_key_points,
    "The ranged coordinate for type f64",
    NoDefaultFormatting
);
make_numeric_coord!(
    u32,
    RangedCoordu32,
    compute_u32_key_points,
    "The ranged coordinate for type u32"
);
make_numeric_coord!(
    i32,
    RangedCoordi32,
    compute_i32_key_points,
    "The ranged coordinate for type i32"
);
make_numeric_coord!(
    u64,
    RangedCoordu64,
    compute_u64_key_points,
    "The ranged coordinate for type u64"
);
make_numeric_coord!(
    i64,
    RangedCoordi64,
    compute_i64_key_points,
    "The ranged coordinate for type i64"
);
make_numeric_coord!(
    u128,
    RangedCoordu128,
    compute_u128_key_points,
    "The ranged coordinate for type u128"
);
make_numeric_coord!(
    i128,
    RangedCoordi128,
    compute_i128_key_points,
    "The ranged coordinate for type i128"
);
make_numeric_coord!(
    usize,
    RangedCoordusize,
    compute_usize_key_points,
    "The ranged coordinate for type usize"
);
make_numeric_coord!(
    isize,
    RangedCoordisize,
    compute_isize_key_points,
    "The ranged coordinate for type isize"
);
