use crate::expression::ColumnVar;
use crate::metadata::MdAccessor;
use crate::operator::logical_scan::{derive_scan_stats, TableDesc};
use crate::operator::OperatorId;
use crate::statistics::{IndexMd, IndexType};
use crate::Demo;
use cso_core::expression::ScalarExpression;
use cso_core::metadata::Stats;
use cso_core::operator::LogicalOperator;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct IndexDesc {
    mdid: u64,
    name: String,
    index_type: IndexType,
    key_columns: Vec<usize>,
    included_columns: Vec<usize>,
}

impl IndexDesc {
    pub fn new(
        mdid: u64,
        name: String,
        index_type: IndexType,
        key_columns: Vec<usize>,
        included_columns: Vec<usize>,
    ) -> Self {
        Self {
            mdid,
            name,
            index_type,
            key_columns,
            included_columns,
        }
    }

    pub fn key_columns(&self) -> &[usize] {
        &self.key_columns
    }
}

#[derive(Debug)]
pub struct LogicalIndexScan {
    index_desc: IndexDesc,
    table_desc: TableDesc,
    output_columns: Vec<ColumnVar>,
    predicate: Rc<dyn ScalarExpression>,
}

impl LogicalIndexScan {
    pub fn new(
        table_desc: TableDesc,
        index_md: &IndexMd,
        output_columns: Vec<ColumnVar>,
        predicate: Rc<dyn ScalarExpression>,
    ) -> Self {
        let index_desc = IndexDesc::new(
            index_md.mdid(),
            index_md.index_name().to_string(),
            index_md.index_type().clone(),
            index_md.key_columns().to_vec(),
            index_md.included_columns().to_vec(),
        );

        Self {
            index_desc,
            table_desc: table_desc.clone(),
            output_columns,
            predicate,
        }
    }

    pub fn index_desc(&self) -> &IndexDesc {
        &self.index_desc
    }

    pub fn table_desc(&self) -> &TableDesc {
        &self.table_desc
    }

    pub fn output_columns(&self) -> &[ColumnVar] {
        &self.output_columns
    }

    pub fn predicate(&self) -> &Rc<dyn ScalarExpression> {
        &self.predicate
    }
}

impl LogicalOperator<Demo> for LogicalIndexScan {
    fn name(&self) -> &str {
        "logical index scan"
    }

    fn operator_id(&self) -> &OperatorId {
        &OperatorId::LogicalIndexScan
    }

    fn derive_statistics(&self, md_accessor: &MdAccessor, input_stats: &[Rc<dyn Stats>]) -> Rc<dyn Stats> {
        let base_table_stats = derive_scan_stats(md_accessor, input_stats, self.table_desc());

        // todo: derive index scan stats from base_table_stats and index desc.
        base_table_stats
    }
}
