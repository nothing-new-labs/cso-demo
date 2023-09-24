use crate::memo::{GroupPlan, GroupRef};
use crate::operator::physical_sort::{OrderSpec, PhysicalSort};
use crate::operator::Operator;
use crate::property::{PhysicalProperty, Property};
use std::rc::Rc;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct SortProperty {
    order_spec: OrderSpec,
}

impl Property for SortProperty {}

impl PhysicalProperty for SortProperty {
    fn make_enforcer(&self, group: GroupRef) -> GroupPlan {
        let physical_sort = PhysicalSort::new(self.order_spec.clone());
        GroupPlan::new(Operator::Physical(Rc::new(physical_sort)), vec![group])
    }
}

impl SortProperty {
    pub fn new() -> SortProperty {
        SortProperty {
            order_spec: OrderSpec { order_desc: vec![] },
        }
    }

    pub fn with_order(order_spec: OrderSpec) -> SortProperty {
        SortProperty { order_spec }
    }

    pub fn satisfy(&self, required: &SortProperty) -> bool {
        if self.order_spec.order_desc.len() < required.order_spec.order_desc.len() {
            return false;
        }
        for sort in self
            .order_spec
            .order_desc
            .iter()
            .zip(required.order_spec.order_desc.iter())
        {
            if sort.0.eq(sort.1) {
                continue;
            } else {
                return false;
            }
        }
        true
    }
}
