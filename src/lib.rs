//! A cascade style optimizer

#![forbid(unsafe_code)]

use crate::memo::Memo;
use crate::statistics::Statistics;
use crate::task::{OptimizeGroupTask, Task, TaskRunner};
use std::rc::Rc;

pub mod rule;

mod memo;
mod statistics;
mod task;

pub trait LogicalOperator {
    fn operator_id(&self) -> i16;
    fn derive_statistics(&self, _input_stats: &[Rc<Statistics>]) -> Statistics;
}

pub trait PhysicalOperator {
    fn operator_id(&self) -> i16;
}

pub enum Operator {
    Logical(Box<dyn LogicalOperator>),
    Physical(Box<dyn PhysicalOperator>),
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
    op: Box<dyn LogicalOperator>,
    inputs: Vec<LogicalPlan>,
    _required_properties: Vec<PhysicalProperties>,
}

pub struct PhysicalPlan {
    _op: Box<dyn PhysicalOperator>,
    _inputs: Vec<PhysicalPlan>,
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
    ) -> PhysicalPlan {
        let mut optimizer_ctx = OptimizerContext::new();
        optimizer_ctx.memo_mut().init(plan);
        let mut task_runner = TaskRunner::new();
        let initial_task = OptimizeGroupTask::new(optimizer_ctx.memo().root_group().clone());
        task_runner.push_task(Task::OptimizeGroup(initial_task));
        task_runner.run(&mut optimizer_ctx);
        todo!()

        Rc::
    }
}

pub struct OptimizerContext {
    memo: Memo,
}

impl OptimizerContext {
    fn new() -> Self {
        OptimizerContext { memo: Memo::new() }
    }

    pub fn memo_mut(&mut self) -> &mut Memo {
        &mut self.memo
    }

    pub fn memo(&self) -> &Memo {
        &self.memo
    }
}
