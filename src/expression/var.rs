use cso_core::expression::ScalarExpression;
use cso_core::ColumnRefSet;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
#[repr(transparent)]
pub struct ColumnVar {
    id: u32, // global column id
}

impl ColumnVar {
    pub fn new(id: u32) -> Self {
        ColumnVar { id }
    }

    pub fn id(&self) -> u32 {
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

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet) {
        col_set.insert(self.id);
    }

    fn split_predicates(&self) -> Vec<Box<dyn ScalarExpression>> {
        Vec::new()
    }
}
