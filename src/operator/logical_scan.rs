use crate::expression::ColumnVar;
use crate::metadata::{MdAccessor, MdId};
use crate::operator::LogicalOperator;
use crate::statistics::Statistics;
use std::any::Any;
use std::rc::Rc;

#[derive(Clone)]
pub struct TableDesc {
    md_id: MdId,
}

impl TableDesc {
    fn md_id(&self) -> &MdId {
        &self.md_id
    }
}

pub struct LogicalScan {
    table_desc: TableDesc,
    output_columns: Vec<ColumnVar>,
}

impl LogicalScan {
    pub fn new(table_desc: TableDesc, output_columns: Vec<ColumnVar>) -> Self {
        LogicalScan {
            table_desc,
            output_columns,
        }
    }

    pub fn table_desc(&self) -> &TableDesc {
        &self.table_desc
    }

    pub fn output_columns(&self) -> &[ColumnVar] {
        &self.output_columns
    }
}

impl LogicalOperator for LogicalScan {
    fn name(&self) -> &str {
        "logical get"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn operator_id(&self) -> i16 {
        1
    }

    fn derive_statistics(&self, md_accessor: &MdAccessor, input_stats: &[Rc<Statistics>]) -> Statistics {
        debug_assert!(input_stats.is_empty());
        md_accessor.derive_stats(self.table_desc.md_id())
    }
}
