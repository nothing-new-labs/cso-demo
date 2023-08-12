pub trait ScalarExpression {
    fn is_boolean_expression(&self) -> bool {
        false
    }
}

pub trait AggregateExpression {}

#[derive(Eq, PartialEq)]
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
