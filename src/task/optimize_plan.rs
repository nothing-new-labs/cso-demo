use crate::memo::{GroupPlan, GroupPlanRef};
use crate::rule::{RuleRef, RuleSet};
use crate::task::{ApplyRuleTask, DeriveStatsTask, ExploreGroupTask, Task, TaskRunner};
use crate::OptimizerContext;
use std::ops::Deref;

pub struct OptimizePlanTask {
    plan: GroupPlanRef,
}

impl From<OptimizePlanTask> for Task {
    #[inline]
    fn from(task: OptimizePlanTask) -> Self {
        Task::OptimizePlan(task)
    }
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

    pub(super) fn execute(self, task_runner: &mut TaskRunner, optimizer_ctx: &mut OptimizerContext) {
        let rules = self.get_rules(optimizer_ctx.rule_set());
        for rule in rules {
            let apply_rule_task = ApplyRuleTask::new(self.plan.clone(), rule);
            task_runner.push_task(apply_rule_task);
        }

        let derive_stats_task = DeriveStatsTask::new(self.plan.clone());
        task_runner.push_task(derive_stats_task);

        let group_plan = self.plan.borrow();

        for group in group_plan.inputs().iter().rev() {
            let task = ExploreGroupTask::new(group.clone());
            task_runner.push_task(task);
        }
    }
}
