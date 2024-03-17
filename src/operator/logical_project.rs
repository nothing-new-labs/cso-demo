use crate::metadata::MdAccessor;
use crate::operator::OperatorId;
use crate::{Demo, Plan};
use cso_core::expression::ScalarExpression;
use cso_core::metadata::Stats;
use cso_core::operator::LogicalOperator;
use cso_core::ColumnRefSet;
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

impl LogicalOperator<Demo> for LogicalProject {
    fn name(&self) -> &str {
        "logical project"
    }

    fn operator_id(&self) -> &OperatorId {
        &OperatorId::LogicalProject
    }

    fn derive_statistics(&self, _md_accessor: &MdAccessor, input_stats: &[Rc<dyn Stats>]) -> Rc<dyn Stats> {
        input_stats[0].clone()
    }

    fn derive_output_columns(&self, inputs: &[Plan], column_set: &mut ColumnRefSet) {
        debug_assert_eq!(inputs.len(), 1);
        inputs[0].derive_output_columns(column_set);
        self.project
            .iter()
            .for_each(|scalar| scalar.derive_used_columns(column_set))
    }
}
