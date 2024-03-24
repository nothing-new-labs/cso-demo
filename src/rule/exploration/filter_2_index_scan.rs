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

        let predicate = logical_filter.predicate();
        let mut filter_predicate_columns = ColumnRefSet::new();
        predicate.derive_used_columns(&mut filter_predicate_columns);
        let mut filter_required_columns = ColumnRefSet::new();
        input.derive_output_columns(&mut filter_required_columns);
        filter_required_columns.union_with(&filter_predicate_columns);

        let predicates = logical_filter.split_predicate();
        let mut new_plans = vec![];
        for i in 0..relation_md.index_count() {
            let index_mdid = relation_md.index_mdid(i);
            let index_md = md_accessor
                .retrieve_metadata(&index_mdid)
                .expect("Index metadata missed!");
            let index_md = index_md.downcast_ref::<IndexMd>().unwrap();

            if let Some((applicable_predicates, residual_predicates)) = index_matched(
                index_md,
                &predicates,
                &filter_required_columns,
                &filter_predicate_columns,
            ) {
                let logical_index_scan = LogicalIndexScan::new(
                    table_desc.clone(),
                    index_md,
                    logical_scan.output_columns().to_vec(),
                    applicable_predicates,
                );
                let index_scan_plan = Plan::new(Operator::Logical(Rc::new(logical_index_scan)), vec![], None);

                if let Some(residual_predicates) = residual_predicates {
                    let logical_filter = LogicalFilter::new(residual_predicates);
                    let filter_plan =
                        Plan::new(Operator::Logical(Rc::new(logical_filter)), vec![index_scan_plan], None);
                    new_plans.push(filter_plan);
                } else {
                    new_plans.push(index_scan_plan);
                }
            }
        }
        new_plans
    }

    fn is_transformation(&self) -> bool {
        true
    }
}

type ApplicableAndResidualPredicates = (Rc<dyn ScalarExpression>, Option<Rc<dyn ScalarExpression>>);

fn index_matched(
    index_md: &IndexMd,
    predicates: &[Rc<dyn ScalarExpression>],
    required_columns: &ColumnRefSet,
    predicate_columns: &ColumnRefSet,
) -> Option<ApplicableAndResidualPredicates> {
    let mut key_columns = ColumnRefSet::new();
    index_md
        .key_columns()
        .iter()
        .for_each(|key| key.derive_used_columns(&mut key_columns));

    let mut include_columns = ColumnRefSet::new();
    index_md
        .included_columns()
        .iter()
        .for_each(|key| key.derive_used_columns(&mut include_columns));

    if !include_columns.is_superset(required_columns) || key_columns.is_disjoint(predicate_columns) {
        return None;
    }

    let mut residual_predicates = Vec::new();
    let mut applicable_predicates = Vec::new();
    for expr in predicates {
        let mut used_columns = ColumnRefSet::new();
        expr.derive_used_columns(&mut used_columns);
        if key_columns.is_superset(&used_columns) {
            applicable_predicates.push(expr.clone());
        } else {
            // For now, we are not considering complex indexing scenarios. We simply assume that
            // when key_columns contain predicate used_columns, the current expr can use this index.
            residual_predicates.push(expr.clone());
        }
    }
    debug_assert!(!applicable_predicates.is_empty());
    if residual_predicates.is_empty() {
        Some((Rc::new(And::new(applicable_predicates)), None))
    } else {
        Some((
            Rc::new(And::new(applicable_predicates)),
            Some(Rc::new(And::new(residual_predicates))),
        ))
    }
}
