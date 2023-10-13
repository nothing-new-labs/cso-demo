use crate::any::AsAny;
use crate::memo::{GroupPlan, GroupRef};
use crate::OptimizerType;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub trait Property {}

pub trait LogicalProperty: Property {}

pub trait PhysicalProperty: Property + AsAny + Debug {
    type OptimizerType: OptimizerType;

    fn clone(&self) -> Box<dyn PhysicalProperty<OptimizerType = Self::OptimizerType>>;
    fn hash(&self, hasher: &mut dyn Hasher);
    fn equal(&self, other: &dyn PhysicalProperty<OptimizerType = Self::OptimizerType>) -> bool;
    fn satisfy(&self, other: &dyn PhysicalProperty<OptimizerType = Self::OptimizerType>) -> bool;
    fn make_enforcer(&self, inputs: GroupRef<Self::OptimizerType>) -> GroupPlan<Self::OptimizerType>;
}

impl<T: OptimizerType> dyn PhysicalProperty<OptimizerType = T> {
    #[inline]
    pub fn downcast_ref<P: PhysicalProperty>(&self) -> Option<&P> {
        self.as_any().downcast_ref::<P>()
    }
}

impl<T: OptimizerType> PartialEq<Self> for dyn PhysicalProperty<OptimizerType = T> {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl<T: OptimizerType> Eq for dyn PhysicalProperty<OptimizerType = T> {}

impl<T: OptimizerType> Hash for dyn PhysicalProperty<OptimizerType = T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash(state)
    }
}

impl<T: OptimizerType> Clone for Box<dyn PhysicalProperty<OptimizerType = T>> {
    fn clone(&self) -> Self {
        PhysicalProperty::clone(self.as_ref())
    }
}

#[derive(Clone)]
pub struct LogicalProperties {}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct PhysicalProperties<T: OptimizerType> {
    properties: Vec<Box<dyn PhysicalProperty<OptimizerType = T>>>,
}

impl<T: OptimizerType> PhysicalProperties<T> {
    pub const fn new() -> PhysicalProperties<T> {
        PhysicalProperties { properties: Vec::new() }
    }

    pub fn with_property(property: Box<dyn PhysicalProperty<OptimizerType = T>>) -> Rc<PhysicalProperties<T>> {
        Rc::new(PhysicalProperties {
            properties: vec![property],
        })
    }

    pub fn satisfy(&self, required_prop: &PhysicalProperties<T>) -> bool {
        // all output properties should be super set of required one

        // TODO: multiple properties

        if self.properties.is_empty() || required_prop.properties.is_empty() {
            return self.properties.is_empty() && required_prop.properties.is_empty();
        }

        self.properties[0].satisfy(required_prop.properties[0].as_ref())
    }

    // TODO: multiple properties
    pub fn make_enforcer(&self, group: GroupRef<T>) -> GroupPlan<T> {
        self.properties[0].make_enforcer(group)
    }
}
