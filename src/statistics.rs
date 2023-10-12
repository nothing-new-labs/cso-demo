use crate::datum::Datum;
use cso_core::any::AsAny;
use cso_core::metadata::Stats;
use cso_core::metadata::{MdId, Metadata};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bucket {
    lower: Datum,     // Lower bound value of the bucket.
    upper: Datum,     // Upper bound value of the bucket.
    ndv: u64,         // Estimated number of distinct values in the bucket.
    value_count: u64, // Estimated number of values in the bucket.
}

impl Bucket {
    #[inline]
    pub const fn new(lower: Datum, upper: Datum, ndv: u64, value_count: u64) -> Self {
        Self {
            lower,
            upper,
            ndv,
            value_count,
        }
    }

    #[inline]
    pub fn lower(&self) -> Datum {
        self.lower
    }

    #[inline]
    pub fn upper(&self) -> Datum {
        self.upper
    }

    #[inline]
    pub fn ndv(&self) -> u64 {
        self.ndv
    }

    #[inline]
    pub fn value_count(&self) -> u64 {
        self.value_count
    }
}

/// A histogram is a representation of the distribution of a column.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Histogram {
    buckets: Vec<Bucket>,
}

impl Histogram {
    #[inline]
    pub const fn new(buckets: Vec<Bucket>) -> Self {
        Self { buckets }
    }

    #[inline]
    pub fn buckets(&self) -> &[Bucket] {
        self.buckets.as_slice()
    }
}

pub type ColumnIndex = usize;

/// Statistics information of a column
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColumnStats {
    col_idx: ColumnIndex,
    name: String,
    min: Datum,                   // Min value of the column
    max: Datum,                   // Max value of the column
    null_count: u64,              // Count of null values
    histogram: Option<Histogram>, // Histogram of column
}

impl ColumnStats {
    pub const fn new(
        col_idx: ColumnIndex,
        name: String,
        min: Datum,
        max: Datum,
        null_count: u64,
        histogram: Option<Histogram>,
    ) -> Self {
        Self {
            col_idx,
            name,
            min,
            max,
            null_count,
            histogram,
        }
    }

    pub fn column_index(&self) -> ColumnIndex {
        self.col_idx
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn min(&self) -> Datum {
        self.min
    }

    pub fn max(&self) -> Datum {
        self.max
    }

    pub fn null_count(&self) -> u64 {
        self.null_count
    }

    pub fn histogram(&self) -> &Option<Histogram> {
        &self.histogram
    }
}

#[typetag::serde]
impl Metadata for ColumnStats {}

#[derive(Clone, Debug)]
pub struct Statistics {
    output_row_count: u64,

    /// Statistics of columns, column index -> column stat
    column_stats: Vec<Box<dyn Metadata>>,
}

impl Statistics {
    pub const fn new(output_row_count: u64, column_stats: Vec<Box<dyn Metadata>>) -> Self {
        Self {
            output_row_count,
            column_stats,
        }
    }

    pub fn column_stats(&self) -> &Vec<Box<dyn Metadata>> {
        &self.column_stats
    }
}

impl Stats for Statistics {
    fn should_update(&self, new_stats: &Rc<dyn Stats>) -> bool {
        let new_stats = new_stats.as_any().downcast_ref::<Statistics>().unwrap();
        new_stats.output_row_count < self.output_row_count
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelationStats {
    name: String,
    rows: u64,
    empty: bool,
    col_stat_mdids: Vec<Box<dyn MdId>>,
}

impl RelationStats {
    pub const fn new(name: String, rows: u64, empty: bool, col_stat_mdids: Vec<Box<dyn MdId>>) -> Self {
        Self {
            name,
            rows,
            empty,
            col_stat_mdids,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rows(&self) -> u64 {
        self.rows
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn col_stat_mdids(&self) -> &[Box<dyn MdId>] {
        &self.col_stat_mdids
    }
}

#[typetag::serde]
impl Metadata for RelationStats {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColumnMetadata {
    name: String,
    attno: u64,
    nullable: bool,
    width: u32,
    default: Datum,
}

impl ColumnMetadata {
    pub const fn new(name: String, attno: u64, nullable: bool, width: u32, default: Datum) -> Self {
        Self {
            name,
            attno,
            nullable,
            width,
            default,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn attno(&self) -> u64 {
        self.attno
    }

    pub fn nullable(&self) -> bool {
        self.nullable
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn default(&self) -> Datum {
        self.default
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelationMetadata {
    name: String,
    column_metadata: Vec<ColumnMetadata>,
    rel_stats_mdid: Box<dyn MdId>,
}

impl RelationMetadata {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(name: String, column_metadata: Vec<ColumnMetadata>, rel_stats_mdid: Box<dyn MdId>) -> Self {
        Self {
            name,
            column_metadata,
            rel_stats_mdid,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn column_metadata(&self) -> &[ColumnMetadata] {
        &self.column_metadata
    }

    pub fn rel_stats_mdid(&self) -> &Box<dyn MdId> {
        &self.rel_stats_mdid
    }
}

#[typetag::serde]
impl Metadata for RelationMetadata {}
