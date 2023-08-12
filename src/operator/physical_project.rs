use crate::expression::ScalarExpression;
use crate::operator::PhysicalOperator;

pub struct PhysicalProject {
    _project: Vec<Box<dyn ScalarExpression>>,
}

impl PhysicalProject {
    pub fn new(project: Vec<Box<dyn ScalarExpression>>) -> Self {
        PhysicalProject { _project: project }
    }
}

impl PhysicalOperator for PhysicalProject {
    fn name(&self) -> &str {
        "physical project"
    }

    fn operator_id(&self) -> i16 {
        6
    }
}
