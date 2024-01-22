use crate::expression::ColumnVar;
use crate::operator::logical_index_scan::IndexDesc;
use crate::operator::logical_scan::TableDesc;
use crate::operator::physical_sort::{OrderSpec, Ordering};
use crate::operator::{OperatorId, PhysicalOperator};
use crate::property::sort_property::SortProperty;
use crate::property::PhysicalProperties;
use crate::Demo;
use cso_core::cost::Cost;
use cso_core::expression::ScalarExpression;
use cso_core::metadata::Stats;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct PhysicalIndexScan {
    index_desc: IndexDesc,
    table_desc: TableDesc,
    output_columns: Vec<ColumnVar>,
    predicate: Rc<dyn ScalarExpression>,
}

impl PhysicalIndexScan {
    pub fn new(
        index_desc: IndexDesc,
        table_desc: TableDesc,
        output_columns: Vec<ColumnVar>,
        predicate: Rc<dyn ScalarExpression>,
    ) -> Self {
        PhysicalIndexScan {
            index_desc,
            table_desc,
            output_columns,
            predicate,
        }
    }
}

impl cso_core::operator::PhysicalOperator<Demo> for PhysicalIndexScan {
    fn name(&self) -> &str {
        "physical index scan"
    }

    fn operator_id(&self) -> &OperatorId {
        &OperatorId::PhysicalIndexScan
    }

    fn derive_output_properties(&self, child_props: &[Rc<PhysicalProperties>]) -> Rc<PhysicalProperties> {
        debug_assert!(child_props.is_empty());
        let key_columns = self.index_desc.key_columns();

        let mut order_desc = vec![];
        for key in key_columns {
            order_desc.push(Ordering::new(*key as u32));
        }

        let sort_prop = SortProperty::with_order(OrderSpec { order_desc });
        PhysicalProperties::with_property(Box::new(sort_prop))
    }

    fn required_properties(&self, _input_prop: Rc<PhysicalProperties>) -> Vec<Vec<Rc<PhysicalProperties>>> {
        vec![vec![]]
    }

    fn compute_cost(&self, _stats: Option<&dyn Stats>) -> Cost {
        Cost::new(1.0)
    }

    fn equal(&self, other: &PhysicalOperator) -> bool {
        match other.downcast_ref::<PhysicalIndexScan>() {
            Some(other) => self.eq(other),
            None => false,
        }
    }
}

impl PartialEq for PhysicalIndexScan {
    fn eq(&self, other: &Self) -> bool {
        self.index_desc == other.index_desc
            && self.table_desc == other.table_desc
            && self.output_columns == other.output_columns
            && self.predicate.equal(other.predicate.as_ref())
    }
}
