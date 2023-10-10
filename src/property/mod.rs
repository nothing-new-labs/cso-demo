pub mod sort_property;

use crate::memo::{GroupPlan, GroupRef};
use cso_core::any::AsAny;
use dyn_clonable::clonable;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub trait Property {}

pub trait LogicalProperty: Property {}

#[clonable]
pub trait PhysicalProperty: Property + AsAny + Clone + Debug {
    fn hash(&self, hasher: &mut dyn Hasher);
    fn equal(&self, other: &dyn PhysicalProperty) -> bool;
    fn satisfy(&self, other: &dyn PhysicalProperty) -> bool;
    fn make_enforcer(&self, inputs: GroupRef) -> GroupPlan;
}

impl dyn PhysicalProperty {
    #[inline]
    pub fn downcast_ref<T: PhysicalProperty>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

impl PartialEq<Self> for dyn PhysicalProperty {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for dyn PhysicalProperty {}

impl Hash for dyn PhysicalProperty {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state)
    }
}

#[derive(Clone)]
pub struct LogicalProperties {}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct PhysicalProperties {
    properties: Vec<Box<dyn PhysicalProperty>>,
}

impl PhysicalProperties {
    pub const fn new() -> PhysicalProperties {
        PhysicalProperties { properties: Vec::new() }
    }

    pub fn with_property(property: Box<dyn PhysicalProperty>) -> Rc<PhysicalProperties> {
        Rc::new(PhysicalProperties {
            properties: vec![property],
        })
    }

    pub fn satisfy(&self, required_prop: &PhysicalProperties) -> bool {
        // all output properties should be super set of required one

        // TODO: multiple properties

        if self.properties.is_empty() || required_prop.properties.is_empty() {
            return self.properties.is_empty() && required_prop.properties.is_empty();
        }

        self.properties[0].satisfy(required_prop.properties[0].as_ref())
    }

    // TODO: multiple properties
    pub fn make_enforcer(&self, group: GroupRef) -> GroupPlan {
        self.properties[0].make_enforcer(group)
    }
}
