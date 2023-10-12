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
use crate::rule::RuleSet;
use crate::task::{OptimizeGroupTask, TaskRunner};
use std::rc::Rc;

pub struct LogicalPlan {
    op: Rc<dyn LogicalOperator>,
    inputs: Vec<LogicalPlan>,
    required_properties: Vec<PhysicalProperties>,
}

impl LogicalPlan {
    #[inline]
    pub const fn new(
        op: Rc<dyn LogicalOperator>,
        inputs: Vec<LogicalPlan>,
        required_properties: Vec<PhysicalProperties>,
    ) -> Self {
        Self {
            op,
            inputs,
            required_properties,
        }
    }

    pub fn required_properties(&self) -> &[PhysicalProperties] {
        &self.required_properties
    }
}

#[derive(Debug)]
pub struct PhysicalPlan {
    op: Rc<dyn PhysicalOperator>,
    inputs: Vec<PhysicalPlan>,
}

impl PhysicalPlan {
    pub const fn new(op: Rc<dyn PhysicalOperator>, inputs: Vec<PhysicalPlan>) -> Self {
        PhysicalPlan { op, inputs }
    }

    pub fn operator(&self) -> &Rc<dyn PhysicalOperator> {
        &self.op
    }

    pub fn inputs(&self) -> &[PhysicalPlan] {
        &self.inputs
    }
}

impl PartialEq<Self> for PhysicalPlan {
    fn eq(&self, other: &Self) -> bool {
        self.op.equal(other.op.as_ref()) && self.inputs.eq(other.inputs())
    }
}

#[derive(Clone)]
pub struct Plan {
    op: Operator,
    inputs: Vec<Plan>,
    _property: LogicalProperties,
    group_plan: Option<GroupPlanRef>,
    _required_properties: Vec<PhysicalProperties>,
}

impl Plan {
    pub fn new(op: Operator, inputs: Vec<Plan>, group_plan: Option<GroupPlanRef>) -> Self {
        Plan {
            op,
            inputs,
            _property: LogicalProperties {},
            group_plan,
            _required_properties: vec![],
        }
    }

    pub fn inputs(&self) -> &[Plan] {
        &self.inputs
    }

    pub fn group_plan(&self) -> Option<&GroupPlanRef> {
        self.group_plan.as_ref()
    }

    pub fn operator(&self) -> &Operator {
        &self.op
    }
}

#[derive(Default)]
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
        required_properties: Rc<PhysicalProperties>,
        md_accessor: MdAccessor,
        rule_set: RuleSet,
    ) -> PhysicalPlan {
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

pub struct OptimizerContext {
    memo: Memo,
    rule_set: RuleSet,
    md_accessor: MdAccessor,
    required_properties: Rc<PhysicalProperties>,
}

impl OptimizerContext {
    fn new(md_accessor: MdAccessor, required_properties: Rc<PhysicalProperties>, rule_set: RuleSet) -> Self {
        OptimizerContext {
            memo: Memo::new(),
            md_accessor,
            rule_set,
            required_properties,
        }
    }

    pub fn memo_mut(&mut self) -> &mut Memo {
        &mut self.memo
    }

    pub fn memo(&self) -> &Memo {
        &self.memo
    }

    pub fn rule_set_mut(&mut self) -> &mut RuleSet {
        &mut self.rule_set
    }

    pub fn rule_set(&self) -> &RuleSet {
        &self.rule_set
    }

    pub fn md_accessor(&self) -> &MdAccessor {
        &self.md_accessor
    }

    pub fn required_properties(&self) -> &Rc<PhysicalProperties> {
        &self.required_properties
    }
}
