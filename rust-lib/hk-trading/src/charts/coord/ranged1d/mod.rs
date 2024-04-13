use std::ops::Range;

/// Since stable Rust doesn't have specialization, it's very hard to make our own trait that
/// automatically implemented the value formatter. This trait uses as a marker indicates if we
/// should automatically implement the default value formater based on it's `Debug` trait
pub trait DefaultValueFormatOption {}

/// This makes the ranged coord uses the default `Debug` based formatting
pub struct DefaultFormatting;
impl DefaultValueFormatOption for DefaultFormatting {}

/// This markers prevent Plotters to implement the default `Debug` based formatting
pub struct NoDefaultFormatting;
impl DefaultValueFormatOption for NoDefaultFormatting {}

/// Specify the weight of key points.
pub enum KeyPointWeight {
    /// Allows only bold key points
    Bold,
    /// Allows any key points
    Any,
}

impl KeyPointWeight {
    /// Check if this key point weight setting allows light point
    pub fn allow_light_points(&self) -> bool {
        match self {
            KeyPointWeight::Bold => false,
            KeyPointWeight::Any => true,
        }
    }
}

/// The trait for a hint provided to the key point algorithm used by the coordinate specs.
/// The most important constraint is the `max_num_points` which means the algorithm could emit no more than specific number of key points
/// `weight` is used to determine if this is used as a bold grid line or light grid line
/// `bold_points` returns the max number of coresponding bold grid lines
pub trait KeyPointHint {
    /// Returns the max number of key points
    fn max_num_points(&self) -> usize;
    /// Returns the weight for this hint
    fn weight(&self) -> KeyPointWeight;
    /// Returns the point number constraint for the bold points
    fn bold_points(&self) -> usize {
        self.max_num_points()
    }
}

impl KeyPointHint for usize {
    fn max_num_points(&self) -> usize {
        *self
    }

    fn weight(&self) -> KeyPointWeight {
        KeyPointWeight::Any
    }
}

/// The trait that indicates we have a ordered and ranged value
/// Which is used to describe any 1D axis.
pub trait Ranged {
    type FormatOption: DefaultValueFormatOption;
    /// The type of this value in this range specification
    type ValueType;

    /// This function maps the value to i32, which is the drawing coordinate
    fn map(&self, value: &Self::ValueType, limit: (i32, i32)) -> i32;

    /// This function gives the key points that we can draw a grid based on this
    fn key_points<Hint: KeyPointHint>(&self, hint: Hint) -> Vec<Self::ValueType>;

    /// Get the range of this value
    fn range(&self) -> Range<Self::ValueType>;

    /// This function provides the on-axis part of its range
    #[allow(clippy::range_plus_one)]
    fn axis_pixel_range(&self, limit: (i32, i32)) -> Range<i32> {
        if limit.0 < limit.1 {
            limit.0..limit.1
        } else {
            limit.1..limit.0
        }
    }
}

/// The trait for the type that can be converted into a ranged coordinate axis
pub trait AsRangedCoord: Sized {
    /// Type to describe a coordinate system
    type CoordDescType: Ranged<ValueType = Self::Value> + From<Self>;
    /// Type for values in the given coordinate system
    type Value;
}

impl<T> AsRangedCoord for T
where
    T: Ranged,
{
    type CoordDescType = T;
    type Value = T::ValueType;
}

pub(super) mod types;
