use crate::cost::Cost;
use crate::memo::{GroupPlanRef, Memo};
use crate::property::PhysicalProperties;
use crate::task::{OptimizeGroupTask, Task, TaskRunner};
use crate::OptimizerContext;
use std::rc::Rc;

pub struct EnforceAndCostTask {
    plan: GroupPlanRef,
    required_prop: Rc<PhysicalProperties>,
    prev_index: usize,
}

impl From<EnforceAndCostTask> for Task {
    #[inline]
    fn from(task: EnforceAndCostTask) -> Self {
        Task::EnforceAndCost(task)
    }
}

impl Clone for EnforceAndCostTask {
    fn clone(&self) -> EnforceAndCostTask {
        todo!()
    }
}

impl EnforceAndCostTask {
    pub const fn new(new_plan: GroupPlanRef, new_required_prop: Rc<PhysicalProperties>) -> Self {
        EnforceAndCostTask {
            plan: new_plan,
            required_prop: new_required_prop,
            prev_index: 0,
        }
    }

    fn make_child_required_props_list(&self) -> Vec<Vec<Rc<PhysicalProperties>>> {
        self.plan
            .borrow()
            .operator()
            .physical_op()
            .required_properties(self.required_prop.clone())
    }

    fn add_enforcer_to_group(&self, required_prop: &PhysicalProperties, memo: &mut Memo) -> GroupPlanRef {
        let group = self.plan.borrow().group();
        let group_plan = required_prop.make_enforcer(group.clone());
        memo.insert_group_plan(group_plan, Some(group))
    }

    fn prev_index(&self) -> usize {
        self.prev_index
    }

    fn submit_cost_plan(&self, child_output_props: &[Rc<PhysicalProperties>], memo: &mut Memo) -> Cost {
        let curr_plan = self.plan.borrow();
        let output_prop = curr_plan
            .operator()
            .physical_op()
            .derive_output_properties(child_output_props);
        let curr_group = curr_plan.group();
        let curr_cost = curr_plan.compute_cost(curr_group.borrow().statistics().as_deref());
        if !output_prop.satisfy(&self.required_prop) {
            let enforcer_plan = self.add_enforcer_to_group(&self.required_prop, memo);
            let enforcer_cost = curr_plan.compute_cost(curr_group.borrow().statistics().as_deref());
            curr_group
                .borrow_mut()
                .update_cost_plan(&self.required_prop, &enforcer_plan, enforcer_cost);
            return enforcer_cost;
        }
        curr_group
            .borrow_mut()
            .update_cost_plan(&self.required_prop, &self.plan, curr_cost);
        curr_cost
    }

    /**
     * 1. make require property for children base of current operator
     * 2. try to optimize child group and get best (Cost, GroupPlan) pair of every children
     * 3. once we get all output property of one candidate loop, derive output property base of current operator
     * 4. if output property does not satisfied require property, add enforcers and submit (Cost, GroupPlan) pair
     */
    pub(super) fn execute(mut self, task_runner: &mut TaskRunner, optimizer_ctx: &mut OptimizerContext) {
        // 1. according to current operator create new requestPropList for children
        let reqd_props_list = self.make_child_required_props_list();
        for (index, child_required_props) in reqd_props_list.iter().enumerate().skip(self.prev_index()) {
            let mut child_output_props = Vec::new();
            if !self.plan.borrow().inputs().is_empty() {
                for (required_prop, child_group) in child_required_props.iter().zip(self.plan.borrow().inputs()) {
                    // 2. optimize children groups using requestPropList
                    match child_group.borrow().lowest_cost_plans().get(required_prop) {
                        Some((_cost, plan)) => {
                            let output = plan.borrow().get_output_prop(required_prop).clone();
                            child_output_props.push(output);
                        }
                        None => {
                            // 3. get output property of child groups and add enforcer to cost and plan pair
                            task_runner.push_task(self.clone());
                            let task = OptimizeGroupTask::new(child_group.clone(), required_prop.clone());
                            task_runner.push_task(task);
                            return;
                        }
                    }
                }
            }
            // 4. now assume we have optimize child groups for child_required_props and get one best cost and plan pairs
            // and we want to compare require_prop and output_prop derived by child output props
            // if do not satisfy, add enforcer
            self.prev_index = index;
            let cost = self.submit_cost_plan(&child_output_props, optimizer_ctx.memo_mut());
            self.plan.borrow().group().borrow_mut().update_child_required_props(
                &self.required_prop,
                child_required_props,
                cost,
            );
        }
    }
}
