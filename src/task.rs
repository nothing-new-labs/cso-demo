use crate::memo::{GroupPlan, GroupPlanRef, GroupRef};
use crate::rule::{Binding, RuleRef, RuleSet};
use crate::OptimizerContext;
use std::ops::Deref;

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

    fn filter_invalid_rules(plan: &GroupPlan, candidate_rules: &[RuleRef], valid_rules: &mut Vec<RuleRef>) {
        candidate_rules
            .iter()
            .filter(|rule| plan.is_rule_explored(rule.as_ref()) || !rule.pattern().match_without_child(plan))
            .for_each(|rule| valid_rules.push(rule.clone()));
    }

    fn get_rules(&self, rule_set: &RuleSet) -> Vec<RuleRef> {
        let mut rules = Vec::new();
        let plan = self.plan.borrow();

        let transform_rules = rule_set.transform_rules();
        Self::filter_invalid_rules(plan.deref(), transform_rules, &mut rules);

        let implement_rules = rule_set.implement_rules();
        Self::filter_invalid_rules(plan.deref(), implement_rules, &mut rules);

        rules
    }

    fn execute(self, task_runner: &mut TaskRunner, optimizer_ctx: &mut OptimizerContext) {
        let rules = self.get_rules(optimizer_ctx.rule_set());
        for rule in rules {
            let apply_rule_task = ApplyRuleTask::new(self.plan.clone(), rule);
            task_runner.push_task(Task::ApplyRule(apply_rule_task));
        }

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
    plan: GroupPlanRef,
    rule: RuleRef,
}

impl ApplyRuleTask {
    pub const fn new(plan: GroupPlanRef, rule: RuleRef) -> Self {
        ApplyRuleTask { plan, rule }
    }

    fn execute(self, task_runner: &mut TaskRunner, optimizer_ctx: &mut OptimizerContext) {
        assert!(!self.plan.borrow().is_rule_explored(self.rule.as_ref()));

        let rule = self.rule.as_ref();
        let plan = self.plan.borrow();
        let group = plan.group();
        let pattern = self.rule.pattern();
        let binding = Binding::new(pattern, plan.deref());
        let mut new_plans = Vec::new();

        for plan in binding {
            if !rule.check(&plan, optimizer_ctx) {
                continue;
            }

            let mut target_plans = rule.transform(&plan, optimizer_ctx);
            new_plans.append(&mut target_plans);
        }

        for plan in new_plans {
            let group_plan = optimizer_ctx.memo_mut().copy_in_plan(Some(group.clone()), &plan);
            if group_plan.borrow().operator().is_logical() {
                task_runner.push_task(Task::OptimizePlan(OptimizePlanTask::new(group_plan)));
            } else {
                task_runner.push_task(Task::EnforceAndCost(EnforceAndCostTask::new(group_plan)));
            }
        }
    }
}

pub struct DeriveStatsTask {
    plan: GroupPlanRef,
}

impl DeriveStatsTask {
    pub const fn new(plan: GroupPlanRef) -> Self {
        DeriveStatsTask { plan }
    }

    fn execute(self, _task_runner: &mut TaskRunner, optimizer_ctx: &mut OptimizerContext) {
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
