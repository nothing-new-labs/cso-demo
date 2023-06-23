use crate::rule::Rule;
use crate::statistics::Statistics;
use crate::{LogicalPlan, Operator, Plan};
use bit_set::BitSet;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct GroupPlan {
    group: GroupWeakRef,
    op: Operator,
    inputs: Vec<GroupRef>,
    rule_masks: BitSet,
    stats_derived: bool,
}

pub type GroupPlanRef = Rc<RefCell<GroupPlan>>;

impl GroupPlan {
    pub fn new(op: Operator, inputs: Vec<GroupRef>) -> Self {
        GroupPlan {
            group: GroupWeakRef::new(),
            op,
            inputs,
            rule_masks: BitSet::new(),
            stats_derived: false,
        }
    }

    fn set_group(&mut self, group: GroupWeakRef) {
        self.group = group;
    }

    pub fn group(&self) -> GroupRef {
        self.group.upgrade().expect("expect the group is existing")
    }

    pub fn group_id(&self) -> u32 {
        self.group
            .upgrade()
            .expect("expect the group is existing")
            .borrow()
            .group_id()
    }

    pub fn operator(&self) -> &Operator {
        &self.op
    }

    pub fn inputs(&self) -> &[GroupRef] {
        &self.inputs
    }

    pub fn is_rule_explored(&self, rule: &dyn Rule) -> bool {
        self.rule_masks.contains(rule.rule_id() as usize)
    }

    pub fn is_stats_derived(&self) -> bool {
        self.stats_derived
    }

    pub fn set_stats_derived(&mut self) {
        self.stats_derived = true;
    }

    pub fn derive_statistics(&self) -> Statistics {
        let mut input_stats = Vec::with_capacity(self.inputs.len());

        for input in &self.inputs {
            let group = input.borrow();
            let stats = group.statistics();
            assert!(stats.is_some());
            input_stats.push(stats.clone().unwrap());
        }

        self.op.derive_statistics(input_stats.as_slice())
    }
}

pub struct Group {
    group_id: u32,
    logical_plans: Vec<GroupPlanRef>,
    physical_plans: Vec<GroupPlanRef>,
    is_explored: bool,
    statistics: Option<Rc<Statistics>>,
}

pub type GroupRef = Rc<RefCell<Group>>;
pub type GroupWeakRef = Weak<RefCell<Group>>;

impl Group {
    const fn new(group_id: u32) -> Self {
        Group {
            group_id,
            logical_plans: Vec::new(),
            physical_plans: Vec::new(),
            is_explored: false,
            statistics: None,
        }
    }

    pub fn group_id(&self) -> u32 {
        self.group_id
    }

    pub fn logical_plans(&self) -> &[GroupPlanRef] {
        &self.logical_plans
    }

    pub fn physical_plans(&self) -> &[GroupPlanRef] {
        &self.physical_plans
    }

    fn add_plan(this: &GroupRef, mut plan: GroupPlan) -> GroupPlanRef {
        plan.set_group(GroupRef::downgrade(this));
        match plan.op {
            Operator::Logical(_) => {
                let plan_ref = Rc::new(RefCell::new(plan));
                this.borrow_mut().logical_plans.push(plan_ref.clone());
                plan_ref
            }
            Operator::Physical(_) => {
                let plan_ref = Rc::new(RefCell::new(plan));
                this.borrow_mut().physical_plans.push(plan_ref.clone());
                plan_ref
            }
        }
    }

    pub fn is_explored(&self) -> bool {
        self.is_explored
    }

    pub fn set_explored(&mut self) {
        self.is_explored = true;
    }

    pub fn set_statistics(&mut self, stats: Statistics) {
        self.statistics = Some(Rc::new(stats));
    }

    pub fn update_statistics(&mut self, stats: Statistics) {
        match self.statistics {
            Some(ref old_stats) => {
                if Statistics::should_update(&stats, old_stats) {
                    self.set_statistics(stats)
                }
            }
            _ => self.set_statistics(stats),
        }
    }

    pub fn statistics(&self) -> &Option<Rc<Statistics>> {
        &self.statistics
    }
}

pub struct Memo {
    groups: Vec<GroupRef>,
    root_group: Option<GroupRef>,
    next_group_id: u32,
}

impl Memo {
    #[inline]
    pub const fn new() -> Self {
        Memo {
            groups: Vec::new(),
            root_group: None,
            next_group_id: 0,
        }
    }

    pub fn init(&mut self, plan: LogicalPlan) {
        let root_group = self.copy_in(None, plan);
        self.root_group = Some(root_group);
    }

    pub(crate) fn copy_in_plan(&mut self, target_group: Option<GroupRef>, plan: &Plan) -> GroupPlanRef {
        let mut inputs = Vec::new();
        for input in plan.inputs() {
            let group = match input.group_plan() {
                None => self.copy_in_plan(None, input).borrow().group(),
                Some(p) => p.borrow().group(),
            };

            inputs.push(group);
        }

        let group_plan = GroupPlan::new(plan.op.clone(), inputs);
        self.insert_group_plan(group_plan, target_group)
    }

    fn copy_in(&mut self, target_group: Option<GroupRef>, plan: LogicalPlan) -> GroupRef {
        let mut inputs = Vec::new();
        for input in plan.inputs {
            let group = self.copy_in(None, input);
            inputs.push(group);
        }

        let group_plan = GroupPlan::new(Operator::Logical(plan.op), inputs);
        let plan_ref = self.insert_group_plan(group_plan, target_group);
        let group = plan_ref.borrow().group();
        group
    }

    fn insert_group_plan(&mut self, plan: GroupPlan, target_group: Option<GroupRef>) -> GroupPlanRef {
        let target_group = match target_group {
            None => self.new_group(),
            Some(group) => group,
        };

        Group::add_plan(&target_group, plan)
    }

    #[inline]
    fn new_group(&mut self) -> GroupRef {
        let group = Rc::new(RefCell::new(Group::new(self.next_group_id)));
        self.next_group_id += 1;
        let group_clone = group.clone();
        self.groups.push(group);
        group_clone
    }

    pub fn root_group(&self) -> &GroupRef {
        self.root_group.as_ref().expect("expect the root group is existing")
    }
}
