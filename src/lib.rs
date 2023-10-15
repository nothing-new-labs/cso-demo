//! A cascade style optimizer

#![forbid(unsafe_code)]
#![allow(clippy::new_without_default)]
#![allow(clippy::borrowed_box)]

use crate::operator::OperatorId;
use crate::rule::RuleId;
use cso_core::OptimizerType;

pub mod datum;
pub mod expression;
pub mod operator;
pub mod property;
pub mod rule;
pub mod statistics;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Demo;

impl OptimizerType for Demo {
    type RuleId = RuleId;
    type OperatorId = OperatorId;
    type MdId = u64;
}

pub use cso_core::Options;

pub mod metadata {
    use crate::Demo;

    pub type MdAccessor = cso_core::metadata::MdAccessor<Demo>;
    pub type MdCache = cso_core::metadata::MdCache<Demo>;
    pub type MdProvider = dyn cso_core::metadata::MdProvider<Demo>;
    pub type CachedMdProvider = cso_core::metadata::CachedMdProvider<Demo>;
    pub use cso_core::metadata::Metadata;
    pub use cso_core::metadata::Stats;
}

pub(crate) type GroupPlan = cso_core::memo::GroupPlan<Demo>;
// pub(crate) type GroupPlanRef = cso_core::memo::GroupPlanRef<Demo>;
// pub(crate) type Group = cso_core::memo::Group<Demo>;
pub(crate) type GroupRef = cso_core::memo::GroupRef<Demo>;
pub(crate) type Pattern = cso_core::rule::Pattern<Demo>;
pub(crate) type PatternType = cso_core::rule::PatternType<Demo>;

pub type Plan = cso_core::Plan<Demo>;
pub type OptimizerContext = cso_core::OptimizerContext<Demo>;
pub type LogicalPlan = cso_core::LogicalPlan<Demo>;
pub type PhysicalPlan = cso_core::PhysicalPlan<Demo>;
pub type Optimizer = cso_core::Optimizer<Demo>;
