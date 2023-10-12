use crate::memo::GroupPlanRef;
use crate::rule::{Binding, RuleRef};
use crate::task::{EnforceAndCostTask, OptimizePlanTask, Task, TaskRunner};
use crate::OptimizerContext;

pub struct ApplyRuleTask {
    plan: GroupPlanRef,
    rule: RuleRef,
}

impl From<ApplyRuleTask> for Task {
    #[inline]
    fn from(task: ApplyRuleTask) -> Self {
        Task::ApplyRule(task)
    }
}

impl ApplyRuleTask {
    pub const fn new(plan: GroupPlanRef, rule: RuleRef) -> Self {
        ApplyRuleTask { plan, rule }
    }

    pub(super) fn execute(self, task_runner: &mut TaskRunner, optimizer_ctx: &mut OptimizerContext) {
        assert!(!self.plan.borrow().is_rule_explored(self.rule.as_ref()));

        let rule = self.rule.as_ref();
        let pattern = self.rule.pattern();
        let binding = Binding::new(pattern, &self.plan);

        let mut new_plans = Vec::new();

        for plan in binding {
            if !rule.check(&plan, optimizer_ctx) {
                continue;
            }

            let mut target_plans = rule.transform(&plan, optimizer_ctx);
            new_plans.append(&mut target_plans);
        }

        let curr_group = self.plan.borrow().group();
        for plan in new_plans {
            let group_plan = optimizer_ctx.memo_mut().copy_in_plan(Some(curr_group.clone()), &plan);
            if group_plan.borrow().operator().is_logical() {
                task_runner.push_task(OptimizePlanTask::new(group_plan));
            } else {
                // todo: get `required_prop` from curr_group
                let required_prop = optimizer_ctx.required_properties();
                let new_task = EnforceAndCostTask::new(group_plan, required_prop.clone());
                task_runner.push_task(new_task);
            }
        }
    }
}
