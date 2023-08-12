use crate::datum::Datum;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Bucket {
    _lower: Datum,      // Lower bound value of the bucket.
    _upper: Datum,      // Upper bound value of the bucket.
    _num_values: f64,   // Estimated number of values in the bucket.
    _num_distinct: f64, // Estimated number of distinct values in the bucket.
}

/// A histogram is a representation of the distribution of a column.
#[derive(Clone)]
pub struct Histogram {
    _buckets: Vec<Bucket>,
}

pub type ColumnIndex = usize;

/// Statistics information of a column
#[derive(Clone)]
pub struct ColumnStat {
    _min: Datum,      // Min value of the column
    _max: Datum,      // Max value of the column
    _ndv: f64,        // Number of distinct values
    _null_count: u64, // Count of null values

    _histogram: Option<Histogram>, // Histogram of column
}

pub type ColumnStatSet = HashMap<ColumnIndex, ColumnStat>;

#[derive(Clone)]
pub struct Statistics {
    output_row_count: usize,

    /// Statistics of columns, column index -> column stat
    _column_stats: ColumnStatSet,
}

impl Statistics {
    pub const fn new(output_row_count: usize, column_stats: ColumnStatSet) -> Self {
        Self {
            output_row_count,
            _column_stats: column_stats,
        }
    }

    pub const fn should_update(new_stats: &Statistics, old_stats: &Statistics) -> bool {
        new_stats.output_row_count < old_stats.output_row_count
    }
}
