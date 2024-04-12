use crate::memo::{GroupPlanRef, GroupRef, LowestCostPlan};
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
     * 4. if output property does not satisfy require property, add enforcers and submit (Cost, GroupPlan) pair
     */
    pub(super) fn execute(mut self, task_runner: &mut TaskRunner<T>, optimizer_ctx: &mut OptimizerContext<T>) {
        // get required properties for children
        let child_reqd_props_list = self.init_child_required_props_list();

        for (index, child_reqd_props) in child_reqd_props_list.iter().skip(self.prev_index).enumerate() {
            let mut cost = self.plan.borrow().compute_cost();
            let mut child_output_props = Vec::with_capacity(child_reqd_props.len());

            for (child_index, child_reqd_prop) in child_reqd_props.iter().enumerate() {
                let curr_child_ref = self.child(child_index);
                let curr_child = curr_child_ref.borrow();

                // check whether the current child group is already optimized for the current child_reqd_prop,
                // if we have optimized current child group, we can get the best (Cost, GroupPlan).
                // otherwise, we need to optimize current child group first.
                match curr_child.lowest_cost_plans().get(child_reqd_prop) {
                    Some((child_cost, plan)) => {
                        let output_prop = plan.borrow().get_output_prop(child_reqd_prop).clone();
                        child_output_props.push(output_prop);
                        cost += *child_cost;
                    }
                    None => {
                        self.prev_index = index;
                        task_runner.push_task(self.clone());
                        let task = OptimizeGroupTask::new(curr_child_ref.clone(), child_reqd_prop.clone());
                        task_runner.push_task(task);
                        return;
                    }
                }
            }

            // successfully optimize all child group, and we can compute the output property for current operator.
            let output_prop = self.derive_output_props(&child_output_props);
            self.submit_best_plan(&output_prop, (cost, self.plan.clone()), child_reqd_props.clone());

            // enforce property if output_prop doesn't satisfy self.required_prop
            let enforcer = self.add_enforcer(&output_prop, &self.required_prop, optimizer_ctx);
            match enforcer {
                Some(enforcer) => {
                    cost += enforcer.borrow().compute_cost();
                    self.submit_best_plan(&self.required_prop, (cost, enforcer), vec![output_prop])
                }
                None => self.submit_best_plan(&self.required_prop, (cost, self.plan.clone()), child_reqd_props.clone()),
            }
        }
    }

    fn derive_output_props(&self, child_output_props: &[Rc<PhysicalProperties<T>>]) -> Rc<PhysicalProperties<T>> {
        let curr_plan = self.plan.borrow();
        curr_plan.derive_output_properties(&child_output_props)
    }

    fn submit_best_plan(
        &self,
        required_prop: &Rc<PhysicalProperties<T>>,
        lowest_cost_plan: LowestCostPlan<T>,
        child_reqd_props: Vec<Rc<PhysicalProperties<T>>>,
    ) {
        let (cost, best_plan) = lowest_cost_plan;

        // The passed-in best_plan might be self.plan, in which case there would be an issue with duplicate borrows.
        // To address this problem, we explicitly add `{}` to control the lifetime of the borrow.
        {
            let curr_plan = self.plan.borrow();
            let curr_group = curr_plan.group();
            let mut curr_group = curr_group.borrow_mut();

            curr_group.update_cost_plan(required_prop, (cost, best_plan.clone()));
            curr_group.update_child_required_props(required_prop, cost, child_reqd_props);
        }

        {
            let mut best_plan = best_plan.borrow_mut();
            best_plan.update_require_to_output_map(required_prop, required_prop);
        }
    }

    fn add_enforcer(
        &self,
        output_prop: &Rc<PhysicalProperties<T>>,
        required_prop: &Rc<PhysicalProperties<T>>,
        optimizer_ctx: &mut OptimizerContext<T>,
    ) -> Option<GroupPlanRef<T>> {
        let curr_plan = self.plan.borrow();
        let curr_group = curr_plan.group();

        if !output_prop.satisfy(required_prop) {
            let enforcer = required_prop.make_enforcer(curr_group.clone());
            let enforcer = optimizer_ctx.memo.insert_group_plan(enforcer, Some(curr_group));
            Some(enforcer)
        } else {
            None
        }
    }
}
