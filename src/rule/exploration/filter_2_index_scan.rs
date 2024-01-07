use crate::expression::And;
use crate::operator::logical_filter::LogicalFilter;
use crate::operator::logical_index_scan::LogicalIndexScan;
use crate::operator::logical_scan::LogicalScan;
use crate::operator::OperatorId;
use crate::rule::RuleId;
use crate::statistics::{IndexMd, RelationMetadata};
use crate::{Demo, OptimizerContext, Pattern, Plan};
use cso_core::expression::ScalarExpression;
use cso_core::operator::Operator;
use cso_core::rule::{PatternType, Rule};
use cso_core::ColumnRefSet;
use std::rc::Rc;

pub struct Filter2IndexScan {
    pattern: Pattern,
}

impl Filter2IndexScan {
    pub fn new() -> Self {
        let pattern = Pattern::with_children(
            PatternType::Operator(OperatorId::LogicalFilter),
            vec![Pattern::new(PatternType::Operator(OperatorId::LogicalScan))],
        );
        Self { pattern }
    }
}

impl Rule<Demo> for Filter2IndexScan {
    fn name(&self) -> &str {
        "Filter2IndexScan"
    }

    fn rule_id(&self) -> RuleId {
        RuleId::Filter2IndexScan
    }

    fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    fn transform(&self, input: &Plan, context: &mut OptimizerContext) -> Vec<Plan> {
        let logical_filter = input
            .operator()
            .logical_op()
            .downcast_ref::<LogicalFilter>()
            .expect("LogicalFilter expected!");

        debug_assert_eq!(input.inputs().len(), 1);
        let logical_scan = input.inputs()[0]
            .operator()
            .logical_op()
            .downcast_ref::<LogicalScan>()
            .expect("LogicalScan expected!");

        let table_desc = logical_scan.table_desc();
        let md_accessor = context.md_accessor();
        let relation_md = md_accessor
            .retrieve_metadata(&table_desc.md_id())
            .expect("Relation metadata missed!");
        let relation_md = relation_md.downcast_ref::<RelationMetadata>().unwrap();

        let mut new_plans = vec![];
        for i in 0..relation_md.index_count() {
            let index_mdid = relation_md.index_mdid(i);
            let index_md = md_accessor
                .retrieve_metadata(&index_mdid)
                .expect("Index metadata missed!");
            let index_md = index_md.downcast_ref::<IndexMd>().unwrap();
            let predicate = logical_filter.predicate();
            let (applicable, residual_predicate) = index_matched(index_md, predicate.as_ref());
            if applicable {
                let logical_index_scan = LogicalIndexScan::new(
                    table_desc.clone(),
                    index_md,
                    logical_scan.output_columns().to_vec(),
                    predicate.clone(),
                );
                let mut new_plan = Plan::new(Operator::Logical(Rc::new(logical_index_scan)), vec![], None);

                if let Some(new_logical_filter) = residual_predicate {
                    new_plan = Plan::new(Operator::Logical(Rc::new(new_logical_filter)), vec![new_plan], None);
                }

                new_plans.push(new_plan);
            }
        }
        new_plans
    }

    fn is_transformation(&self) -> bool {
        true
    }
}

fn index_matched(index_md: &IndexMd, predicate: &dyn ScalarExpression) -> (bool, Option<LogicalFilter>) {
    let mut predicate_include_columns = ColumnRefSet::new();
    predicate.derive_used_columns(&mut predicate_include_columns);
    let mut index_include_columns = ColumnRefSet::new();
    for index_key in index_md.included_columns() {
        index_key.derive_used_columns(&mut index_include_columns);
    }
    // index type is only btree now, so no need to judge
    if predicate_include_columns.is_superset(&index_include_columns) {
        (true, None)
    } else {
        // split predicates into index part and not index part
        let mut residual_predicate = Vec::new();
        let mut applicable = false;
        for expression in predicate.split_predicates() {
            let mut predicate_include_columns = ColumnRefSet::new();
            expression.derive_used_columns(&mut predicate_include_columns);
            if index_include_columns.is_disjoint(&predicate_include_columns) {
                residual_predicate.push(expression);
            } else {
                applicable = true;
            }
        }
        if applicable {
            let new_logical_filter = LogicalFilter::new(Rc::new(And::new(residual_predicate)));
            (applicable, Some(new_logical_filter))
        } else {
            (false, None)
        }
    }
}
