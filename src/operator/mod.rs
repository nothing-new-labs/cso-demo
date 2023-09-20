pub mod logical_filter;
pub mod logical_project;
pub mod logical_scan;
pub mod physical_filter;
pub mod physical_project;
pub mod physical_scan;
pub mod physical_sort;

use crate::any::AsAny;
use crate::cost::Cost;
use crate::metadata::md_accessor::MdAccessor;
use crate::metadata::statistics::Stats;
use crate::property::PhysicalProperties;
use std::any::Any;
use std::rc::Rc;

pub trait LogicalOperator: AsAny {
    fn name(&self) -> &str;
    fn operator_id(&self) -> i16;
    fn derive_statistics(&self, _md_accessor: &MdAccessor, input_stats: &[Rc<dyn Stats>]) -> Rc<dyn Stats>;
}

impl dyn LogicalOperator {
    #[inline]
    pub fn downcast_ref<T: LogicalOperator>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

pub trait PhysicalOperator: Any {
    fn name(&self) -> &str;
    fn operator_id(&self) -> i16;
    fn derive_output_properties(&self, child_props: &[Rc<PhysicalProperties>]) -> Rc<PhysicalProperties>;
    fn required_properties(&self, input_prop: Rc<PhysicalProperties>) -> Vec<Vec<Rc<PhysicalProperties>>>;
    fn compute_cost(&self, _stats: Option<&dyn Stats>) -> Cost {
        Cost::new()
    }
}

#[derive(Clone)]
pub enum Operator {
    Logical(Rc<dyn LogicalOperator>),
    Physical(Rc<dyn PhysicalOperator>),
}

impl Operator {
    #[inline]
    pub fn is_logical(&self) -> bool {
        match self {
            Operator::Logical(_) => true,
            Operator::Physical(_) => false,
        }
    }

    #[inline]
    pub fn is_physical(&self) -> bool {
        match self {
            Operator::Logical(_) => false,
            Operator::Physical(_) => true,
        }
    }

    #[inline]
    pub fn logical_op(&self) -> &Rc<dyn LogicalOperator> {
        match self {
            Operator::Logical(op) => op,
            Operator::Physical(_) => unreachable!("expect logical operator"),
        }
    }

    #[inline]
    pub fn physical_op(&self) -> &Rc<dyn PhysicalOperator> {
        match self {
            Operator::Logical(_) => unreachable!("expect physical operator"),
            Operator::Physical(op) => op,
        }
    }
}
