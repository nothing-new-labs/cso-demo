use crate::expression::ColumnVar;
use crate::statistics::{RelationMetadata, RelationStats, Statistics};
use cso_core::metadata::accessor::MdAccessor;
use cso_core::metadata::statistics::Stats;
use cso_core::metadata::MdId;
use cso_core::operator::LogicalOperator;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct TableDesc {
    md_id: Box<dyn MdId>,
}

impl TableDesc {
    pub const fn new(md_id: Box<dyn MdId>) -> Self {
        Self { md_id }
    }

    fn md_id(&self) -> &Box<dyn MdId> {
        &self.md_id
    }
}

impl PartialEq for TableDesc {
    fn eq(&self, other: &Self) -> bool {
        self.md_id.equal(other.md_id().as_ref())
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

    fn derive_statistics(&self, md_accessor: &MdAccessor, input_stats: &[Rc<dyn Stats>]) -> Rc<dyn Stats> {
        debug_assert!(input_stats.is_empty());

        let relation_md_id = self.table_desc.md_id();
        let rel_md = md_accessor.retrieve_metadata(relation_md_id).expect("Missing metadata");
        let rel_md = rel_md
            .downcast_ref::<RelationMetadata>()
            .expect("RelationMetadata expected");

        let rel_stats_md_id = rel_md.rel_stats_mdid();
        let rel_stats = md_accessor
            .retrieve_metadata(rel_stats_md_id)
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
}

impl LogicalOperator for LogicalScan {
    fn name(&self) -> &str {
        "logical get"
    }

    fn operator_id(&self) -> i16 {
        1
    }

    fn derive_statistics(&self, md_accessor: &MdAccessor, input_stats: &[Rc<dyn Stats>]) -> Rc<dyn Stats> {
        self.derive_statistics(md_accessor, input_stats)
    }
}
