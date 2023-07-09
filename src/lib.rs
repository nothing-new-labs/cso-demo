//! A cascade style optimizer

#![forbid(unsafe_code)]

pub mod rule;

mod memo;
mod statistics;
mod task;

use crate::memo::{GroupPlanRef, Memo};
use crate::rule::RuleSet;
use crate::statistics::Statistics;
use crate::task::{OptimizeGroupTask, Task, TaskRunner};
use std::rc::Rc;

pub trait LogicalOperator {
    fn name(&self) -> &str;
    fn operator_id(&self) -> i16;
    fn derive_statistics(&self, _input_stats: &[Rc<Statistics>]) -> Statistics;
}

pub trait PhysicalOperator {
    fn name(&self) -> &str;
    fn operator_id(&self) -> i16;
}

#[derive(Clone)]
pub enum Operator {
    Logical(Rc<dyn LogicalOperator>),
    Physical(Rc<dyn PhysicalOperator>),
}

impl Operator {
    #[inline]
    pub fn is_logical(&self) -> bool {
        match self {
            Operator::Logical(_) => true,
            Operator::Physical(_) => false,
        }
    }

    #[inline]
    pub fn is_physical(&self) -> bool {
        match self {
            Operator::Logical(_) => false,
            Operator::Physical(_) => true,
        }
    }

    #[inline]
    pub fn derive_statistics(&self, input_stats: &[Rc<Statistics>]) -> Statistics {
        match self {
            Operator::Logical(op) => op.derive_statistics(input_stats),
            Operator::Physical(_) => unreachable!("only logical operators can derive statistics"),
        }
    }
}

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

    pub fn optimize(&mut self, plan: LogicalPlan, _required_properties: PhysicalProperties) -> PhysicalPlan {
        let mut optimizer_ctx = OptimizerContext::new();
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
}

impl OptimizerContext {
    fn new() -> Self {
        OptimizerContext {
            memo: Memo::new(),
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
}
