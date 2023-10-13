use crate::memo::{GroupPlanRef, Memo};
use crate::property::PhysicalProperties;
use crate::task::{OptimizeGroupTask, Task, TaskRunner};
use crate::{OptimizerContext, OptimizerType};
use std::rc::Rc;

pub struct EnforceAndCostTask<T: OptimizerType> {
    plan: GroupPlanRef<T>,
    required_prop: Rc<PhysicalProperties<T>>,
    prev_index: usize,
}

impl<T: OptimizerType> From<EnforceAndCostTask<T>> for Task<T> {
    #[inline]
    fn from(task: EnforceAndCostTask<T>) -> Self {
        Task::EnforceAndCost(task)
    }
}

impl<T: OptimizerType> Clone for EnforceAndCostTask<T> {
    fn clone(&self) -> EnforceAndCostTask<T> {
        Self {
            plan: self.plan.clone(),
            required_prop: self.required_prop.clone(),
            prev_index: self.prev_index + 1,
        }
    }
}

impl<T: OptimizerType> EnforceAndCostTask<T> {
    pub const fn new(new_plan: GroupPlanRef<T>, new_required_prop: Rc<PhysicalProperties<T>>) -> Self {
        EnforceAndCostTask {
            plan: new_plan,
            required_prop: new_required_prop,
            prev_index: 0,
        }
    }

    fn make_child_required_props_list(&self) -> Vec<Vec<Rc<PhysicalProperties<T>>>> {
        self.plan
            .borrow()
            .operator()
            .physical_op()
            .required_properties(self.required_prop.clone())
    }

    fn add_enforcer_to_group(&self, required_prop: &PhysicalProperties<T>, memo: &mut Memo<T>) -> GroupPlanRef<T> {
        let group = self.plan.borrow().group();
        let group_plan = required_prop.make_enforcer(group.clone());
        memo.insert_group_plan(group_plan, Some(group))
    }

    fn prev_index(&self) -> usize {
        self.prev_index
    }

    fn submit_cost_plan(&self, child_output_props: Vec<Rc<PhysicalProperties<T>>>, memo: &mut Memo<T>) {
        let output_prop = {
            let mut curr_plan = self.plan.borrow_mut();
            let curr_group = curr_plan.group();
            let mut curr_group = curr_group.borrow_mut();

            let output_prop = curr_plan.derive_output_properties(&child_output_props);
            let curr_cost = curr_plan.compute_cost(curr_group.statistics().as_deref());

            curr_plan.update_require_to_output_map(&output_prop, &output_prop);
            curr_group.update_cost_plan(&output_prop, &self.plan, curr_cost);
            curr_group.update_child_required_props(&output_prop, child_output_props, curr_cost);
            output_prop
        };

        if !output_prop.satisfy(&self.required_prop) {
            let enforcer_plan = self.add_enforcer_to_group(&self.required_prop, memo);
            enforcer_plan
                .borrow_mut()
                .update_require_to_output_map(&self.required_prop, &output_prop);

            let curr_group = self.plan.borrow().group();
            let mut curr_group = curr_group.borrow_mut();
            let enforcer_cost = enforcer_plan.borrow().compute_cost(curr_group.statistics().as_deref());

            curr_group.update_cost_plan(&self.required_prop, &enforcer_plan, enforcer_cost);
            curr_group.update_child_required_props(&self.required_prop, vec![output_prop], enforcer_cost);
        }
    }

    /**
     * 1. make require property for children base of current operator
     * 2. try to optimize child group and get best (Cost, GroupPlan) pair of every children
     * 3. once we get all output property of one candidate loop, derive output property base of current operator
     * 4. if output property does not satisfied require property, add enforcers and submit (Cost, GroupPlan) pair
     */
    pub(super) fn execute(mut self, task_runner: &mut TaskRunner<T>, optimizer_ctx: &mut OptimizerContext<T>) {
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

            self.submit_cost_plan(child_output_props, optimizer_ctx.memo_mut());
        }
    }
}
