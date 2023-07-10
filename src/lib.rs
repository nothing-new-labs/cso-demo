//! A cascade style optimizer

#![forbid(unsafe_code)]

pub mod rule;

mod datum;
mod memo;
mod metadata;
mod operators;
mod statistics;
mod task;

use crate::memo::{GroupPlanRef, Memo};
use crate::metadata::MdAccessor;
use crate::operators::{LogicalOperator, Operator, PhysicalOperator};
use crate::rule::RuleSet;
use crate::task::{OptimizeGroupTask, Task, TaskRunner};
use std::rc::Rc;

pub trait ScalarExpression {}
pub trait AggregateExpression {}

pub struct LogicalPlan {
    op: Rc<dyn LogicalOperator>,
    inputs: Vec<LogicalPlan>,
    _required_properties: Vec<PhysicalProperties>,
}

pub struct PhysicalPlan {
    _op: Rc<dyn PhysicalOperator>,
    _inputs: Vec<PhysicalPlan>,
}

pub struct Plan {
    op: Operator,
    inputs: Vec<Plan>,
    _property: LogicalProperties,
    group_plan: Option<GroupPlanRef>,
    _required_properties: Vec<PhysicalProperties>,
}

impl Plan {
    pub fn new(op: Operator, inputs: Vec<Plan>) -> Self {
        Plan {
            op,
            inputs,
            _property: LogicalProperties {},
            group_plan: None,
            _required_properties: vec![],
        }
    }

    pub fn inputs(&self) -> &[Plan] {
        &self.inputs
    }

    pub fn group_plan(&self) -> Option<&GroupPlanRef> {
        self.group_plan.as_ref()
    }
}

pub trait Property {}
pub trait LogicalProperty: Property {}
pub trait PhysicalProperty: Property {}

pub struct LogicalProperties {}
pub struct PhysicalProperties {}

pub struct Options {}

pub struct Optimizer {
    _options: Options,
}

impl Optimizer {
    pub fn new(_options: Options) -> Optimizer {
        Optimizer { _options }
    }

    pub fn optimize(
        &mut self,
        plan: LogicalPlan,
        _required_properties: PhysicalProperties,
        md_accessor: MdAccessor,
    ) -> PhysicalPlan {
        let mut optimizer_ctx = OptimizerContext::new(md_accessor);
        optimizer_ctx.memo_mut().init(plan);
        let mut task_runner = TaskRunner::new();
        let initial_task = OptimizeGroupTask::new(optimizer_ctx.memo().root_group().clone());
        task_runner.push_task(Task::OptimizeGroup(initial_task));
        task_runner.run(&mut optimizer_ctx);
        todo!()
    }
}

pub struct OptimizerContext {
    memo: Memo,
    rule_set: RuleSet,
    md_accessor: MdAccessor,
}

impl OptimizerContext {
    fn new(md_accessor: MdAccessor) -> Self {
        OptimizerContext {
            memo: Memo::new(),
            md_accessor,
            rule_set: RuleSet::new(),
        }
    }

    pub fn memo_mut(&mut self) -> &mut Memo {
        &mut self.memo
    }

    pub fn memo(&self) -> &Memo {
        &self.memo
    }

    pub fn rule_set(&self) -> &RuleSet {
        &self.rule_set
    }

    pub fn md_accessor(&self) -> &MdAccessor {
        &self.md_accessor
    }
}
