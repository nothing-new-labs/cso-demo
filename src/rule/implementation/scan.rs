use crate::operator::logical_scan::LogicalScan;
use crate::operator::physical_scan::PhysicalScan;
use crate::operator::OperatorId;
use crate::rule::RuleId;
use crate::{Demo, Pattern, PatternType};
use crate::{OptimizerContext, Plan};
use cso_core::operator::Operator;
use std::rc::Rc;

pub struct ScanImplementation {
    pattern: Pattern,
}

impl ScanImplementation {
    pub fn new() -> Self {
        ScanImplementation {
            pattern: Pattern::new(PatternType::Operator(OperatorId::LogicalScan)),
        }
    }
}

impl cso_core::rule::Rule<Demo> for ScanImplementation {
    fn name(&self) -> &str {
        "scan implementation"
    }

    fn rule_id(&self) -> RuleId {
        RuleId::ScanImplementation
    }

    fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    fn transform(&self, input: &Plan, _context: &mut OptimizerContext) -> Vec<Plan> {
        let logical_scan = input.operator().logical_op().downcast_ref::<LogicalScan>().unwrap();
        let physical_scan = PhysicalScan::new(
            logical_scan.table_desc().clone(),
            logical_scan.output_columns().to_vec(),
        );
        vec![Plan::new(
            Operator::Physical(Rc::new(physical_scan)),
            vec![],
            input.group_plan().cloned(),
        )]
    }

    fn is_implementation(&self) -> bool {
        true
    }
}
