use crate::cost::Cost;
use crate::memo::{GroupPlanRef, GroupRef};
use crate::property::PhysicalProperties;
use crate::task::{Task, TaskRunner};
use crate::{OptimizeGroupTask, OptimizerContext, OptimizerType};
use std::rc::Rc;

pub struct EnforceAndCostTask<T: OptimizerType> {
    plan: GroupPlanRef<T>,
    required_prop: Rc<PhysicalProperties<T>>,

    child_reqd_props_list: Vec<Vec<Rc<PhysicalProperties<T>>>>,
    cur_prop_pair_index: usize,
    child_output_prors: Vec<Rc<PhysicalProperties<T>>>,
    total_cost: Cost,
    local_cost: Cost,
    cur_child_index: isize,
    prev_child_index: isize,
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
            cur_prop_pair_index: 0,

            child_reqd_props_list: vec![],
            child_output_prors: vec![],
            total_cost: Cost::new(0.0),
            local_cost: Cost::new(0.0),
            cur_child_index: 0,
            prev_child_index: -1,
        }
    }

    fn init_child_required_props_list(&mut self) {
        if self.cur_child_index != -1 {
            self.child_reqd_props_list = self
                .plan
                .borrow()
                .operator()
                .physical_op()
                .required_properties(self.required_prop.clone());
            self.cur_child_index = 0;
        }
    }

    fn cur_prop_pair_index(&self) -> usize {
        self.cur_prop_pair_index
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
        self.init_child_required_props_list();

        let pair_count = self.child_reqd_props_list.len();
        let cur_prop_pair_index = self.cur_prop_pair_index();
        for pair_index in cur_prop_pair_index..pair_count {
            self.cur_prop_pair_index = pair_index;
            let child_reqd_prors = &self.child_reqd_props_list[pair_index];

            if self.cur_child_index == 0 && self.prev_child_index == -1 {
                self.local_cost = self.plan.borrow().compute_cost();
                self.total_cost += self.local_cost;
            }

            let child_count = child_reqd_prors.len();
            let cur_child_index = self.cur_child_index as usize;
            for child_index in cur_child_index..child_count {
                self.cur_child_index = child_index as isize;
                let curr_child_ref = self.child(child_index);
                let curr_child = curr_child_ref.borrow();
                let child_reqd_prop = &child_reqd_prors[child_index];

                // 2. optimize children groups using requestPropList
                match curr_child.lowest_cost_plans().get(child_reqd_prop) {
                    Some((cost, plan)) => {
                        let output_prop = plan.borrow().get_output_prop(child_reqd_prop).clone();
                        self.child_output_prors.push(output_prop);
                        self.total_cost += *cost;
                    }
                    None => {
                        self.prev_child_index = self.cur_child_index;

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
            let output_prop = curr_plan.derive_output_properties(&self.child_output_prors);
            curr_plan.update_require_to_output_map(&output_prop, &output_prop);

            {
                let curr_group_ref = curr_plan.group();
                let mut curr_group = curr_group_ref.borrow_mut();
                curr_group.update_cost_plan(&output_prop, &self.plan, self.total_cost);
                curr_group.update_child_required_props(&output_prop, child_reqd_prors.clone(), self.total_cost);
            }

            if !output_prop.satisfy(&self.required_prop) {
                let curr_group = curr_plan.group();
                let enforer = self.required_prop.make_enforcer(curr_group.clone());
                let enforer = optimizer_ctx.memo.insert_group_plan(enforer, Some(curr_group.clone()));
                enforer
                    .borrow_mut()
                    .update_require_to_output_map(&self.required_prop, &output_prop);

                let enforer_cost = enforer.borrow().compute_cost();
                self.total_cost += enforer_cost;

                let mut curr_group = curr_group.borrow_mut();
                curr_group.update_cost_plan(&self.required_prop, &enforer, self.total_cost);
                curr_group.update_child_required_props(&self.required_prop, vec![output_prop], self.total_cost);
            }

            self.cur_prop_pair_index = 0;
            self.cur_child_index = 0;
            self.prev_child_index = -1;
            self.total_cost = Cost::new(0.0);
            self.child_output_prors.clear();
        }
    }
}

impl<T: OptimizerType> Clone for EnforceAndCostTask<T> {
    fn clone(&self) -> Self {
        Self {
            plan: self.plan.clone(),
            required_prop: self.required_prop.clone(),

            child_reqd_props_list: self.child_reqd_props_list.clone(),
            cur_prop_pair_index: self.cur_prop_pair_index + 1,
            child_output_prors: self.child_output_prors.clone(),
            total_cost: self.total_cost,
            local_cost: self.local_cost,
            cur_child_index: self.cur_child_index,
            prev_child_index: self.prev_child_index,
        }
    }
}
