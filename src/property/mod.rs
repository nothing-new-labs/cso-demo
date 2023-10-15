use crate::Demo;

pub mod sort_property;

pub type PhysicalProperties = cso_core::property::PhysicalProperties<Demo>;
pub type PhysicalProperty = dyn cso_core::property::PhysicalProperty<Demo>;
