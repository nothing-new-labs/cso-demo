use crate::memo::{GroupId, GroupPlanId};
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
    group_id: GroupId,
}

impl OptimizeGroupTask {
    pub const fn new(group_id: GroupId) -> Self {
        OptimizeGroupTask { group_id }
    }

    fn execute(self, task_runner: &mut TaskRunner, optimizer_ctx: &mut OptimizerContext) {
        let group = &optimizer_ctx.memo_mut()[self.group_id];

        for plan in group.logical_plans().iter().rev() {
            let task = OptimizePlanTask::new(self.group_id, plan.plan_id());
            task_runner.push_task(Task::OptimizePlan(task));
        }

        for plan in group.physical_plans().iter().rev() {
            let task = EnforceAndCostTask::new(self.group_id, plan.plan_id());
            task_runner.push_task(Task::EnforceAndCost(task));
        }
    }
}

pub struct OptimizePlanTask {
    group_id: GroupId,
    plan_id: GroupPlanId,
}

impl OptimizePlanTask {
    pub const fn new(group_id: GroupId, plan_id: GroupPlanId) -> Self {
        OptimizePlanTask { group_id, plan_id }
    }

    fn execute(self, task_runner: &mut TaskRunner, optimizer_ctx: &mut OptimizerContext) {
        // todo: for each rules
        let apply_rule_task = ApplyRuleTask::new(self.group_id, self.plan_id);
        task_runner.push_task(Task::ApplyRule(apply_rule_task));

        let derive_stats_task = DeriveStatsTask::new(self.group_id, self.plan_id);
        task_runner.push_task(Task::DeriveStats(derive_stats_task));

        let group_plan = &optimizer_ctx.memo_mut()[self.group_id][self.plan_id];

        for group_id in group_plan.inputs().iter().rev() {
            let task = ExploreGroupTask::new(*group_id);
            task_runner.push_task(Task::ExploreGroup(task));
        }
    }
}

pub struct EnforceAndCostTask {
    _group_id: GroupId,
    _plan_id: GroupPlanId,
}

impl EnforceAndCostTask {
    pub const fn new(group_id: GroupId, plan_id: GroupPlanId) -> Self {
        EnforceAndCostTask {
            _group_id: group_id,
            _plan_id: plan_id,
        }
    }

    fn execute(self, _task_runner: &mut TaskRunner, _optimizer_ctx: &mut OptimizerContext) {
        todo!()
    }
}

pub struct ApplyRuleTask {
    _group_id: GroupId,
    _plan_id: GroupPlanId,
}

impl ApplyRuleTask {
    pub const fn new(group_id: GroupId, plan_id: GroupPlanId) -> Self {
        ApplyRuleTask {
            _group_id: group_id,
            _plan_id: plan_id,
        }
    }

    fn execute(self, _task_runner: &mut TaskRunner, _optimizer_ctx: &mut OptimizerContext) {
        todo!()
    }
}

pub struct DeriveStatsTask {
    _group_id: GroupId,
    _plan_id: GroupPlanId,
}

impl DeriveStatsTask {
    pub const fn new(group_id: GroupId, plan_id: GroupPlanId) -> Self {
        DeriveStatsTask {
            _group_id: group_id,
            _plan_id: plan_id,
        }
    }

    fn execute(self, _task_runner: &mut TaskRunner, _optimizer_ctx: &mut OptimizerContext) {
        todo!()
    }
}

pub struct ExploreGroupTask {
    group_id: GroupId,
}

impl ExploreGroupTask {
    pub const fn new(group_id: GroupId) -> Self {
        ExploreGroupTask { group_id }
    }

    fn execute(self, task_runner: &mut TaskRunner, optimizer_ctx: &mut OptimizerContext) {
        let group = &mut optimizer_ctx.memo_mut()[self.group_id];
        if group.is_explored() {
            return;
        }

        for plan in group.logical_plans() {
            debug_assert_eq!(self.group_id, plan.group_id());
            let task = OptimizePlanTask::new(plan.group_id(), plan.plan_id());
            task_runner.push_task(Task::OptimizePlan(task));
        }

        group.set_explored();
    }
}
