use crate::expression::ColumnVar;
use crate::operator::{OperatorId, PhysicalOperator};
use crate::property::sort_property::SortProperty;
use crate::property::PhysicalProperties;
use crate::Demo;
use cso_core::cost::Cost;
use cso_core::metadata::Stats;
use std::rc::Rc;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Ordering {
    pub key: ColumnVar,
    pub ascending: bool,
    pub nulls_first: bool,
}

impl Ordering {
    pub fn new(id: u32) -> Self {
        Self {
            key: ColumnVar::new(id),
            ascending: true,
            nulls_first: true,
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct OrderSpec {
    pub order_desc: Vec<Ordering>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhysicalSort {
    order_spec: OrderSpec,
}

impl PhysicalSort {
    pub fn new(order_spec: OrderSpec) -> Self {
        PhysicalSort { order_spec }
    }

    pub fn order_spec(&self) -> &OrderSpec {
        &self.order_spec
    }
}

impl cso_core::operator::PhysicalOperator<Demo> for PhysicalSort {
    fn name(&self) -> &str {
        "physical sort"
    }

    fn operator_id(&self) -> &OperatorId {
        &OperatorId::PhysicalSort
    }

    fn derive_output_properties(&self, _: &[Rc<PhysicalProperties>]) -> Rc<PhysicalProperties> {
        PhysicalProperties::with_property(Box::new(SortProperty::with_order(self.order_spec.clone())))
    }

    fn required_properties(&self, _input_prop: Rc<PhysicalProperties>) -> Vec<Vec<Rc<PhysicalProperties>>> {
        vec![vec![PhysicalProperties::with_property(Box::new(
            SortProperty::with_order(self.order_spec.clone()),
        ))]]
    }

    fn compute_cost(&self, _stats: Option<&dyn Stats>) -> Cost {
        Cost::new(2.0)
    }

    fn equal(&self, other: &PhysicalOperator) -> bool {
        match other.downcast_ref::<PhysicalSort>() {
            Some(other) => self.eq(other),
            None => false,
        }
    }
}
