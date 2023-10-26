use crate::memo::{GroupPlan, GroupPlanRef};
use crate::property::PhysicalProperties;
use crate::rule::{RuleRef, RuleSet};
use crate::task::{ApplyRuleTask, DeriveStatsTask, ExploreGroupTask, Task, TaskRunner};
use crate::{OptimizerContext, OptimizerType};
use std::ops::Deref;
use std::rc::Rc;

pub struct OptimizePlanTask<T: OptimizerType> {
    plan: GroupPlanRef<T>,
    required_prop: Rc<PhysicalProperties<T>>,
}

impl<T: OptimizerType> From<OptimizePlanTask<T>> for Task<T> {
    #[inline]
    fn from(task: OptimizePlanTask<T>) -> Self {
        Task::OptimizePlan(task)
    }
}

impl<T: OptimizerType> OptimizePlanTask<T> {
    pub const fn new(plan: GroupPlanRef<T>, required_prop: Rc<PhysicalProperties<T>>) -> Self {
        OptimizePlanTask { plan, required_prop }
    }

    fn filter_invalid_rules(plan: &GroupPlan<T>, candidate_rules: &[RuleRef<T>], valid_rules: &mut Vec<RuleRef<T>>) {
        candidate_rules
            .iter()
            .filter(|rule| !plan.is_rule_explored(rule.as_ref()) && rule.pattern().match_without_child(plan))
            .for_each(|rule| valid_rules.push(rule.clone()));
    }

    fn get_rules(&self, rule_set: &RuleSet<T>) -> Vec<RuleRef<T>> {
        let mut rules = Vec::new();
        let plan = self.plan.borrow();

        let transform_rules = rule_set.transform_rules();
        Self::filter_invalid_rules(plan.deref(), transform_rules, &mut rules);

        let implement_rules = rule_set.implement_rules();
        Self::filter_invalid_rules(plan.deref(), implement_rules, &mut rules);

        rules
    }

    pub(super) fn execute(self, task_runner: &mut TaskRunner<T>, optimizer_ctx: &mut OptimizerContext<T>) {
        let rules = self.get_rules(optimizer_ctx.rule_set());
        for rule in rules {
            let apply_rule_task = ApplyRuleTask::new(self.plan.clone(), rule, self.required_prop.clone());
            task_runner.push_task(apply_rule_task);
        }

        let derive_stats_task = DeriveStatsTask::new(self.plan.clone());
        task_runner.push_task(derive_stats_task);

        let group_plan = self.plan.borrow();

        for group in group_plan.inputs().iter().rev() {
            let task = ExploreGroupTask::new(group.clone(), self.required_prop.clone());
            task_runner.push_task(task);
        }
    }
}
