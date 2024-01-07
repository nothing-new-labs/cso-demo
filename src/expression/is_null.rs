use cso_core::expression::ScalarExpression;
use cso_core::ColumnRefSet;

#[derive(Clone, Debug)]
pub struct IsNull {
    inner: Box<dyn ScalarExpression>,
}

impl IsNull {
    pub fn new(inner: Box<dyn ScalarExpression>) -> Self {
        Self { inner }
    }
}

impl ScalarExpression for IsNull {
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<IsNull>() {
            Some(other) => self.inner.eq(&other.inner),
            None => false,
        }
    }

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet) {
        self.inner.derive_used_columns(col_set);
    }

    fn split_predicates(&self) -> Vec<Box<dyn ScalarExpression>> {
        let mut expressions = Vec::new();
        expressions.push(self.inner.clone());
        expressions
    }
}

#[derive(Clone, Debug)]
pub struct IsNotNull {
    inner: Box<dyn ScalarExpression>,
}

impl IsNotNull {
    pub fn new(inner: Box<dyn ScalarExpression>) -> Self {
        Self { inner }
    }
}

impl ScalarExpression for IsNotNull {
    fn is_boolean_expression(&self) -> bool {
        true
    }

    fn equal(&self, other: &dyn ScalarExpression) -> bool {
        match other.downcast_ref::<IsNotNull>() {
            Some(other) => self.inner.eq(&other.inner),
            None => false,
        }
    }

    fn derive_used_columns(&self, col_set: &mut ColumnRefSet) {
        self.inner.derive_used_columns(col_set);
    }

    fn split_predicates(&self) -> Vec<Box<dyn ScalarExpression>> {
        let mut expressions = Vec::new();
        expressions.push(self.inner.clone());
        expressions
    }
}
