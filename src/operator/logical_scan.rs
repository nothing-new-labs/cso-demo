use crate::expression::ColumnVar;
use crate::metadata::MdAccessor;
use crate::operator::OperatorId;
use crate::statistics::{RelationMetadata, RelationStats, Statistics};
use crate::{Demo, Plan};
use cso_core::expression::ScalarExpression;
use cso_core::metadata::Stats;
use cso_core::operator::LogicalOperator;
use cso_core::ColumnRefSet;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct TableDesc {
    md_id: u64,
}

impl TableDesc {
    pub const fn new(md_id: u64) -> Self {
        Self { md_id }
    }

    pub fn md_id(&self) -> u64 {
        self.md_id
    }
}

#[derive(Debug)]
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

pub fn derive_scan_stats(
    md_accessor: &MdAccessor,
    input_stats: &[Rc<dyn Stats>],
    table_desc: &TableDesc,
) -> Rc<dyn Stats> {
    debug_assert!(input_stats.is_empty());

    let relation_md_id = table_desc.md_id();
    let rel_md = md_accessor
        .retrieve_metadata(&relation_md_id)
        .expect("Missing metadata");
    let rel_md = rel_md
        .downcast_ref::<RelationMetadata>()
        .expect("RelationMetadata expected");

    let rel_stats_md_id = rel_md.rel_stats_mdid();
    let rel_stats = md_accessor
        .retrieve_metadata(&rel_stats_md_id)
        .expect("Missing metadata");
    let rel_stats = rel_stats
        .downcast_ref::<RelationStats>()
        .expect("RelationStats expected");

    let output_row_count = rel_stats.rows();

    let mut column_stats = Vec::new();
    for col_stats_md_id in rel_stats.col_stat_mdids() {
        let col_stats = md_accessor
            .retrieve_metadata(col_stats_md_id)
            .expect("Missing metadata");
        column_stats.push(col_stats);
    }

    let stats = Statistics::new(output_row_count, column_stats);
    Rc::new(stats)
}

impl LogicalOperator<Demo> for LogicalScan {
    fn name(&self) -> &str {
        "logical get"
    }

    fn operator_id(&self) -> &OperatorId {
        &OperatorId::LogicalScan
    }

    fn derive_statistics(&self, md_accessor: &MdAccessor, input_stats: &[Rc<dyn Stats>]) -> Rc<dyn Stats> {
        derive_scan_stats(md_accessor, input_stats, self.table_desc())
    }

    fn derive_output_columns(&self, inputs: &[Plan], column_set: &mut ColumnRefSet) {
        debug_assert!(inputs.is_empty());
        self.output_columns
            .iter()
            .for_each(|expr| expr.derive_used_columns(column_set));
    }
}
