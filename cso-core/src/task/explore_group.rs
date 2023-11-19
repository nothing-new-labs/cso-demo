use crate::memo::GroupRef;
use crate::property::PhysicalProperties;
use crate::task::{OptimizePlanTask, Task, TaskRunner};
use crate::{OptimizerContext, OptimizerType};
use std::rc::Rc;

pub struct ExploreGroupTask<T: OptimizerType> {
    group: GroupRef<T>,
    required_prop: Rc<PhysicalProperties<T>>,
}

impl<T: OptimizerType> From<ExploreGroupTask<T>> for Task<T> {
    #[inline]
    fn from(task: ExploreGroupTask<T>) -> Self {
        Task::ExploreGroup(task)
    }
}

impl<T: OptimizerType> ExploreGroupTask<T> {
    pub const fn new(group: GroupRef<T>, required_prop: Rc<PhysicalProperties<T>>) -> Self {
        ExploreGroupTask { group, required_prop }
    }

    pub(super) fn execute(self, task_runner: &mut TaskRunner<T>, _optimizer_ctx: &mut OptimizerContext<T>) {
        let mut group = self.group.borrow_mut();
        if group.is_explored() {
            return;
        }

        for plan in group.logical_plans() {
            let task = OptimizePlanTask::new(plan.clone(), self.required_prop.clone());
            task_runner.push_task(task);
        }

        group.set_explored();
    }
}
