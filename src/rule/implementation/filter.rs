use crate::operator::logical_filter::LogicalFilter;
use crate::operator::physical_filter::PhysicalFilter;
use crate::operator::Operator;
use crate::rule::{Pattern, PatternType, Rule};
use crate::{OptimizerContext, Plan};
use std::rc::Rc;

pub struct FilterImplementation {
    pattern: Pattern,
}

impl FilterImplementation {
    pub fn new() -> Self {
        FilterImplementation {
            pattern: Pattern::with_children(PatternType::LogicalOperator(2), vec![Pattern::new(PatternType::Leaf)]),
        }
    }
}

impl Rule for FilterImplementation {
    fn name(&self) -> &str {
        "filter implementation"
    }

    fn rule_id(&self) -> u16 {
        2
    }

    fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    fn transform(&self, input: &Plan, _context: &mut OptimizerContext) -> Vec<Plan> {
        let logical_filter = input
            .operator()
            .logical_op()
            .as_any()
            .downcast_ref::<LogicalFilter>()
            .unwrap();
        let physical_filter = PhysicalFilter::new(logical_filter.predicate().clone());
        vec![Plan::new(
            Operator::Physical(Rc::new(physical_filter)),
            input.inputs().to_vec(),
        )]
    }
}
