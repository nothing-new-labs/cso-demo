use crate::memo::{GroupPlanRef, GroupRef};
use crate::property::PhysicalProperties;
use crate::task::{Task, TaskRunner};
use crate::{OptimizeGroupTask, OptimizerContext, OptimizerType};
use std::rc::Rc;

#[derive(Clone)]
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

impl<T: OptimizerType> EnforceAndCostTask<T> {
    pub const fn new(new_plan: GroupPlanRef<T>, new_required_prop: Rc<PhysicalProperties<T>>) -> Self {
        EnforceAndCostTask {
            plan: new_plan,
            required_prop: new_required_prop,
            prev_index: 0,
        }
    }

    fn init_child_required_props_list(&mut self) -> Vec<Vec<Rc<PhysicalProperties<T>>>> {
        self.plan
            .borrow()
            .operator()
            .physical_op()
            .required_properties(self.required_prop.clone())
    }

    #[inline]
    fn child(&self, index: usize) -> GroupRef<T> {
        self.plan.borrow().inputs()[index].clone()
    }

    /**
     * 1. make require property for children base of current operator
     * 2. try to optimize child group and get best (Cost, GroupPlan) pair of every children
     * 3. once we get all output property of one candidate loop, derive output property base of current operator
     * 4. if output property does not satisfied require property, add enforcers and submit (Cost, GroupPlan) pair
     */
    pub(super) fn execute(mut self, task_runner: &mut TaskRunner<T>, optimizer_ctx: &mut OptimizerContext<T>) {
        // 1. according to current operator create new requestPropList for children
        let child_reqd_props_list = self.init_child_required_props_list();

        for (index, child_reqd_prors) in child_reqd_props_list.iter().skip(self.prev_index).enumerate() {
            let mut total_cost = self.plan.borrow().compute_cost();

            let mut child_output_props = vec![];
            for child_index in 0..child_reqd_prors.len() {
                let curr_child_ref = self.child(child_index);
                let curr_child = curr_child_ref.borrow();
                let child_reqd_prop = &child_reqd_prors[child_index];

                // 2. optimize children groups using requestPropList
                match curr_child.lowest_cost_plans().get(child_reqd_prop) {
                    Some((cost, plan)) => {
                        let output_prop = plan.borrow().get_output_prop(child_reqd_prop).clone();
                        child_output_props.push(output_prop);
                        total_cost += *cost;
                    }
                    None => {
                        self.prev_index = index;
                        // 3. get output property of child groups and add enforcer to cost and plan pair
                        task_runner.push_task(self.clone());
                        let task = OptimizeGroupTask::new(curr_child_ref.clone(), child_reqd_prop.clone());
                        task_runner.push_task(task);
                        return;
                    }
                }
            }

            // 4. now assume we have optimize child groups for child_required_props and get one best cost and plan pairs
            // and we want to compare require_prop and output_prop derived by child output props
            // if do not satisfy, add enforcer
            let mut curr_plan = self.plan.borrow_mut();
            let output_prop = curr_plan.derive_output_properties(&child_output_props);
            curr_plan.update_require_to_output_map(&output_prop, &output_prop);

            {
                let curr_group_ref = curr_plan.group();
                let mut curr_group = curr_group_ref.borrow_mut();
                curr_group.update_cost_plan(&output_prop, &self.plan, total_cost);
                curr_group.update_child_required_props(&output_prop, child_reqd_prors.clone(), total_cost);
            }

            if !output_prop.satisfy(&self.required_prop) {
                let curr_group = curr_plan.group();
                let enforer = self.required_prop.make_enforcer(curr_group.clone());
                let enforer = optimizer_ctx.memo.insert_group_plan(enforer, Some(curr_group.clone()));
                enforer
                    .borrow_mut()
                    .update_require_to_output_map(&self.required_prop, &self.required_prop);
                let enforer_cost = enforer.borrow().compute_cost();
                total_cost += enforer_cost;

                let mut curr_group = curr_group.borrow_mut();
                curr_group.update_cost_plan(&self.required_prop, &enforer, total_cost);
                curr_group.update_child_required_props(&self.required_prop, vec![output_prop], total_cost);
            } else {
                curr_plan.update_require_to_output_map(&self.required_prop, &self.required_prop);

                let curr_group_ref = curr_plan.group();
                let mut curr_group = curr_group_ref.borrow_mut();
                curr_group.update_cost_plan(&self.required_prop, &self.plan, total_cost);
                curr_group.update_child_required_props(&self.required_prop, child_reqd_prors.clone(), total_cost);
            }
        }
    }
}
