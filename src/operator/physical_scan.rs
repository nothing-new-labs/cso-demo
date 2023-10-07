use crate::expression::ColumnVar;
use crate::operator::logical_scan::TableDesc;
use crate::operator::PhysicalOperator;
use crate::property::PhysicalProperties;
use std::rc::Rc;

#[derive(Clone, Debug)]
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

    fn derive_output_properties(&self, _: &[Rc<PhysicalProperties>]) -> Rc<PhysicalProperties> {
        Rc::new(PhysicalProperties::new())
    }

    fn required_properties(&self, _input_prop: Rc<PhysicalProperties>) -> Vec<Vec<Rc<PhysicalProperties>>> {
        vec![vec![]]
    }

    fn equal(&self, other: &dyn PhysicalOperator) -> bool {
        let other = other.downcast_ref::<PhysicalScan>().unwrap();
        self._table_desc.eq(&other._table_desc) && self._output_columns.eq(&other._output_columns)
    }
}
