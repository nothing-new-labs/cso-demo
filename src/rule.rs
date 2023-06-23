use crate::memo::{Group, GroupPlan, GroupPlanRef};
use crate::{Operator, OptimizerContext, Plan};
use std::any::Any;
use std::ops::Deref;
use std::rc::Rc;

pub enum PatternType {
    LogicalOperator(i16),
    Tree,
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
        matches!(self.pattern_type, PatternType::Leaf | PatternType::MultiLeaf)
    }

    pub fn is_logical_operator(&self, operator_id: i16) -> bool {
        if let PatternType::LogicalOperator(id) = self.pattern_type {
            if id == operator_id {
                return true;
            }
        }
        false
    }

    pub fn match_without_child(&self, plan: &GroupPlan) -> bool {
        if plan.inputs().len() < self.children.len() && self.children.iter().all(|child| !child.is_multi_leaf()) {
            return false;
        }

        if self.is_leaf_or_multi_leaf() {
            return true;
        }

        match plan.operator() {
            Operator::Logical(op) => self.is_logical_operator(op.operator_id()),
            Operator::Physical(_op) => false,
        }
    }
}

pub trait Rule: Any {
    fn name(&self) -> &str;
    fn rule_id(&self) -> u16;
    fn pattern(&self) -> &Pattern;
    fn check(&self, input: &Plan, context: &OptimizerContext) -> bool;
    fn transform(&self, input: &Plan, context: &mut OptimizerContext) -> Vec<Plan>;

    fn promise(&self) -> i32 {
        1
    }

    fn need_statistics(&self) -> bool {
        false
    }

    fn apply_once(&self) -> bool {
        false
    }
}

pub type RuleRef = Rc<dyn Rule>;

#[derive(Default)]
pub struct RuleSet {
    transform_rules: Vec<RuleRef>,
    implement_rules: Vec<RuleRef>,
}

impl RuleSet {
    pub const fn new() -> Self {
        RuleSet {
            transform_rules: vec![],
            implement_rules: vec![],
        }
    }

    pub fn transform_rules(&self) -> &[RuleRef] {
        &self.transform_rules
    }

    pub fn implement_rules(&self) -> &[RuleRef] {
        &self.implement_rules
    }
}

pub struct Binding<'a> {
    pattern: &'a Pattern,
    plan: &'a GroupPlan,
    group_trace_id: usize,
    group_plan_index: Vec<u32>,
}

impl<'a> Binding<'a> {
    pub fn new(pattern: &'a Pattern, plan: &'a GroupPlan) -> Self {
        Binding {
            pattern,
            plan,
            group_trace_id: 0,
            group_plan_index: vec![0],
        }
    }

    fn extract_group_plan(&mut self, pattern: &Pattern, group: &Group) -> Option<GroupPlanRef> {
        if pattern.is_leaf_or_multi_leaf() {
            if self.group_plan_index[self.group_trace_id] > 0 {
                self.group_plan_index.remove(self.group_trace_id);
                None
            } else {
                Some(group.logical_plans()[0].clone())
            }
        } else {
            let id = self.group_plan_index[self.group_trace_id];
            if id as usize >= group.logical_plans().len() {
                self.group_plan_index.remove(self.group_trace_id);
                None
            } else {
                Some(group.logical_plans()[id as usize].clone())
            }
        }
    }

    fn matches(&mut self, pattern: &Pattern, group_plan: &GroupPlan) -> Option<Plan> {
        if !pattern.match_without_child(group_plan) {
            return None;
        }

        let mut inputs = Vec::new();
        let mut pattern_index = 0;
        let mut group_plan_index = 0;

        while pattern_index < pattern.children().len() && group_plan_index < group_plan.inputs().len() {
            self.group_trace_id += 1;
            self.group_plan_index.resize(self.group_trace_id + 1, 0);

            let group = &*group_plan.inputs()[group_plan_index].borrow();
            let child_pattern = pattern.child(pattern_index);

            let current_plan = self.extract_group_plan(child_pattern, group)?;
            let child_plan = self.matches(child_pattern, current_plan.borrow().deref())?;
            inputs.push(child_plan);

            if !(child_pattern.is_multi_leaf()
                && (group_plan.inputs().len() - group_plan_index > pattern.children.len() - pattern_index))
            {
                pattern_index += 1;
            }
            group_plan_index += 1;
        }

        Some(Plan::new(group_plan.operator().clone(), inputs))
    }

    fn next(&mut self) -> Option<Plan> {
        if self.pattern.children().is_empty() && self.group_plan_index[0] > 0 {
            return None;
        }

        let plan = None;

        loop {
            self.group_trace_id = 0;
            if let Some(last) = self.group_plan_index.last_mut() {
                *last += 1;
            }

            let plan = self.matches(self.pattern, self.plan);
            if plan.is_some() || self.group_plan_index.len() != 1 {
                break;
            }
        }

        plan
    }
}

impl<'a> Iterator for Binding<'a> {
    type Item = Plan;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
