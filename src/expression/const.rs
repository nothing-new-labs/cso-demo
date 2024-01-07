use cso_core::expression::ScalarExpression;
use cso_core::ColumnRefSet;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum Const {
    Int32(i32),
    Int64(i64),
    Str(String),
}

impl ScalarExpression for Const {
    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<Const>() {
            Some(other) => self == other,
            None => false,
        }
    }

    fn derive_used_columns(&self, _col_set: &mut ColumnRefSet) {
        // no column
    }

    fn split_predicates(&self) -> Vec<Box<dyn ScalarExpression>> {
        Vec::new()
    }
}
