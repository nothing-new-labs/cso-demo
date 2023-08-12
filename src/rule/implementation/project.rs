use crate::operator::logical_project::LogicalProject;
use crate::operator::physical_project::PhysicalProject;
use crate::operator::Operator;
use crate::rule::{Pattern, PatternType, Rule};
use crate::{OptimizerContext, Plan};
use std::rc::Rc;
use std::vec;

pub struct ProjectImplementation {
    pattern: Pattern,
}

impl ProjectImplementation {
    pub fn new() -> Self {
        ProjectImplementation {
            pattern: Pattern::with_children(PatternType::LogicalOperator(3), vec![Pattern::new(PatternType::Leaf)]),
        }
    }
}

impl Rule for ProjectImplementation {
    fn name(&self) -> &str {
        "project implementation"
    }

    fn rule_id(&self) -> u16 {
        3
    }

    fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    fn transform(&self, input: &Plan, _context: &mut OptimizerContext) -> Vec<Plan> {
        let logical_project = input
            .operator()
            .logical_op()
            .as_any()
            .downcast_ref::<LogicalProject>()
            .unwrap();
        let physical_project = PhysicalProject::new(logical_project.project().to_vec());
        vec![Plan::new(
            Operator::Physical(Rc::new(physical_project)),
            input.inputs().to_vec(),
        )]
    }
}
