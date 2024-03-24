use crate::any::AsAny;
use crate::memo::{GroupPlan, GroupRef};
use crate::OptimizerType;
use dyn_clonable::clonable;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub trait Property {}

pub trait LogicalProperty: Property {}

#[clonable]
pub trait PhysicalProperty<T: OptimizerType>: Property + AsAny + Debug + Clone {
    fn hash(&self, hasher: &mut dyn Hasher);
    fn equal(&self, other: &dyn PhysicalProperty<T>) -> bool;
    fn satisfy(&self, other: &dyn PhysicalProperty<T>) -> bool;
    fn make_enforcer(&self, inputs: GroupRef<T>) -> GroupPlan<T>;
}

impl<T: OptimizerType> dyn PhysicalProperty<T> {
    #[inline]
    pub fn downcast_ref<P: PhysicalProperty<T>>(&self) -> Option<&P> {
        self.as_any().downcast_ref::<P>()
    }
}

impl<T: OptimizerType> PartialEq<Self> for dyn PhysicalProperty<T> {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl<T: OptimizerType> Eq for dyn PhysicalProperty<T> {}

impl<T: OptimizerType> Hash for dyn PhysicalProperty<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state)
    }
}

#[derive(Clone)]
pub struct LogicalProperties {}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct PhysicalProperties<T: OptimizerType> {
    properties: Vec<Box<dyn PhysicalProperty<T>>>,
}

impl<T: OptimizerType> PhysicalProperties<T> {
    pub const fn new() -> PhysicalProperties<T> {
        PhysicalProperties { properties: Vec::new() }
    }

    pub fn with_property(property: Box<dyn PhysicalProperty<T>>) -> Rc<PhysicalProperties<T>> {
        Rc::new(PhysicalProperties {
            properties: vec![property],
        })
    }

    pub fn satisfy(&self, required_prop: &PhysicalProperties<T>) -> bool {
        // all output properties should be super set of required one

        // TODO: multiple properties
        match (self.properties.is_empty(), required_prop.properties.is_empty()) {
            (_, true) => true,
            (true, false) => false,
            (false, false) => self.properties[0].satisfy(required_prop.properties[0].as_ref())
        }

    }

    // TODO: multiple properties
    pub fn make_enforcer(&self, group: GroupRef<T>) -> GroupPlan<T> {
        self.properties[0].make_enforcer(group)
    }
}
