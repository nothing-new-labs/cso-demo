use crate::expression::ColumnVar;
use crate::operator::PhysicalOperator;
use crate::property::sort_property::SortProperty;
use crate::property::PhysicalProperties;
use std::rc::Rc;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Ordering {
    pub key: ColumnVar,
    pub ascending: bool,
    pub nulls_first: bool,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct OrderSpec {
    pub order_desc: Vec<Ordering>,
}

#[derive(Debug)]
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

impl PhysicalOperator for PhysicalSort {
    fn name(&self) -> &str {
        "physical sort"
    }

    fn operator_id(&self) -> i16 {
        7
    }

    fn derive_output_properties(&self, _: &[Rc<PhysicalProperties>]) -> Rc<PhysicalProperties> {
        PhysicalProperties::with_sort_property(SortProperty::with_order(self.order_spec.clone()))
    }

    fn required_properties(&self, _input_prop: Rc<PhysicalProperties>) -> Vec<Vec<Rc<PhysicalProperties>>> {
        vec![vec![PhysicalProperties::with_sort_property(SortProperty::with_order(
            self.order_spec.clone(),
        ))]]
    }
}
