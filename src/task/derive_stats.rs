use crate::memo::GroupPlanRef;
use crate::task::{Task, TaskRunner};
use crate::OptimizerContext;

pub struct DeriveStatsTask {
    plan: GroupPlanRef,
}

impl From<DeriveStatsTask> for Task {
    #[inline]
    fn from(task: DeriveStatsTask) -> Self {
        Task::DeriveStats(task)
    }
}

impl DeriveStatsTask {
    pub const fn new(plan: GroupPlanRef) -> Self {
        DeriveStatsTask { plan }
    }

    pub(super) fn execute(self, _task_runner: &mut TaskRunner, optimizer_ctx: &mut OptimizerContext) {
        let mut plan = self.plan.borrow_mut();

        if plan.is_stats_derived() {
            return;
        }

        let stats = plan.derive_statistics(optimizer_ctx);

        let group = plan.group();
        group.borrow_mut().update_statistics(stats);

        plan.set_stats_derived();
    }
}
