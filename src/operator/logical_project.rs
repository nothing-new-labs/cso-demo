use crate::metadata::md_accessor::MdAccessor;
use crate::metadata::statistics::Stats;
use crate::operator::LogicalOperator;
use cso_core::expression::ScalarExpression;
use std::rc::Rc;

#[derive(Debug)]
pub struct LogicalProject {
    project: Vec<Rc<dyn ScalarExpression>>,
}

impl LogicalProject {
    pub fn new(project: Vec<Rc<dyn ScalarExpression>>) -> Self {
        LogicalProject { project }
    }

    pub fn project(&self) -> &[Rc<dyn ScalarExpression>] {
        &self.project
    }
}

impl LogicalOperator for LogicalProject {
    fn name(&self) -> &str {
        "logical project"
    }

    fn operator_id(&self) -> i16 {
        3
    }

    fn derive_statistics(&self, _md_accessor: &MdAccessor, input_stats: &[Rc<dyn Stats>]) -> Rc<dyn Stats> {
        input_stats[0].clone()
    }
}
