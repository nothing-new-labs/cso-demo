//! A cascade style optimizer

#![forbid(unsafe_code)]
#![allow(clippy::new_without_default)]
#![allow(clippy::borrowed_box)]

use crate::rule::RuleId;
use cso_core::OptimizerType;

pub mod datum;
pub mod expression;
pub mod operator;
pub mod property;
pub mod rule;
pub mod statistics;

pub struct Demo;

impl OptimizerType for Demo {
    type RuleId = RuleId;
}
