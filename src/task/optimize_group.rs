use crate::memo::GroupRef;
use crate::property::PhysicalProperties;
use crate::task::{EnforceAndCostTask, OptimizePlanTask, Task, TaskRunner};
use crate::OptimizerContext;
use std::rc::Rc;

pub struct OptimizeGroupTask {
    group: GroupRef,
    required_prop: Rc<PhysicalProperties>,
    is_explored: bool,
}

impl OptimizeGroupTask {
    pub const fn new(group: GroupRef, required_prop: Rc<PhysicalProperties>) -> Self {
        OptimizeGroupTask {
            group,
            required_prop,
            is_explored: false,
        }
    }

    fn is_explored(&self) -> bool {
        self.is_explored
    }

    pub(super) fn execute(mut self, task_runner: &mut TaskRunner, _optimizer_ctx: &mut OptimizerContext) {
        let group = self.group.borrow();

        if !self.is_explored() {
            for plan in group.logical_plans().iter().rev() {
                let task = OptimizePlanTask::new(plan.clone());
                task_runner.push_task(Task::OptimizePlan(task));
            }
            self.is_explored = true;
        }

        for plan in group.physical_plans().iter().rev() {
            let task = EnforceAndCostTask::new(plan.clone(), self.required_prop.clone());
            task_runner.push_task(Task::EnforceAndCost(task));
        }
    }
}
