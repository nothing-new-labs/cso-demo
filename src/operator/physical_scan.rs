use crate::expression::ColumnVar;
use crate::operator::logical_scan::TableDesc;
use crate::operator::PhysicalOperator;
use crate::property::PhysicalProperties;
use std::rc::Rc;

pub struct PhysicalScan {
    _table_desc: TableDesc,
    _output_columns: Vec<ColumnVar>,
}

impl PhysicalScan {
    pub fn new(table_desc: TableDesc, output_columns: Vec<ColumnVar>) -> Self {
        PhysicalScan {
            _table_desc: table_desc,
            _output_columns: output_columns,
        }
    }
}

impl PhysicalOperator for PhysicalScan {
    fn name(&self) -> &str {
        "physical scan"
    }

    fn operator_id(&self) -> i16 {
        4
    }

    fn derive_output_properties(&self, _: &[Rc<PhysicalProperties>]) -> PhysicalProperties {
        PhysicalProperties::new()
    }

    fn get_required_properties(&self) -> Vec<Vec<PhysicalProperties>> {
        vec![vec![PhysicalProperties::new()]]
    }
}
