use crate::{LogicalPlan, Operator};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct GroupPlan {
    group: GroupWeakRef,
    op: Operator,
    inputs: Vec<GroupRef>,
}

pub type GroupPlanRef = Rc<RefCell<GroupPlan>>;

impl GroupPlan {
    pub fn new(op: Operator, inputs: Vec<GroupRef>) -> Self {
        GroupPlan {
            group: GroupWeakRef::new(),
            op,
            inputs,
        }
    }

    fn set_group(&mut self, group: GroupWeakRef) {
        self.group = group;
    }

    pub fn group_id(&self) -> u32 {
        self.group
            .upgrade()
            .expect("expected group is existing")
            .borrow()
            .group_id()
    }

    pub fn inputs(&self) -> &[GroupRef] {
        &self.inputs
    }
}

pub struct Group {
    group_id: u32,
    logical_plans: Vec<GroupPlanRef>,
    physical_plans: Vec<GroupPlanRef>,
    is_explored: bool,
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

    fn add_plan(this: &GroupRef, mut plan: GroupPlan) {
        plan.set_group(GroupRef::downgrade(this));
        match plan.op {
            Operator::Logical(_) => {
                this.borrow_mut()
                    .logical_plans
                    .push(Rc::new(RefCell::new(plan)));
            }
            Operator::Physical(_) => {
                this.borrow_mut()
                    .physical_plans
                    .push(Rc::new(RefCell::new(plan)));
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

    fn copy_in(&mut self, target_group: Option<GroupRef>, plan: LogicalPlan) -> GroupRef {
        let mut inputs = Vec::new();
        for input in plan.inputs {
            let group = self.copy_in(None, input);
            inputs.push(group);
        }

        let group_plan = GroupPlan::new(Operator::Logical(plan.op), inputs);
        self.insert_group_plan(group_plan, target_group)
    }

    fn insert_group_plan(&mut self, plan: GroupPlan, target_group: Option<GroupRef>) -> GroupRef {
        let target_group = match target_group {
            None => self.new_group(),
            Some(group) => group,
        };

        Group::add_plan(&target_group, plan);
        target_group
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
        self.root_group
            .as_ref()
            .expect("expected root group is existing")
    }
}
