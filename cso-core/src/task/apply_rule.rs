use crate::memo::GroupPlanRef;
use crate::property::PhysicalProperties;
use crate::rule::{Binding, RuleRef};
use crate::task::{EnforceAndCostTask, OptimizePlanTask, Task, TaskRunner};
use crate::{OptimizerContext, OptimizerType};
use std::rc::Rc;

pub struct ApplyRuleTask<T: OptimizerType> {
    plan: GroupPlanRef<T>,
    rule: RuleRef<T>,
    required_prop: Rc<PhysicalProperties<T>>,
}

impl<T: OptimizerType> From<ApplyRuleTask<T>> for Task<T> {
    #[inline]
    fn from(task: ApplyRuleTask<T>) -> Self {
        Task::ApplyRule(task)
    }
}

impl<T: OptimizerType> ApplyRuleTask<T> {
    pub const fn new(plan: GroupPlanRef<T>, rule: RuleRef<T>, required_prop: Rc<PhysicalProperties<T>>) -> Self {
        ApplyRuleTask {
            plan,
            rule,
            required_prop,
        }
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
                task_runner.push_task(OptimizePlanTask::new(group_plan, self.required_prop.clone()));
            } else {
                let new_task = EnforceAndCostTask::new(group_plan, self.required_prop.clone());
                task_runner.push_task(new_task);
            }
        }
    }
}
