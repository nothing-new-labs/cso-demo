use crate::metadata::MdId;
use crate::operators::LogicalOperator;
use crate::statistics::Statistics;
use crate::OptimizerContext;
use std::rc::Rc;

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
}

impl LogicalOperator for LogicalScan {
    fn name(&self) -> &str {
        "logical get"
    }

    fn operator_id(&self) -> i16 {
        1
    }

    fn derive_statistics(&self, optimizer_ctx: &OptimizerContext, input_stats: &[Rc<Statistics>]) -> Statistics {
        debug_assert!(input_stats.is_empty());

        let md_accessor = optimizer_ctx.md_accessor();
        md_accessor.derive_stats(self.table_desc.md_id())
    }
}
