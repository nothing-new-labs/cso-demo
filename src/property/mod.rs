pub mod sort_property;

use crate::memo::{GroupPlan, GroupRef};
use crate::property::sort_property::SortProperty;

pub trait Property {}
pub trait LogicalProperty: Property {}
pub trait PhysicalProperty: Property {
    fn make_enforcer(&self, inputs: GroupRef) -> GroupPlan;
}

#[derive(Clone)]
pub struct LogicalProperties {}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PhysicalProperties {
    sort_property: SortProperty,
}

impl PhysicalProperties {
    pub fn new() -> PhysicalProperties {
        PhysicalProperties {
            sort_property: SortProperty::new(),
        }
    }

    pub fn with_sort_property(sort_property: SortProperty) -> PhysicalProperties{
        PhysicalProperties {
            sort_property,
        }
    }

    pub fn satisfy(&self, required_prop: &PhysicalProperties) -> bool {
        // all output properties should be super set of required one
        self.sort_property.satisfy(&required_prop.sort_property)
    }

    pub fn make_enforcer(&self, group: GroupRef) -> GroupPlan {
        self.sort_property.make_enforcer(group)
    }
}
