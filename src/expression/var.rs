use cso_core::expression::ScalarExpression;

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
        match other.downcast_ref::<ColumnVar>() {
            Some(other) => self.id() == other.id(),
            None => false,
        }
    }
}
