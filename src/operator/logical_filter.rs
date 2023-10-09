use crate::metadata::md_accessor::MdAccessor;
use crate::metadata::statistics::Stats;
use crate::operator::LogicalOperator;
use cso_core::expression::ScalarExpression;
use std::rc::Rc;

#[derive(Debug)]
pub struct LogicalFilter {
    predicate: Rc<dyn ScalarExpression>,
}

impl LogicalFilter {
    pub fn new(predicate: Rc<dyn ScalarExpression>) -> Self {
        assert!(predicate.is_boolean_expression());
        LogicalFilter { predicate }
    }

    pub fn predicate(&self) -> &Rc<dyn ScalarExpression> {
        &self.predicate
    }
}

impl LogicalOperator for LogicalFilter {
    fn name(&self) -> &str {
        "logical filter"
    }

    fn operator_id(&self) -> i16 {
        2
    }

    fn derive_statistics(&self, _md_accessor: &MdAccessor, input_stats: &[Rc<dyn Stats>]) -> Rc<dyn Stats> {
        input_stats[0].clone()
    }
}
