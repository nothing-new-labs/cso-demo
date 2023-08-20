use crate::memo::{GroupPlan, GroupRef};
use crate::operator::physical_topn::{OrderSpec, PhysicalTopN};
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
        let physical_topn = PhysicalTopN::new(self.order_spec.clone(), 1, 0);
        GroupPlan::new(Operator::Physical(Rc::new(physical_topn)), vec![group])
    }
}

impl SortProperty {
    pub fn new() -> SortProperty {
        SortProperty {
            order_spec: OrderSpec { order_desc: vec![] },
        }
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
