use crate::operator::logical_filter::LogicalFilter;
use crate::operator::physical_filter::PhysicalFilter;
use crate::rule::RuleId;
use crate::Demo;
use cso_core::operator::Operator;
use cso_core::rule::{Pattern, PatternType, Rule};
use cso_core::{OptimizerContext, Plan};
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
    type OptimizerType = Demo;

    fn name(&self) -> &str {
        "filter implementation"
    }

    fn rule_id(&self) -> RuleId {
        RuleId::FilterImplementation
    }

    fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    fn transform(&self, input: &Plan, _context: &mut OptimizerContext<Demo>) -> Vec<Plan> {
        let logical_filter = input.operator().logical_op().downcast_ref::<LogicalFilter>().unwrap();
        let physical_filter = PhysicalFilter::new(logical_filter.predicate().clone());
        vec![Plan::new(
            Operator::Physical(Rc::new(physical_filter)),
            input.inputs().to_vec(),
            input.group_plan().cloned(),
        )]
    }

    fn is_implementation(&self) -> bool {
        true
    }
}
