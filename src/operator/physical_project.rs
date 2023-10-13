use crate::operator::{OperatorId, PhysicalOperator};
use crate::property::PhysicalProperties;
use crate::Demo;
use cso_core::expression::ScalarExpression;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhysicalProject {
    _project: Vec<Rc<dyn ScalarExpression>>,
}

impl PhysicalProject {
    pub fn new(project: Vec<Rc<dyn ScalarExpression>>) -> Self {
        PhysicalProject { _project: project }
    }
}

impl cso_core::operator::PhysicalOperator for PhysicalProject {
    type OptimizerType = Demo;

    fn name(&self) -> &str {
        "physical project"
    }

    fn operator_id(&self) -> &OperatorId {
        &OperatorId::LogicalProject
    }

    fn clone(&self) -> Box<PhysicalOperator> {
        Box::new(Clone::clone(self))
    }

    fn derive_output_properties(&self, child_output_props: &[Rc<PhysicalProperties>]) -> Rc<PhysicalProperties> {
        child_output_props[0].clone()
    }

    fn required_properties(&self, input_prop: Rc<PhysicalProperties>) -> Vec<Vec<Rc<PhysicalProperties>>> {
        vec![vec![input_prop], vec![Rc::new(PhysicalProperties::new())]]
    }

    fn equal(&self, other: &PhysicalOperator) -> bool {
        match other.downcast_ref::<PhysicalProject>() {
            Some(other) => self.eq(other),
            None => false,
        }
    }
}
