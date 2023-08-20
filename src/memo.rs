use crate::cost::Cost;
use crate::operator::Operator;
use crate::property::PhysicalProperties;
use crate::rule::Rule;
use crate::statistics::Statistics;
use crate::{LogicalPlan, OptimizerContext, Plan};
use bit_set::BitSet;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type RequireToOutputMap = HashMap<PhysicalProperties, Rc<PhysicalProperties>>;

pub struct GroupPlan {
    group: GroupWeakRef,
    op: Operator,
    inputs: Vec<GroupRef>,
    rule_masks: BitSet,
    require_to_output_map: RequireToOutputMap,
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
            require_to_output_map: HashMap::new(),
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

    pub fn derive_statistics(&self, optimizer_ctx: &OptimizerContext) -> Statistics {
        let mut input_stats = Vec::with_capacity(self.inputs.len());

        for input in &self.inputs {
            let group = input.borrow();
            let stats = group.statistics();
            assert!(stats.is_some());
            input_stats.push(stats.clone().unwrap());
        }

        let md_accessor = optimizer_ctx.md_accessor();
        let input_stats = input_stats.as_slice();
        self.op.logical_op().derive_statistics(md_accessor, input_stats)
    }

    pub fn get_output_prop(&self, reqd_prop: &PhysicalProperties) -> &Rc<PhysicalProperties> {
        self.require_to_output_map.get(reqd_prop).expect("output not null")
    }

    pub fn compute_cost(&self) -> Cost {
        // todo: add compute cost to operator
        Cost::new()
    }
}

type LowestCostPlans = HashMap<Rc<PhysicalProperties>, (Cost, GroupPlanRef)>;

pub struct Group {
    group_id: u32,
    logical_plans: Vec<GroupPlanRef>,
    physical_plans: Vec<GroupPlanRef>,
    is_explored: bool,
    statistics: Option<Rc<Statistics>>,
    lowest_cost_plans: LowestCostPlans,
}

pub type GroupRef = Rc<RefCell<Group>>;
pub type GroupWeakRef = Weak<RefCell<Group>>;

impl Group {
    fn new(group_id: u32) -> Self {
        Group {
            group_id,
            logical_plans: Vec::new(),
            physical_plans: Vec::new(),
            is_explored: false,
            statistics: None,
            lowest_cost_plans: HashMap::new(),
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

    pub fn lowest_cost_plans(&self) -> &HashMap<Rc<PhysicalProperties>, (Cost, GroupPlanRef)> {
        &self.lowest_cost_plans
    }

    pub fn lowest_cost_plans_mut(&mut self) -> &mut HashMap<Rc<PhysicalProperties>, (Cost, GroupPlanRef)> {
        &mut self.lowest_cost_plans
    }

    pub fn update_cost_plan(
        &mut self,
        required_prop: &Rc<PhysicalProperties>,
        curr_plan: &GroupPlanRef,
        curr_cost: Cost,
    ) {
        if let Some((cost, _group_plan)) = self.lowest_cost_plans.get(required_prop) {
            // if current cost is larger, do nothing
            if curr_cost.value() >= cost.value() {
                return;
            }
        }
        // update lowest_cost_plans
        self.lowest_cost_plans
            .insert(required_prop.clone(), (curr_cost, curr_plan.clone()));
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

    pub fn insert_group_plan(&mut self, plan: GroupPlan, target_group: Option<GroupRef>) -> GroupPlanRef {
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
