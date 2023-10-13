use crate::operator::logical_project::LogicalProject;
use crate::operator::physical_project::PhysicalProject;
use crate::operator::OperatorId;
use crate::rule::RuleId;
use crate::{Demo, Pattern, PatternType};
use crate::{OptimizerContext, Plan};
use cso_core::operator::Operator;
use std::rc::Rc;
use std::vec;

pub struct ProjectImplementation {
    pattern: Pattern,
}

impl ProjectImplementation {
    pub fn new() -> Self {
        ProjectImplementation {
            pattern: Pattern::with_children(
                PatternType::Operator(OperatorId::LogicalProject),
                vec![Pattern::new(PatternType::Leaf)],
            ),
        }
    }
}

impl cso_core::rule::Rule for ProjectImplementation {
    type OptimizerType = Demo;

    fn name(&self) -> &str {
        "project implementation"
    }

    fn rule_id(&self) -> RuleId {
        RuleId::ProjectImplementation
    }

    fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    fn transform(&self, input: &Plan, _context: &mut OptimizerContext) -> Vec<Plan> {
        let logical_project = input.operator().logical_op().downcast_ref::<LogicalProject>().unwrap();
        let physical_project = PhysicalProject::new(logical_project.project().to_vec());
        vec![Plan::new(
            Operator::Physical(Rc::new(physical_project)),
            input.inputs().to_vec(),
            input.group_plan().cloned(),
        )]
    }

    fn is_implementation(&self) -> bool {
        true
    }
}
