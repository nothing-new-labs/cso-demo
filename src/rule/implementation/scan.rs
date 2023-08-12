use crate::operator::logical_scan::LogicalScan;
use crate::operator::physical_scan::PhysicalScan;
use crate::operator::Operator;
use crate::rule::{Pattern, PatternType, Rule};
use crate::{OptimizerContext, Plan};
use std::rc::Rc;

pub struct ScanImplementation {
    pattern: Pattern,
}

impl ScanImplementation {
    pub fn new() -> Self {
        ScanImplementation {
            pattern: Pattern::new(PatternType::LogicalOperator(1)),
        }
    }
}

impl Rule for ScanImplementation {
    fn name(&self) -> &str {
        "scan implementation"
    }

    fn rule_id(&self) -> u16 {
        1
    }

    fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    fn transform(&self, input: &Plan, _context: &mut OptimizerContext) -> Vec<Plan> {
        let logical_scan = input
            .operator()
            .logical_op()
            .as_any()
            .downcast_ref::<LogicalScan>()
            .unwrap();
        let physical_scan = PhysicalScan::new(
            logical_scan.table_desc().clone(),
            logical_scan.output_columns().to_vec(),
        );
        vec![Plan::new(Operator::Physical(Rc::new(physical_scan)), vec![])]
    }
}
