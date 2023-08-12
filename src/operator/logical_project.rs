use crate::expression::ScalarExpression;
use crate::metadata::MdAccessor;
use crate::operator::LogicalOperator;
use crate::statistics::Statistics;
use std::rc::Rc;

pub struct LogicalProject {
    _project: Vec<Box<dyn ScalarExpression>>,
}

impl LogicalProject {
    pub fn new(project: Vec<Box<dyn ScalarExpression>>) -> Self {
        LogicalProject { _project: project }
    }
}

impl LogicalOperator for LogicalProject {
    fn name(&self) -> &str {
        "logical project"
    }

    fn operator_id(&self) -> i16 {
        3
    }

    fn derive_statistics(&self, _md_accessor: &MdAccessor, input_stats: &[Rc<Statistics>]) -> Statistics {
        // TODO: projection stats
        (*input_stats[0]).clone()
    }
}
