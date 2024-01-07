use crate::any::AsAny;
use crate::ColumnRefSet;
use dyn_clonable::clonable;
use std::fmt::Debug;

#[clonable]
pub trait ScalarExpression: AsAny + Debug + Clone {
    fn is_boolean_expression(&self) -> bool {
        false
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool;

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet);

    fn split_predicates(&self) -> Vec<Box<dyn ScalarExpression>>;
}

impl dyn ScalarExpression {
    #[inline]
    pub fn downcast_ref<T: ScalarExpression>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

impl PartialEq<Self> for dyn ScalarExpression {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for dyn ScalarExpression {}

pub trait AggregateExpression {}
