use crate::expression::ScalarExpression;
use crate::operator::PhysicalOperator;

pub struct PhysicalFilter {
    predicate: Box<dyn ScalarExpression>,
}

impl PhysicalFilter {
    pub fn new(predicate: Box<dyn ScalarExpression>) -> Self {
        assert!(predicate.is_boolean_expression());
        PhysicalFilter { predicate }
    }

    pub fn predicate(&self) -> &dyn ScalarExpression {
        &*self.predicate
    }
}

impl PhysicalOperator for PhysicalFilter {
    fn name(&self) -> &str {
        "physical filter"
    }

    fn operator_id(&self) -> i16 {
        5
    }
}
