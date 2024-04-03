use crate::cost::{COST_INIT_SCAN_FACTOR, COST_TABLE_SCAN_COST_UNIT};
use crate::expression::ColumnVar;
use crate::operator::logical_scan::TableDesc;
use crate::operator::{OperatorId, PhysicalOperator};
use crate::property::PhysicalProperties;
use crate::Demo;
use cso_core::cost::Cost;
use cso_core::metadata::Stats;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct PhysicalScan {
    table_desc: TableDesc,
    output_columns: Vec<ColumnVar>,
}

impl PhysicalScan {
    pub fn new(table_desc: TableDesc, output_columns: Vec<ColumnVar>) -> Self {
        PhysicalScan {
            table_desc,
            output_columns,
        }
    }
}

impl cso_core::operator::PhysicalOperator<Demo> for PhysicalScan {
    fn name(&self) -> &str {
        "physical scan"
    }

    fn operator_id(&self) -> &OperatorId {
        &OperatorId::PhysicalScan
    }

    fn derive_output_properties(&self, _: &[Rc<PhysicalProperties>]) -> Rc<PhysicalProperties> {
        Rc::new(PhysicalProperties::new())
    }

    fn required_properties(&self, _input_prop: Rc<PhysicalProperties>) -> Vec<Vec<Rc<PhysicalProperties>>> {
        vec![vec![]]
    }

    fn compute_cost(&self, stats: Option<&dyn Stats>) -> Cost {
        debug_assert!(stats.is_some());

        let row_count = stats.unwrap().output_row_count() as f64;
        Cost::new(COST_INIT_SCAN_FACTOR + row_count * COST_TABLE_SCAN_COST_UNIT)
    }

    fn equal(&self, other: &PhysicalOperator) -> bool {
        match other.downcast_ref::<PhysicalScan>() {
            Some(other) => self.eq(other),
            None => false,
        }
    }
}
