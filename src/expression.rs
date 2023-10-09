use cso_core::expression::ScalarExpression;
use std::fmt::Debug;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
#[repr(transparent)]
pub struct ColumnVar {
    id: i32,
}

impl ColumnVar {
    pub fn new(id: i32) -> Self {
        ColumnVar { id }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

impl ScalarExpression for ColumnVar {
    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        let other = other.downcast_ref::<ColumnVar>().unwrap();
        self.id() == other.id()
    }
}

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct IsNull {
    inner: ColumnVar,
}

impl IsNull {
    pub fn new(inner: ColumnVar) -> Self {
        Self { inner }
    }
}

impl ScalarExpression for IsNull {
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        let other = other.downcast_ref::<IsNull>().unwrap();
        self.inner == other.inner
    }
}
