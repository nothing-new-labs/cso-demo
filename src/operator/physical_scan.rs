use crate::expression::ColumnVar;
use crate::operator::logical_scan::TableDesc;
use crate::operator::{OperatorId, PhysicalOperator};
use crate::property::PhysicalProperties;
use crate::Demo;
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

impl cso_core::operator::PhysicalOperator for PhysicalScan {
    type OptimizerType = Demo;

    fn name(&self) -> &str {
        "physical scan"
    }

    fn operator_id(&self) -> &OperatorId {
        &OperatorId::PhysicalScan
    }

    fn clone(&self) -> Box<PhysicalOperator> {
        Box::new(Clone::clone(self))
    }

    fn derive_output_properties(&self, _: &[Rc<PhysicalProperties>]) -> Rc<PhysicalProperties> {
        Rc::new(PhysicalProperties::new())
    }

    fn required_properties(&self, _input_prop: Rc<PhysicalProperties>) -> Vec<Vec<Rc<PhysicalProperties>>> {
        vec![vec![]]
    }

    fn equal(&self, other: &PhysicalOperator) -> bool {
        match other.downcast_ref::<PhysicalScan>() {
            Some(other) => self.eq(other),
            None => false,
        }
    }
}
