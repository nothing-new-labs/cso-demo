mod apply_rule;
mod derive_stats;
mod enforce_and_cost;
mod explore_group;
mod optimize_group;
mod optimize_plan;

pub use apply_rule::ApplyRuleTask;
pub use derive_stats::DeriveStatsTask;
pub use enforce_and_cost::EnforceAndCostTask;
pub use explore_group::ExploreGroupTask;
pub use optimize_group::OptimizeGroupTask;
pub use optimize_plan::OptimizePlanTask;

use crate::{OptimizerContext, OptimizerType};

pub(crate) enum Task<T: OptimizerType> {
    OptimizeGroup(OptimizeGroupTask),
    OptimizePlan(OptimizePlanTask),
    ApplyRule(ApplyRuleTask<T>),
    EnforceAndCost(EnforceAndCostTask),
    DeriveStats(DeriveStatsTask),
    ExploreGroup(ExploreGroupTask),
}

impl<T: OptimizerType> Task<T> {
    fn execute(self, task_runner: &mut TaskRunner<T>, optimizer_ctx: &mut OptimizerContext<T>) {
        match self {
            Task::OptimizeGroup(task) => {
                task.execute(task_runner, optimizer_ctx);
            }
            Task::OptimizePlan(task) => {
                task.execute(task_runner, optimizer_ctx);
            }
            Task::ApplyRule(task) => {
                task.execute(task_runner, optimizer_ctx);
            }
            Task::EnforceAndCost(task) => {
                task.execute(task_runner, optimizer_ctx);
            }
            Task::DeriveStats(task) => {
                task.execute(task_runner, optimizer_ctx);
            }
            Task::ExploreGroup(task) => {
                task.execute(task_runner, optimizer_ctx);
            }
        }
    }
}

pub(crate) struct TaskRunner<T: OptimizerType> {
    tasks: Vec<Task<T>>,
}

impl<OT: OptimizerType> TaskRunner<OT> {
    pub fn new() -> Self {
        TaskRunner { tasks: Vec::new() }
    }

    #[inline]
    pub fn push_task<T: Into<Task<OT>>>(&mut self, task: T) {
        self.tasks.push(task.into());
    }

    pub fn run(&mut self, optimizer_ctx: &mut OptimizerContext<OT>) {
        while let Some(task) = self.tasks.pop() {
            task.execute(self, optimizer_ctx);
        }
    }
}
