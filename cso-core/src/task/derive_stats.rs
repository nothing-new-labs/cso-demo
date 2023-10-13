use crate::memo::GroupPlanRef;
use crate::task::{Task, TaskRunner};
use crate::{OptimizerContext, OptimizerType};

pub struct DeriveStatsTask<T: OptimizerType> {
    plan: GroupPlanRef<T>,
}

impl<T: OptimizerType> From<DeriveStatsTask<T>> for Task<T> {
    #[inline]
    fn from(task: DeriveStatsTask<T>) -> Self {
        Task::DeriveStats(task)
    }
}

impl<T: OptimizerType> DeriveStatsTask<T> {
    pub const fn new(plan: GroupPlanRef<T>) -> Self {
        DeriveStatsTask { plan }
    }

    pub(super) fn execute(self, _task_runner: &mut TaskRunner<T>, optimizer_ctx: &mut OptimizerContext<T>) {
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
