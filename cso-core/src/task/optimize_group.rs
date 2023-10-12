use crate::memo::GroupRef;
use crate::property::PhysicalProperties;
use crate::task::{EnforceAndCostTask, OptimizePlanTask, Task, TaskRunner};
use crate::{OptimizerContext, OptimizerType};
use std::rc::Rc;

pub struct OptimizeGroupTask {
    group: GroupRef,
    required_prop: Rc<PhysicalProperties>,
}

impl<T: OptimizerType> From<OptimizeGroupTask> for Task<T> {
    #[inline]
    fn from(task: OptimizeGroupTask) -> Self {
        Task::OptimizeGroup(task)
    }
}

impl OptimizeGroupTask {
    pub const fn new(group: GroupRef, required_prop: Rc<PhysicalProperties>) -> Self {
        OptimizeGroupTask { group, required_prop }
    }

    pub(super) fn execute<T: OptimizerType>(
        self,
        task_runner: &mut TaskRunner<T>,
        _optimizer_ctx: &mut OptimizerContext<T>,
    ) {
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
