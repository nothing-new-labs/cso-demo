use crate::memo::GroupRef;
use crate::task::{OptimizePlanTask, Task, TaskRunner};
use crate::{OptimizerContext, OptimizerType};

pub struct ExploreGroupTask {
    group: GroupRef,
}

impl<T: OptimizerType> From<ExploreGroupTask> for Task<T> {
    #[inline]
    fn from(task: ExploreGroupTask) -> Self {
        Task::ExploreGroup(task)
    }
}

impl ExploreGroupTask {
    pub const fn new(group: GroupRef) -> Self {
        ExploreGroupTask { group }
    }

    pub(super) fn execute<T: OptimizerType>(
        self,
        task_runner: &mut TaskRunner<T>,
        _optimizer_ctx: &mut OptimizerContext<T>,
    ) {
        let mut group = self.group.borrow_mut();
        if group.is_explored() {
            return;
        }

        for plan in group.logical_plans() {
            let task = OptimizePlanTask::new(plan.clone());
            task_runner.push_task(task);
        }

        group.set_explored();
    }
}
