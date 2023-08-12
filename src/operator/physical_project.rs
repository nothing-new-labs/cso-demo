use crate::expression::ScalarExpression;
use crate::operator::PhysicalOperator;
use crate::property::PhysicalProperties;
use std::rc::Rc;

pub struct PhysicalProject {
    _project: Vec<Rc<dyn ScalarExpression>>,
}

impl PhysicalProject {
    pub fn new(project: Vec<Rc<dyn ScalarExpression>>) -> Self {
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

    fn derive_output_prop(&self, _: &[Rc<PhysicalProperties>]) -> PhysicalProperties {
        todo!()
    }
}
