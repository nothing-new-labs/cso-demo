use crate::operator::physical_topn::OrderSpec;
use crate::operator::Operator;
use crate::property::{PhysicalProperties, PhysicalProperty, Property};
use crate::Plan;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct SortProperty {
    order_spec: OrderSpec,
}

impl Property for SortProperty {}

impl PhysicalProperty for SortProperty {
    fn satisfy(&self, _input: PhysicalProperties) -> bool {
        todo!()
    }

    fn add_enforcer(&self, _physical_op: Operator, _inputs: Vec<Plan>) -> Plan {
        todo!()
    }
}

impl SortProperty {
    pub fn new() -> SortProperty {
        SortProperty {
            order_spec: OrderSpec { order_desc: vec![] },
        }
    }

    pub fn make_plan(self, _inputs: Vec<Plan>) -> Plan {
        todo!()
    }
}
