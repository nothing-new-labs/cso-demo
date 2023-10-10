use crate::any::AsAny;
use dyn_clonable::clonable;
use std::fmt::Debug;

#[clonable]
pub trait ScalarExpression: AsAny + Debug + Clone {
    fn is_boolean_expression(&self) -> bool {
        false
    }
    fn equal(&self, other: &dyn ScalarExpression) -> bool;
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
