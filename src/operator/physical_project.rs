use crate::cost::COST_TUP_DEFAULT_PROC_COST_UNIT;
use crate::operator::{OperatorId, PhysicalOperator};
use crate::property::PhysicalProperties;
use crate::Demo;
use cso_core::cost::Cost;
use cso_core::expression::ScalarExpression;
use cso_core::metadata::Stats;
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

impl cso_core::operator::PhysicalOperator<Demo> for PhysicalProject {
    fn name(&self) -> &str {
        "physical project"
    }

    fn operator_id(&self) -> &OperatorId {
        &OperatorId::PhysicalProject
    }

    fn derive_output_properties(&self, child_output_props: &[Rc<PhysicalProperties>]) -> Rc<PhysicalProperties> {
        child_output_props[0].clone()
    }

    fn required_properties(&self, input_prop: Rc<PhysicalProperties>) -> Vec<Vec<Rc<PhysicalProperties>>> {
        vec![vec![Rc::new(PhysicalProperties::new())], vec![input_prop]]
    }

    fn compute_cost(&self, stats: Option<&dyn Stats>) -> Cost {
        debug_assert!(stats.is_some());

        let row_count = stats.unwrap().output_row_count() as f64;
        Cost::new(row_count * COST_TUP_DEFAULT_PROC_COST_UNIT)
    }

    fn equal(&self, other: &PhysicalOperator) -> bool {
        match other.downcast_ref::<PhysicalProject>() {
            Some(other) => self.eq(other),
            None => false,
        }
    }
}
