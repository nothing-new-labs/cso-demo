use crate::memo::{Group, GroupPlan, GroupPlanRef};
use crate::operator::Operator;
use crate::{OptimizerContext, OptimizerType, Plan};
use std::any::Any;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

pub enum PatternType<T: OptimizerType> {
    Operator(T::OperatorId),
    Tree,
    Leaf,
    MultiLeaf,
}

pub struct Pattern<T: OptimizerType> {
    pattern_type: PatternType<T>,
    children: Vec<Pattern<T>>,
}

impl<T: OptimizerType> Pattern<T> {
    #[inline]
    pub const fn new(pattern_type: PatternType<T>) -> Pattern<T> {
        Pattern {
            pattern_type,
            children: Vec::new(),
        }
    }

    pub const fn with_children(pattern_type: PatternType<T>, children: Vec<Pattern<T>>) -> Self {
        Pattern { pattern_type, children }
    }

    pub fn children(&self) -> &[Pattern<T>] {
        &self.children
    }

    pub fn child(&self, index: usize) -> &Pattern<T> {
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

    pub fn is_logical_operator(&self, operator_id: &T::OperatorId) -> bool {
        if let PatternType::Operator(ref id) = self.pattern_type {
            if id == operator_id {
                return true;
            }
        }
        false
    }

    pub fn match_without_child(&self, plan: &GroupPlan<T>) -> bool {
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

pub trait RuleId: Copy + PartialEq + Debug {
    fn as_usize(self) -> usize;
}

pub trait Rule<T: OptimizerType>: Any {
    fn name(&self) -> &str;
    fn rule_id(&self) -> T::RuleId;
    fn pattern(&self) -> &Pattern<T>;
    fn transform(&self, input: &Plan<T>, context: &mut OptimizerContext<T>) -> Vec<Plan<T>>;

    fn check(&self, _input: &Plan<T>, _context: &OptimizerContext<T>) -> bool {
        true
    }

    fn promise(&self) -> i32 {
        1
    }

    fn need_statistics(&self) -> bool {
        false
    }

    fn apply_once(&self) -> bool {
        false
    }

    fn is_implementation(&self) -> bool {
        false
    }

    fn is_transformation(&self) -> bool {
        false
    }
}

pub type RuleRef<T> = Rc<dyn Rule<T>>;

pub struct RuleSet<T: OptimizerType> {
    transform_rules: Vec<RuleRef<T>>,
    implement_rules: Vec<RuleRef<T>>,
}

impl<T: OptimizerType> RuleSet<T> {
    pub fn new() -> Self {
        RuleSet {
            transform_rules: vec![],
            implement_rules: vec![],
        }
    }

    pub fn transform_rules(&self) -> &[RuleRef<T>] {
        &self.transform_rules
    }

    pub fn implement_rules(&self) -> &[RuleRef<T>] {
        &self.implement_rules
    }

    pub fn set_transform_rules(&mut self, rules: Vec<RuleRef<T>>) {
        assert!(rules.iter().all(|x| { x.is_transformation() }));
        self.transform_rules = rules;
    }

    pub fn set_implement_rules(&mut self, rules: Vec<RuleRef<T>>) {
        assert!(rules.iter().all(|x| { x.is_implementation() }));
        self.transform_rules = rules;
    }
}

pub(crate) struct Binding<'a, T: OptimizerType> {
    pattern: &'a Pattern<T>,
    plan: &'a GroupPlanRef<T>,
    group_trace_id: usize,
    group_plan_index: Vec<u32>,
}

impl<'a, T: OptimizerType> Binding<'a, T> {
    pub fn new(pattern: &'a Pattern<T>, plan: &'a GroupPlanRef<T>) -> Self {
        Binding {
            pattern,
            plan,
            group_trace_id: 0,
            group_plan_index: vec![0],
        }
    }

    fn extract_group_plan(&mut self, pattern: &Pattern<T>, group: &Group<T>) -> Option<GroupPlanRef<T>> {
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

    fn matches(&mut self, pattern: &Pattern<T>, group_plan: &GroupPlanRef<T>) -> Option<Plan<T>> {
        let curr_plan = group_plan.borrow();

        if !pattern.match_without_child(curr_plan.deref()) {
            return None;
        }

        let mut inputs = Vec::new();
        let mut pattern_index = 0;
        let mut group_plan_index = 0;

        while pattern_index < pattern.children().len() && group_plan_index < curr_plan.inputs().len() {
            self.group_trace_id += 1;
            self.group_plan_index.resize(self.group_trace_id + 1, 0);

            let group = &*curr_plan.inputs()[group_plan_index].borrow();
            let child_pattern = pattern.child(pattern_index);

            let extracted_plan = self.extract_group_plan(child_pattern, group)?;
            let child_plan = self.matches(child_pattern, &extracted_plan)?;
            inputs.push(child_plan);

            if !(child_pattern.is_multi_leaf()
                && (curr_plan.inputs().len() - group_plan_index > pattern.children.len() - pattern_index))
            {
                pattern_index += 1;
            }
            group_plan_index += 1;
        }

        Some(Plan::new(
            curr_plan.operator().clone(),
            inputs,
            Some(group_plan.clone()),
        ))
    }

    fn next(&mut self) -> Option<Plan<T>> {
        if self.pattern.children().is_empty() && self.group_plan_index[0] > 0 {
            return None;
        }

        loop {
            self.group_trace_id = 0;
            if let Some(last) = self.group_plan_index.last_mut() {
                *last += 1;
            }

            let plan = self.matches(self.pattern, self.plan);
            if plan.is_some() || self.group_plan_index.len() == 1 {
                return plan;
            }
        }
    }
}

impl<'a, T: OptimizerType> Iterator for Binding<'a, T> {
    type Item = Plan<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
