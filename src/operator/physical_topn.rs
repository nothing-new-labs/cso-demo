use crate::expression::ColumnVar;
use crate::operator::PhysicalOperator;
use crate::property::PhysicalProperties;
use std::rc::Rc;
use crate::property::sort_property::SortProperty;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Ordering {
    pub key: ColumnVar,
    pub ascending: bool,
    pub nulls_first: bool,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct OrderSpec {
    pub order_desc: Vec<Ordering>,
}

pub struct PhysicalTopN {
    order_spec: OrderSpec,
    limit: u64,
    offset: u64,
}

impl PhysicalTopN {
    pub fn new(order_spec: OrderSpec, limit: u64, offset: u64) -> Self {
        PhysicalTopN {
            order_spec,
            limit,
            offset,
        }
    }

    pub fn order_spec(&self) -> &OrderSpec {
        &self.order_spec
    }

    pub fn limit(&self) -> u64 {
        self.limit
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }
}

impl PhysicalOperator for PhysicalTopN {
    fn name(&self) -> &str {
        "physical topN"
    }

    fn operator_id(&self) -> i16 {
        7
    }

    fn derive_output_properties(&self, _: &[Rc<PhysicalProperties>]) -> PhysicalProperties {
        PhysicalProperties::with_sort_property(SortProperty::with_order(self.order_spec.clone()))
    }

    fn get_required_properties(&self) -> Vec<Vec<PhysicalProperties>> {
        vec![vec![PhysicalProperties::with_sort_property(SortProperty::with_order(self.order_spec.clone()))]]
    }
}
