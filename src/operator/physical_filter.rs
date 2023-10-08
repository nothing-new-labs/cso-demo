use crate::expression::ScalarExpression;
use crate::operator::PhysicalOperator;
use crate::property::PhysicalProperties;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct PhysicalFilter {
    predicate: Rc<dyn ScalarExpression>,
}

impl PhysicalFilter {
    pub fn new(predicate: Rc<dyn ScalarExpression>) -> Self {
        assert!(predicate.is_boolean_expression());
        PhysicalFilter { predicate }
    }

    pub fn predicate(&self) -> &dyn ScalarExpression {
        self.predicate.as_ref()
    }
}

impl PhysicalOperator for PhysicalFilter {
    fn name(&self) -> &str {
        "physical filter"
    }

    fn operator_id(&self) -> i16 {
        5
    }

    fn derive_output_properties(&self, child_output_props: &[Rc<PhysicalProperties>]) -> Rc<PhysicalProperties> {
        child_output_props[0].clone()
    }

    fn required_properties(&self, input_prop: Rc<PhysicalProperties>) -> Vec<Vec<Rc<PhysicalProperties>>> {
        vec![vec![input_prop], vec![Rc::new(PhysicalProperties::new())]]
    }

    fn equal(&self, other: &dyn PhysicalOperator) -> bool {
        let other = other.downcast_ref::<PhysicalFilter>().unwrap();
        self.eq(other)
    }
}

impl PartialEq for PhysicalFilter {
    fn eq(&self, other: &Self) -> bool {
        self.predicate.equal(other.predicate())
    }
}
