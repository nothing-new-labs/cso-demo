use crate::memo::GroupPlanRef;
use crate::rule::{Binding, RuleRef};
use crate::task::{EnforceAndCostTask, OptimizePlanTask, Task, TaskRunner};
use crate::{OptimizerContext, OptimizerType};

pub struct ApplyRuleTask<T> {
    plan: GroupPlanRef,
    rule: RuleRef<T>,
}

impl<T: OptimizerType> From<ApplyRuleTask<T>> for Task<T> {
    #[inline]
    fn from(task: ApplyRuleTask<T>) -> Self {
        Task::ApplyRule(task)
    }
}

impl<T: OptimizerType> ApplyRuleTask<T> {
    pub const fn new(plan: GroupPlanRef, rule: RuleRef<T>) -> Self {
        ApplyRuleTask { plan, rule }
    }

    pub(super) fn execute(self, task_runner: &mut TaskRunner<T>, optimizer_ctx: &mut OptimizerContext<T>) {
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
