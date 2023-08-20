use crate::memo::{GroupPlan, GroupPlanRef, GroupRef, Memo};
use crate::property::PhysicalProperties;
use crate::rule::{Binding, RuleRef, RuleSet};
use crate::OptimizerContext;
use std::ops::Deref;
use std::rc::Rc;

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
    required_prop: Rc<PhysicalProperties>,
    is_explored: bool,
}

impl OptimizeGroupTask {
    pub const fn new(group: GroupRef, required_prop: Rc<PhysicalProperties>) -> Self {
        OptimizeGroupTask {
            group,
            required_prop,
            is_explored: false,
        }
    }

    fn is_explored(&self) -> bool {
        self.is_explored
    }

    fn execute(mut self, task_runner: &mut TaskRunner, _optimizer_ctx: &mut OptimizerContext) {
        let group = self.group.borrow();

        if !self.is_explored() {
            for plan in group.logical_plans().iter().rev() {
                let task = OptimizePlanTask::new(plan.clone());
                task_runner.push_task(Task::OptimizePlan(task));
            }
            self.is_explored = true;
        }

        for plan in group.physical_plans().iter().rev() {
            let task = EnforceAndCostTask::new(plan.clone(), self.required_prop.clone());
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
    plan: GroupPlanRef,
    required_prop: Rc<PhysicalProperties>,
    prev_index: usize,
}

impl Clone for EnforceAndCostTask {
    fn clone(&self) -> EnforceAndCostTask {
        todo!()
    }
}

impl EnforceAndCostTask {
    pub const fn new(new_plan: GroupPlanRef, new_required_prop: Rc<PhysicalProperties>) -> Self {
        EnforceAndCostTask {
            plan: new_plan,
            required_prop: new_required_prop,
            prev_index: 0,
        }
    }

    fn make_required_props_list(&self) -> Vec<Vec<PhysicalProperties>> {
        self.plan.borrow().operator().physical_op().get_required_properties()
    }

    fn add_enforcer_to_group(&self, required_prop: &PhysicalProperties, memo: &mut Memo) -> GroupPlanRef {
        let group = self.plan.borrow().group();
        let group_plan = required_prop.make_enforcer(group.clone());
        memo.insert_group_plan(group_plan, Some(group))
    }

    fn prev_index(&self) -> usize {
        self.prev_index
    }

    fn submit_cost_plan(&self, child_output_props: &[Rc<PhysicalProperties>], memo: &mut Memo) {
        let curr_plan = self.plan.borrow();
        let output_prop = curr_plan
            .operator()
            .physical_op()
            .derive_output_properties(child_output_props);
        let curr_cost = curr_plan.compute_cost();
        let curr_group = curr_plan.group();
        if !output_prop.satisfy(&self.required_prop) {
            let enforcer_plan = self.add_enforcer_to_group(&self.required_prop, memo);
            let enforcer_cost = curr_plan.compute_cost();
            curr_group
                .borrow_mut()
                .update_cost_plan(&self.required_prop, &enforcer_plan, enforcer_cost);
            return;
        }
        curr_group
            .borrow_mut()
            .update_cost_plan(&self.required_prop, &self.plan, curr_cost);
    }

    /**
     * 1. make require property for children base of current operator
     * 2. try to optimize child group and get best (Cost, GroupPlan) pair of every children
     * 3. once we get all output property of one candidate loop, derive output property base of current operator
     * 4. if output property does not satisfied require property, add enforcers and submit (Cost, GroupPlan) pair
     */
    fn execute(mut self, task_runner: &mut TaskRunner, optimizer_ctx: &mut OptimizerContext) {
        // 1. according to current operator create new requestPropList for children
        let reqd_props_list = self.make_required_props_list();
        for (index, required_props) in reqd_props_list.iter().enumerate().skip(self.prev_index()) {
            let mut child_output_props = Vec::new();
            for (required_prop, child_group) in required_props.iter().zip(self.plan.borrow().inputs()) {
                // 2. optimize children groups using requestPropList
                match child_group.borrow().lowest_cost_plans().get(required_prop) {
                    Some((_cost, plan)) => {
                        let output = plan.borrow().get_output_prop(required_prop).clone();
                        child_output_props.push(output);
                    }
                    None => {
                        // 3. get output property of child groups and add enforcer to cost and plan pair
                        task_runner.push_task(Task::EnforceAndCost(self.clone()));
                        let task = OptimizeGroupTask::new(child_group.clone(), Rc::new(required_prop.clone()));
                        task_runner.push_task(Task::OptimizeGroup(task));
                        return;
                    }
                }
            }
            // 4. now assume we have optimize child groups for required_props and get one best cost and plan pairs
            // and we want to compare require_prop and output_prop derived by child output props
            // if do not satisfy, add enforcer
            self.prev_index = index;
            self.submit_cost_plan(&child_output_props, optimizer_ctx.memo_mut());
        }
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
                let required_prop: PhysicalProperties = PhysicalProperties::new();
                let new_task = EnforceAndCostTask::new(group_plan, Rc::new(required_prop.clone()));
                task_runner.push_task(Task::EnforceAndCost(new_task));
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
