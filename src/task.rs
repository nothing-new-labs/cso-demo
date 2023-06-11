use crate::memo::{GroupPlanRef, GroupRef};
use crate::OptimizerContext;

pub enum Task {
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

pub struct TaskRunner {
    tasks: Vec<Task>,
}

impl TaskRunner {
    pub fn new() -> Self {
        TaskRunner { tasks: Vec::new() }
    }

    pub fn push_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn run(&mut self, optimizer_ctx: &mut OptimizerContext) {
        while let Some(task) = self.tasks.pop() {
            task.execute(self, optimizer_ctx);
        }
    }
}

pub struct OptimizeGroupTask {
    group: GroupRef,
}

impl OptimizeGroupTask {
    pub const fn new(group: GroupRef) -> Self {
        OptimizeGroupTask { group }
    }

    fn execute(self, task_runner: &mut TaskRunner, _optimizer_ctx: &mut OptimizerContext) {
        let group = self.group.borrow();

        for plan in group.logical_plans().iter().rev() {
            let task = OptimizePlanTask::new(plan.clone());
            task_runner.push_task(Task::OptimizePlan(task));
        }

        for plan in group.physical_plans().iter().rev() {
            let task = EnforceAndCostTask::new(plan.clone());
            task_runner.push_task(Task::EnforceAndCost(task));
        }
    }
}

pub struct OptimizePlanTask {
    plan: GroupPlanRef,
}

impl OptimizePlanTask {
    pub const fn new(plan: GroupPlanRef) -> Self {
        OptimizePlanTask { plan }
    }

    fn execute(self, task_runner: &mut TaskRunner, _optimizer_ctx: &mut OptimizerContext) {
        // todo: for each rules
        let apply_rule_task = ApplyRuleTask::new(self.plan.clone());
        task_runner.push_task(Task::ApplyRule(apply_rule_task));

        let derive_stats_task = DeriveStatsTask::new(self.plan.clone());
        task_runner.push_task(Task::DeriveStats(derive_stats_task));

        let group_plan = self.plan.borrow();

        for group in group_plan.inputs().iter().rev() {
            let task = ExploreGroupTask::new(group.clone());
            task_runner.push_task(Task::ExploreGroup(task));
        }
    }
}

pub struct EnforceAndCostTask {
    _plan: GroupPlanRef,
}

impl EnforceAndCostTask {
    pub const fn new(plan: GroupPlanRef) -> Self {
        EnforceAndCostTask { _plan: plan }
    }

    fn execute(self, _task_runner: &mut TaskRunner, _optimizer_ctx: &mut OptimizerContext) {
        todo!()
    }
}

pub struct ApplyRuleTask {
    _plan: GroupPlanRef,
}

impl ApplyRuleTask {
    pub const fn new(plan: GroupPlanRef) -> Self {
        ApplyRuleTask { _plan: plan }
    }

    fn execute(self, _task_runner: &mut TaskRunner, _optimizer_ctx: &mut OptimizerContext) {
        todo!()
    }
}

pub struct DeriveStatsTask {
    plan: GroupPlanRef,
}

impl DeriveStatsTask {
    pub const fn new(plan: GroupPlanRef) -> Self {
        DeriveStatsTask { plan }
    }

    fn execute(self, _task_runner: &mut TaskRunner, _optimizer_ctx: &mut OptimizerContext) {
        let mut plan = self.plan.borrow_mut();

        if plan.is_stats_derived() {
            return;
        }

        let stats = plan.derive_statistics();

        let group = plan.group();
        group.borrow_mut().update_statistics(stats);

        plan.set_stats_derived();
    }
}

pub struct ExploreGroupTask {
    group: GroupRef,
}

impl ExploreGroupTask {
    pub const fn new(group: GroupRef) -> Self {
        ExploreGroupTask { group }
    }

    fn execute(self, task_runner: &mut TaskRunner, _optimizer_ctx: &mut OptimizerContext) {
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
