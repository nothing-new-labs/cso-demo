mod logical_scan;

use crate::statistics::Statistics;
use crate::OptimizerContext;
use std::rc::Rc;

pub trait LogicalOperator {
    fn name(&self) -> &str;
    fn operator_id(&self) -> i16;
    fn derive_statistics(&self, _optimizer_ctx: &OptimizerContext, _input_stats: &[Rc<Statistics>]) -> Statistics;
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
    pub fn logical_op(&self) -> &Rc<dyn LogicalOperator> {
        match self {
            Operator::Logical(op) => op,
            Operator::Physical(_) => unreachable!("expect logical operator"),
        }
    }

    #[inline]
    pub fn physical_op(&self) -> &Rc<dyn LogicalOperator> {
        match self {
            Operator::Logical(_) => unreachable!("expect physical operator"),
            Operator::Physical(op) => op,
        }
    }
}
