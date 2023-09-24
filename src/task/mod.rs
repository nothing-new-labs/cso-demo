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

use crate::OptimizerContext;

pub(crate) enum Task {
    OptimizeGroup(OptimizeGroupTask),
    OptimizePlan(OptimizePlanTask),
    ApplyRule(ApplyRuleTask),
    EnforceAndCost(EnforceAndCostTask),
    DeriveStats(DeriveStatsTask),
    ExploreGroup(ExploreGroupTask),
}

impl Task {
    fn execute(self, task_runner: &mut TaskRunner, optimizer_ctx: &mut OptimizerContext) {
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

pub(crate) struct TaskRunner {
    tasks: Vec<Task>,
}

impl TaskRunner {
    pub fn new() -> Self {
        TaskRunner { tasks: Vec::new() }
    }

    #[inline]
    pub fn push_task<T: Into<Task>>(&mut self, task: T) {
        self.tasks.push(task.into());
    }

    #[inline]
    pub fn pop_task(&mut self) -> Option<Task> {
        let task = self.tasks.pop();
        task
    }

    pub fn run(&mut self, optimizer_ctx: &mut OptimizerContext) {
        while let Some(task) = self.pop_task() {
            task.execute(self, optimizer_ctx);
        }
    }
}
