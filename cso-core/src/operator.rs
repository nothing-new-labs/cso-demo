use crate::any::AsAny;
use crate::cost::Cost;
use crate::metadata::MdAccessor;
use crate::metadata::Stats;
use crate::property::PhysicalProperties;
use crate::OptimizerType;
use std::fmt::Debug;
use std::rc::Rc;

pub trait LogicalOperator<T: OptimizerType>: AsAny + Debug {
    fn name(&self) -> &str;
    fn operator_id(&self) -> &T::OperatorId;
    fn derive_statistics(&self, _md_accessor: &MdAccessor<T>, input_stats: &[Rc<dyn Stats>]) -> Rc<dyn Stats>;
}

impl<O: OptimizerType> dyn LogicalOperator<O> {
    #[inline]
    pub fn downcast_ref<T: LogicalOperator<O>>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

pub trait PhysicalOperator: AsAny + Debug {
    type OptimizerType: OptimizerType;

    fn name(&self) -> &str;
    fn operator_id(&self) -> &<Self::OptimizerType as OptimizerType>::OperatorId;
    fn clone(&self) -> Box<dyn PhysicalOperator<OptimizerType = Self::OptimizerType>>;
    fn derive_output_properties(
        &self,
        child_props: &[Rc<PhysicalProperties<Self::OptimizerType>>],
    ) -> Rc<PhysicalProperties<Self::OptimizerType>>;
    fn required_properties(
        &self,
        input_prop: Rc<PhysicalProperties<Self::OptimizerType>>,
    ) -> Vec<Vec<Rc<PhysicalProperties<Self::OptimizerType>>>>;
    fn compute_cost(&self, _stats: Option<&dyn Stats>) -> Cost {
        Cost::new()
    }
    fn equal(&self, other: &dyn PhysicalOperator<OptimizerType = Self::OptimizerType>) -> bool;
}

impl<OT: OptimizerType> dyn PhysicalOperator<OptimizerType = OT> {
    #[inline]
    pub fn downcast_ref<T: PhysicalOperator>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

impl<T: OptimizerType> Clone for Box<dyn PhysicalOperator<OptimizerType = T>> {
    fn clone(&self) -> Self {
        PhysicalOperator::clone(self.as_ref())
    }
}

#[derive(Clone, Debug)]
pub enum Operator<T: OptimizerType> {
    Logical(Rc<dyn LogicalOperator<T>>),
    Physical(Rc<dyn PhysicalOperator<OptimizerType = T>>),
}

impl<T: OptimizerType> Operator<T> {
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
    pub fn logical_op(&self) -> &Rc<dyn LogicalOperator<T>> {
        match self {
            Operator::Logical(op) => op,
            Operator::Physical(_) => unreachable!("expect logical operator"),
        }
    }

    #[inline]
    pub fn physical_op(&self) -> &Rc<dyn PhysicalOperator<OptimizerType = T>> {
        match self {
            Operator::Logical(_) => unreachable!("expect physical operator"),
            Operator::Physical(op) => op,
        }
    }
}
