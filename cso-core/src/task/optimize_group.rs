use crate::memo::GroupRef;
use crate::property::PhysicalProperties;
use crate::task::{EnforceAndCostTask, OptimizePlanTask, Task, TaskRunner};
use crate::{OptimizerContext, OptimizerType};
use std::rc::Rc;

pub struct OptimizeGroupTask<T: OptimizerType> {
    group: GroupRef<T>,
    required_prop: Rc<PhysicalProperties<T>>,
}

impl<T: OptimizerType> From<OptimizeGroupTask<T>> for Task<T> {
    #[inline]
    fn from(task: OptimizeGroupTask<T>) -> Self {
        Task::OptimizeGroup(task)
    }
}

impl<T: OptimizerType> OptimizeGroupTask<T> {
    pub const fn new(group: GroupRef<T>, required_prop: Rc<PhysicalProperties<T>>) -> Self {
        OptimizeGroupTask { group, required_prop }
    }

    pub(super) fn execute(self, task_runner: &mut TaskRunner<T>, _optimizer_ctx: &mut OptimizerContext<T>) {
        let mut group = self.group.borrow_mut();

        if !group.is_explored() {
            for plan in group.logical_plans().iter().rev() {
                let task = OptimizePlanTask::new(plan.clone());
                task_runner.push_task(task);
            }
            group.set_explored();
        }

        for plan in group.physical_plans().iter().rev() {
            let task = EnforceAndCostTask::new(plan.clone(), self.required_prop.clone());
            task_runner.push_task(task);
        }
    }
}
