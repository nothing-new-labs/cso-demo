use crate::memo::GroupPlanRef;
use crate::property::PhysicalProperties;
use crate::rule::{Binding, RuleRef};
use crate::task::{EnforceAndCostTask, OptimizePlanTask, Task, TaskRunner};
use crate::OptimizerContext;
use std::rc::Rc;

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
        let plan = self.plan.borrow();
        let group = plan.group();
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

        for plan in new_plans {
            let group_plan = optimizer_ctx.memo_mut().copy_in_plan(Some(group.clone()), &plan);
            if group_plan.borrow().operator().is_logical() {
                task_runner.push_task(OptimizePlanTask::new(group_plan));
            } else {
                let required_prop: PhysicalProperties = PhysicalProperties::new();
                let new_task = EnforceAndCostTask::new(group_plan, Rc::new(required_prop.clone()));
                task_runner.push_task(new_task);
            }
        }
    }
}
