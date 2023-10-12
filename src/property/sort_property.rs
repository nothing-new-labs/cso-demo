use crate::operator::physical_sort::{OrderSpec, PhysicalSort};
use cso_core::memo::{GroupPlan, GroupRef};
use cso_core::operator::Operator;
use cso_core::property::{PhysicalProperty, Property};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct SortProperty {
    order_spec: OrderSpec,
}

impl Property for SortProperty {}

impl PhysicalProperty for SortProperty {
    fn hash(&self, mut hasher: &mut dyn Hasher) {
        Hash::hash(self, &mut hasher)
    }

    fn equal(&self, other: &dyn PhysicalProperty) -> bool {
        match other.downcast_ref::<SortProperty>() {
            Some(property) => self.eq(property),
            None => false,
        }
    }

    fn satisfy(&self, other: &dyn PhysicalProperty) -> bool {
        match other.downcast_ref::<SortProperty>() {
            Some(property) => self.satisfy(property),
            None => false,
        }
    }

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
