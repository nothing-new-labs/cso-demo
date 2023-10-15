//! The core framework of cascade style optimizer

#![forbid(unsafe_code)]
#![allow(clippy::new_without_default)]

pub mod any;
pub mod cost;
pub mod expression;
pub mod memo;
pub mod metadata;
pub mod operator;
pub mod property;
pub mod rule;

mod task;

use crate::memo::{GroupPlanRef, Memo};
use crate::metadata::MdAccessor;
use crate::operator::{LogicalOperator, Operator, PhysicalOperator};
use crate::property::{LogicalProperties, PhysicalProperties};
use crate::rule::{RuleId, RuleSet};
use crate::task::{OptimizeGroupTask, TaskRunner};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;

pub trait OptimizerType: 'static + PartialEq + Eq + Hash + Clone {
    type RuleId: RuleId;
    type OperatorId: PartialEq + Debug;
    type MdId: PartialEq + Eq + Clone + Hash + Debug + Serialize + for<'a> Deserialize<'a>;
}

pub struct LogicalPlan<T: OptimizerType> {
    op: Rc<dyn LogicalOperator<T>>,
    inputs: Vec<LogicalPlan<T>>,
    required_properties: Vec<PhysicalProperties<T>>,
}

impl<T: OptimizerType> LogicalPlan<T> {
    #[inline]
    pub const fn new(
        op: Rc<dyn LogicalOperator<T>>,
        inputs: Vec<LogicalPlan<T>>,
        required_properties: Vec<PhysicalProperties<T>>,
    ) -> Self {
        Self {
            op,
            inputs,
            required_properties,
        }
    }

    pub fn required_properties(&self) -> &[PhysicalProperties<T>] {
        &self.required_properties
    }
}

#[derive(Debug)]
pub struct PhysicalPlan<T: OptimizerType> {
    op: Rc<dyn PhysicalOperator<OptimizerType = T>>,
    inputs: Vec<PhysicalPlan<T>>,
}

impl<T: OptimizerType> PhysicalPlan<T> {
    pub const fn new(op: Rc<dyn PhysicalOperator<OptimizerType = T>>, inputs: Vec<PhysicalPlan<T>>) -> Self {
        PhysicalPlan { op, inputs }
    }

    pub fn operator(&self) -> &Rc<dyn PhysicalOperator<OptimizerType = T>> {
        &self.op
    }

    pub fn inputs(&self) -> &[PhysicalPlan<T>] {
        &self.inputs
    }
}

impl<T: OptimizerType> PartialEq<Self> for PhysicalPlan<T> {
    fn eq(&self, other: &Self) -> bool {
        self.op.equal(other.op.as_ref()) && self.inputs.eq(other.inputs())
    }
}

#[derive(Clone)]
pub struct Plan<T: OptimizerType> {
    op: Operator<T>,
    inputs: Vec<Plan<T>>,
    _property: LogicalProperties,
    group_plan: Option<GroupPlanRef<T>>,
    _required_properties: Vec<PhysicalProperties<T>>,
}

impl<T: OptimizerType> Plan<T> {
    pub fn new(op: Operator<T>, inputs: Vec<Plan<T>>, group_plan: Option<GroupPlanRef<T>>) -> Self {
        Plan {
            op,
            inputs,
            _property: LogicalProperties {},
            group_plan,
            _required_properties: vec![],
        }
    }

    pub fn inputs(&self) -> &[Plan<T>] {
        &self.inputs
    }

    pub fn group_plan(&self) -> Option<&GroupPlanRef<T>> {
        self.group_plan.as_ref()
    }

    pub fn operator(&self) -> &Operator<T> {
        &self.op
    }
}

#[derive(Default)]
pub struct Options {}

pub struct Optimizer<T: OptimizerType> {
    _options: Options,
    _mark: PhantomData<T>,
}

impl<T: OptimizerType> Optimizer<T> {
    pub fn new(_options: Options) -> Optimizer<T> {
        Optimizer {
            _options,
            _mark: PhantomData,
        }
    }

    pub fn optimize(
        &mut self,
        plan: LogicalPlan<T>,
        required_properties: Rc<PhysicalProperties<T>>,
        md_accessor: MdAccessor<T>,
        rule_set: RuleSet<T>,
    ) -> PhysicalPlan<T> {
        let mut optimizer_ctx = OptimizerContext::new(md_accessor, required_properties.clone(), rule_set);
        optimizer_ctx.memo_mut().init(plan);
        let mut task_runner = TaskRunner::new();
        let initial_task =
            OptimizeGroupTask::new(optimizer_ctx.memo().root_group().clone(), required_properties.clone());
        task_runner.push_task(initial_task);
        task_runner.run(&mut optimizer_ctx);
        optimizer_ctx.memo().extract_best_plan(&required_properties)
    }
}

pub struct OptimizerContext<T: OptimizerType> {
    memo: Memo<T>,
    rule_set: RuleSet<T>,
    md_accessor: MdAccessor<T>,
    required_properties: Rc<PhysicalProperties<T>>,
}

impl<T: OptimizerType> OptimizerContext<T> {
    fn new(md_accessor: MdAccessor<T>, required_properties: Rc<PhysicalProperties<T>>, rule_set: RuleSet<T>) -> Self {
        OptimizerContext {
            memo: Memo::new(),
            md_accessor,
            rule_set,
            required_properties,
        }
    }

    pub fn memo_mut(&mut self) -> &mut Memo<T> {
        &mut self.memo
    }

    pub fn memo(&self) -> &Memo<T> {
        &self.memo
    }

    pub fn rule_set_mut(&mut self) -> &mut RuleSet<T> {
        &mut self.rule_set
    }

    pub fn rule_set(&self) -> &RuleSet<T> {
        &self.rule_set
    }

    pub fn md_accessor(&self) -> &MdAccessor<T> {
        &self.md_accessor
    }

    pub fn required_properties(&self) -> &Rc<PhysicalProperties<T>> {
        &self.required_properties
    }
}
