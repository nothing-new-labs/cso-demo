use crate::any::AsAny;
use crate::metadata::Metadata;
use std::fmt::Debug;
use std::rc::Rc;

pub trait Stats: Debug + AsAny {
    fn should_update(&self, new_stats: &Rc<dyn Stats>) -> bool;
}

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
