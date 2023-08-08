use crate::expression::ScalarExpression;
use crate::operator::PhysicalOperator;

pub struct PhysicalFilter {
    predicate: Box<dyn ScalarExpression>,
}

impl PhysicalFilter {
    pub const fn new(predicate: Box<dyn ScalarExpression>) -> Self {
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
