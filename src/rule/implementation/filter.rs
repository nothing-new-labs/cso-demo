use crate::operator::logical_filter::LogicalFilter;
use crate::operator::physical_filter::PhysicalFilter;
use crate::operator::OperatorId;
use crate::rule::RuleId;
use crate::{Demo, OptimizerContext, Pattern, PatternType, Plan};
use cso_core::operator::Operator;
use std::rc::Rc;

pub struct FilterImplementation {
    pattern: Pattern,
}

impl FilterImplementation {
    pub fn new() -> Self {
        FilterImplementation {
            pattern: Pattern::with_children(
                PatternType::Operator(OperatorId::LogicalFilter),
                vec![Pattern::new(PatternType::Leaf)],
            ),
        }
    }
}

impl cso_core::rule::Rule<Demo> for FilterImplementation {
    fn name(&self) -> &str {
        "filter implementation"
    }

    fn rule_id(&self) -> RuleId {
        RuleId::FilterImplementation
    }

    fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    fn transform(&self, input: &Plan, _context: &mut OptimizerContext) -> Vec<Plan> {
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
