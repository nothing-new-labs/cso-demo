pub mod sort_property;

use crate::operator::Operator;
use crate::property::sort_property::SortProperty;
use crate::Plan;

pub trait Property {}
pub trait LogicalProperty: Property {}
pub trait PhysicalProperty: Property {
    fn satisfy(&self, _input: PhysicalProperties) -> bool
    where
        Self: Sized,
    {
        true
    }
    fn add_enforcer(&self, physical_op: Operator, inputs: Vec<Plan>) -> Plan
    where
        Self: Sized,
    {
        Plan::new(physical_op, inputs)
    }
}

#[derive(Clone)]
pub struct LogicalProperties {}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PhysicalProperties {
    _order_spec: SortProperty,
}

impl PhysicalProperties {
    pub fn new() -> PhysicalProperties {
        PhysicalProperties {
            _order_spec: SortProperty::new(),
        }
    }

    pub fn satisfy(&self, _required_prop: &PhysicalProperties) -> bool {
        todo!()
    }
}
