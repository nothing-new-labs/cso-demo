use crate::memo::GroupRef;
use crate::task::{OptimizePlanTask, Task, TaskRunner};
use crate::OptimizerContext;

pub struct ExploreGroupTask {
    group: GroupRef,
}

impl ExploreGroupTask {
    pub const fn new(group: GroupRef) -> Self {
        ExploreGroupTask { group }
    }

    pub(super) fn execute(self, task_runner: &mut TaskRunner, _optimizer_ctx: &mut OptimizerContext) {
        let mut group = self.group.borrow_mut();
        if group.is_explored() {
            return;
        }

        for plan in group.logical_plans() {
            let task = OptimizePlanTask::new(plan.clone());
            task_runner.push_task(Task::OptimizePlan(task));
        }

        group.set_explored();
    }
}
