use crate::operator::logical_index_scan::LogicalIndexScan;
use crate::operator::physical_index_scan::PhysicalIndexScan;
use crate::operator::OperatorId;
use crate::rule::RuleId;
use crate::{Demo, Pattern, PatternType};
use crate::{OptimizerContext, Plan};
use cso_core::operator::Operator;
use std::rc::Rc;

pub struct IndexScanImplementation {
    pattern: Pattern,
}

impl IndexScanImplementation {
    pub fn new() -> Self {
        IndexScanImplementation {
            pattern: Pattern::new(PatternType::Operator(OperatorId::LogicalIndexScan)),
        }
    }
}

impl cso_core::rule::Rule<Demo> for IndexScanImplementation {
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
        let logical_index_scan = input
            .operator()
            .logical_op()
            .downcast_ref::<LogicalIndexScan>()
            .unwrap();
        let physical_index_scan = PhysicalIndexScan::new(
            logical_index_scan.index_desc().clone(),
            logical_index_scan.table_desc().clone(),
            logical_index_scan.output_columns().to_vec(),
            logical_index_scan.predicate().clone(),
        );
        vec![Plan::new(
            Operator::Physical(Rc::new(physical_index_scan)),
            vec![],
            input.group_plan().cloned(),
        )]
    }

    fn is_implementation(&self) -> bool {
        true
    }
}
