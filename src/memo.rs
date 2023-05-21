use crate::{LogicalPlan, Operator};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(transparent)]
pub struct GroupId(i32);

impl GroupId {
    const INVALID: GroupId = GroupId(-1);

    pub const fn new(value: usize) -> Self {
        assert!(value <= i32::MAX as usize);
        GroupId(value as i32)
    }

    #[inline]
    const fn is_valid(&self) -> bool {
        self.0 >= 0
    }
}

#[derive(Copy, Clone)]
pub struct GroupPlanId {
    plan_type_id: i32, // logical: 0, physical: 1
    plan_id: i32,
}

impl GroupPlanId {
    const INVALID: GroupPlanId = GroupPlanId {
        plan_type_id: -1,
        plan_id: -1,
    };

    pub const fn new(is_physical: bool, plan_id: usize) -> Self {
        assert!(plan_id <= i32::MAX as usize);
        GroupPlanId {
            plan_type_id: is_physical as i32,
            plan_id: plan_id as i32,
        }
    }
}

impl Index<GroupPlanId> for Group {
    type Output = GroupPlan;

    fn index(&self, index: GroupPlanId) -> &Self::Output {
        &self.plans[index.plan_type_id as usize][index.plan_id as usize]
    }
}

impl IndexMut<GroupPlanId> for Group {
    fn index_mut(&mut self, index: GroupPlanId) -> &mut Self::Output {
        &mut self.plans[index.plan_type_id as usize][index.plan_id as usize]
    }
}

pub struct GroupPlan {
    group_id: GroupId,
    plan_id: GroupPlanId,
    op: Operator,
    inputs: Vec<GroupId>,
}

impl GroupPlan {
    pub const fn new(op: Operator, inputs: Vec<GroupId>) -> Self {
        GroupPlan {
            group_id: GroupId::INVALID,
            plan_id: GroupPlanId::INVALID,
            op,
            inputs,
        }
    }

    fn set_group_id(&mut self, group_id: GroupId) {
        self.group_id = group_id;
    }

    pub fn group_id(&self) -> GroupId {
        self.group_id
    }

    fn set_plan_id(&mut self, plan_id: GroupPlanId) {
        self.plan_id = plan_id;
    }

    pub fn plan_id(&self) -> GroupPlanId {
        self.plan_id
    }

    pub fn inputs(&self) -> &[GroupId] {
        &self.inputs
    }
}

pub struct Group {
    group_id: GroupId,
    // plans[0] is logical and plans[1] is physical
    plans: [Vec<GroupPlan>; 2],
    is_explored: bool,
}

impl Group {
    const fn new(group_id: GroupId) -> Self {
        debug_assert!(group_id.is_valid());
        Group {
            group_id,
            plans: [Vec::new(), Vec::new()],
            is_explored: false,
        }
    }

    pub fn logical_plans(&self) -> &[GroupPlan] {
        &self.plans[0]
    }

    pub fn physical_plans(&self) -> &[GroupPlan] {
        &self.plans[1]
    }

    fn add_plan(&mut self, mut plan: GroupPlan) {
        plan.set_group_id(self.group_id);
        match plan.op {
            Operator::Logical(_) => {
                let plan_id = GroupPlanId::new(false, self.plans[0].len());
                plan.set_plan_id(plan_id);
                self.plans[0].push(plan);
            }
            Operator::Physical(_) => {
                let plan_id = GroupPlanId::new(true, self.plans[1].len());
                plan.set_plan_id(plan_id);
                self.plans[1].push(plan);
            }
        }
    }

    pub fn is_explored(&self) -> bool {
        self.is_explored
    }

    pub fn set_explored(&mut self) {
        self.is_explored = true;
    }
}

pub struct Memo {
    groups: Vec<Group>,
    root_group_id: GroupId,
}

impl Index<GroupId> for Memo {
    type Output = Group;

    #[inline]
    fn index(&self, index: GroupId) -> &Self::Output {
        debug_assert!(index.is_valid());
        &self.groups[index.0 as usize]
    }
}

impl IndexMut<GroupId> for Memo {
    #[inline]
    fn index_mut(&mut self, index: GroupId) -> &mut Self::Output {
        debug_assert!(index.is_valid());
        &mut self.groups[index.0 as usize]
    }
}

impl Memo {
    #[inline]
    pub const fn new() -> Self {
        Memo {
            groups: Vec::new(),
            root_group_id: GroupId::INVALID,
        }
    }

    pub fn init(&mut self, plan: LogicalPlan) {
        let root_group_id = self.copy_in(None, plan);
        self.root_group_id = root_group_id;
    }

    fn copy_in(&mut self, target_group_id: Option<GroupId>, plan: LogicalPlan) -> GroupId {
        let mut inputs = Vec::new();
        for input in plan.inputs {
            let group_id = self.copy_in(None, input);
            inputs.push(group_id);
        }

        let group_plan = GroupPlan::new(Operator::Logical(plan.op), inputs);
        self.insert_group_plan(group_plan, target_group_id)
    }

    fn insert_group_plan(&mut self, plan: GroupPlan, target_group_id: Option<GroupId>) -> GroupId {
        let target_group = match target_group_id {
            None => self.new_group(),
            Some(id) => &mut self[id],
        };

        target_group.add_plan(plan);
        target_group.group_id
    }

    #[inline]
    fn new_group(&mut self) -> &mut Group {
        let group_id = GroupId::new(self.groups.len());
        let group = Group::new(group_id);
        self.groups.push(group);
        &mut self[group_id]
    }

    pub fn root_group_id(&self) -> GroupId {
        self.root_group_id
    }
}
