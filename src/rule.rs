use crate::memo::GroupPlan;
use crate::{LogicalPlan, Operator, OptimizerContext, PhysicalPlan};
use std::any::Any;

pub enum PatternType {
    LogicalOperator(i16),
    PhysicalOperator(i16),
    General,
    Leaf,
    MultiLeaf,
}

pub struct Pattern {
    pattern_type: PatternType,
    children: Vec<Pattern>,
}

impl Pattern {
    #[inline]
    pub const fn new(pattern_type: PatternType) -> Pattern {
        Pattern {
            pattern_type,
            children: Vec::new(),
        }
    }

    pub fn children(&self) -> &[Pattern] {
        &self.children
    }

    pub fn child(&self, index: usize) -> &Pattern {
        &self.children[index]
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self.pattern_type, PatternType::Leaf)
    }

    pub fn is_multi_leaf(&self) -> bool {
        matches!(self.pattern_type, PatternType::MultiLeaf)
    }

    pub fn is_leaf_or_multi_leaf(&self) -> bool {
        matches!(
            self.pattern_type,
            PatternType::Leaf | PatternType::MultiLeaf
        )
    }

    pub fn is_logical_operator(&self, operator_id: i16) -> bool {
        if let PatternType::LogicalOperator(id) = self.pattern_type {
            if id == operator_id {
                return true;
            }
        }
        false
    }

    pub fn is_physical_operator(&self, operator_id: i16) -> bool {
        if let PatternType::PhysicalOperator(id) = self.pattern_type {
            if id == operator_id {
                return true;
            }
        }
        false
    }

    pub fn match_without_child(&self, plan: &GroupPlan) -> bool {
        if plan.inputs().len() < self.children.len()
            && self.children.iter().all(|child| !child.is_multi_leaf())
        {
            return false;
        }

        if self.is_leaf_or_multi_leaf() {
            return true;
        }

        match plan.operator() {
            Operator::Logical(op) => self.is_logical_operator(op.operator_id()),
            Operator::Physical(op) => self.is_physical_operator(op.operator_id()),
        }
    }
}

pub trait Rule: Any {
    fn name(&self) -> &str;
    fn rule_id(&self) -> i16;
    fn pattern(&self) -> Pattern;
}

pub trait TransformRule: Rule {
    fn check(&self, _input: &LogicalPlan, _context: &OptimizerContext) -> bool {
        true
    }
    fn transform(&self, input: &LogicalPlan, context: &mut OptimizerContext) -> Vec<LogicalPlan>;
}

pub trait ImplementRule: Rule {
    fn check(&self, _input: &PhysicalPlan, _context: &OptimizerContext) -> bool {
        true
    }
    fn implement(&self, input: &PhysicalPlan, context: &mut OptimizerContext) -> Vec<PhysicalPlan>;
}
