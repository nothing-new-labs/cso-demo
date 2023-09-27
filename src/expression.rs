use std::fmt::Debug;

pub trait ScalarExpression: Debug {
    fn is_boolean_expression(&self) -> bool {
        false
    }
}

pub trait AggregateExpression {}

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

impl ScalarExpression for ColumnVar {}

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
}
