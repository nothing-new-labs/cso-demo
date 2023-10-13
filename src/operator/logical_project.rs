use crate::metadata::MdAccessor;
use crate::operator::OperatorId;
use crate::Demo;
use cso_core::expression::ScalarExpression;
use cso_core::metadata::Stats;
use cso_core::operator::LogicalOperator;
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
    type OptimizerType = Demo;

    fn name(&self) -> &str {
        "logical project"
    }

    fn operator_id(&self) -> &OperatorId {
        &OperatorId::LogicalProject
    }

    fn derive_statistics(&self, _md_accessor: &MdAccessor, input_stats: &[Rc<dyn Stats>]) -> Rc<dyn Stats> {
        input_stats[0].clone()
    }
}
